use crate::analytics::optimization::{
    filter_constraints, portfolio_optimization, Constraints, ObjectiveFunction,
};
use crate::analytics::statistics::{
    applicable_periods, compute_periodic_stats, compute_periodic_stats_per_asset,
    daily_portfolio_returns, mean_portfolio_return, parse_naive_date, portfolio_std_dev,
    resample_returns_pct, resample_series_pct, resample_values_last, shrink_covariance, xirr,
    DatedCashFlow, PerformancePeriod, PerformanceStats, PortfolioReturnsResult, RebalanceConfig,
    RebalanceEvent, ReturnsFrequency, ShrinkageMethod, TransactionEvent,
};
use crate::data::ticker::TickerData;
use crate::models::portfolio::{expand_scheduled_cash_flows, ScheduledCashFlow, Transaction};
use crate::prelude::{Column, IntervalDays, Ticker, Tickers};
use crate::utils::date_utils::interval_days;
use chrono::{DateTime, NaiveDateTime};
use polars::prelude::*;
use std::collections::HashMap;
use std::error::Error;

/// Compute percentage returns from a price vector.
/// Returns a Vec of length `prices.len() - 1`.
fn prices_to_returns(prices: &[Option<f64>]) -> Vec<f64> {
    prices
        .windows(2)
        .map(|w| match (w[0], w[1]) {
            (Some(prev), Some(curr)) if prev.abs() > 1e-12 => (curr - prev) / prev,
            _ => 0.0,
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct TickerPerformanceStats {
    pub ticker_symbol: String,
    pub benchmark_symbol: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub dates_array: Vec<String>,
    pub interval: IntervalDays,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub security_prices: Series,
    pub security_returns: Series,
    pub benchmark_returns: Option<Series>,
    pub performance_stats: PerformanceStats,
    /// Resampled percentage returns keyed by frequency.
    /// Each entry is `(group_labels, resampled_values)`.
    pub returns_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, Vec<f64>)>,
    /// Performance stats computed for each applicable trailing period.
    pub periodic_stats: Vec<(PerformancePeriod, PerformanceStats)>,
}

pub trait TickerPerformance {
    fn performance_stats(
        &self,
    ) -> impl std::future::Future<Output = Result<TickerPerformanceStats, Box<dyn Error>>>;
}

impl TickerPerformance for Ticker {
    /// Computes the performance statistics for the ticker
    ///
    /// # Returns
    ///
    /// * `TickerPerformanceStats` struct
    async fn performance_stats(&self) -> Result<TickerPerformanceStats, Box<dyn Error>> {
        // Fetch security prices
        let security_chart = self.get_chart().await?;
        let security_prices_df = DataFrame::new(vec![
            security_chart.column("timestamp")?.clone(),
            security_chart
                .column(Column::AdjClose.as_str())?
                .clone()
                .with_name(self.ticker.as_str().into()),
        ])?;
        let security_prices = security_chart
            .column(Column::AdjClose.as_str())?
            .as_materialized_series();

        // If benchmark is present, align prices then compute returns
        let (aligned_security_returns, aligned_benchmark_returns, dates_array, interval) =
            if let Some(ref bench_ticker) = self.benchmark_ticker {
                let bench_chart = bench_ticker.get_chart().await?;
                let bench_prices_df = DataFrame::new(vec![
                    bench_chart.column("timestamp")?.clone(),
                    bench_chart
                        .column(Column::AdjClose.as_str())?
                        .clone()
                        .with_name("__benchmark__".into()),
                ])?;

                // Full outer join on timestamp, then forward/backward-fill prices
                let joined = security_prices_df.join(
                    &bench_prices_df,
                    ["timestamp"],
                    ["timestamp"],
                    JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
                    None,
                )?;
                let joined = joined.sort(
                    ["timestamp"],
                    SortMultipleOptions::new().with_order_descending(false),
                )?;
                let joined = joined.fill_null(FillNullStrategy::Forward(None))?;
                let joined = joined.fill_null(FillNullStrategy::Backward(None))?;

                let dates = joined
                    .column("timestamp")?
                    .datetime()?
                    .into_no_null_iter()
                    .map(|x| {
                        DateTime::from_timestamp_millis(x)
                            .expect("valid millis timestamp")
                            .naive_local()
                    })
                    .collect::<Vec<NaiveDateTime>>();
                let ivl = interval_days(dates.clone());
                let dates_str: Vec<String> = dates.iter().map(|x| x.to_string()).collect();

                // Compute returns from aligned prices
                let sec_prices_col = joined.column(&self.ticker)?.f64()?.to_vec();
                let bench_prices_col = joined.column("__benchmark__")?.f64()?.to_vec();

                let sec_returns = prices_to_returns(&sec_prices_col);
                let bench_returns = prices_to_returns(&bench_prices_col);

                let sec_series = Series::new(self.ticker.as_str().into(), &sec_returns);
                let bench_series = Series::new("benchmark_returns".into(), &bench_returns);

                // Trim the first date (no return for it)
                let dates_str = dates_str[1..].to_vec();

                (sec_series, Some(bench_series), dates_str, ivl)
            } else {
                // No benchmark: compute returns from security prices only
                let sorted = security_prices_df.sort(
                    ["timestamp"],
                    SortMultipleOptions::new().with_order_descending(false),
                )?;
                let dates = sorted
                    .column("timestamp")?
                    .datetime()?
                    .into_no_null_iter()
                    .map(|x| {
                        DateTime::from_timestamp_millis(x)
                            .expect("valid millis timestamp")
                            .naive_local()
                    })
                    .collect::<Vec<NaiveDateTime>>();
                let ivl = interval_days(dates.clone());
                let dates_str: Vec<String> = dates.iter().map(|x| x.to_string()).collect();

                let sec_prices_col = sorted.column(&self.ticker)?.f64()?.to_vec();
                let sec_returns = prices_to_returns(&sec_prices_col);
                let sec_series = Series::new(self.ticker.as_str().into(), &sec_returns);

                let dates_str = dates_str[1..].to_vec();

                (sec_series, None, dates_str, ivl)
            };

        let performance_stats = PerformanceStats::compute_stats(
            aligned_security_returns.clone(),
            aligned_benchmark_returns.clone(),
            self.risk_free_rate,
            self.confidence_level,
            interval,
        )?;

        // Compute resampled returns for all available frequencies
        let native_freq = ReturnsFrequency::from_interval(&interval);
        let available_freqs = native_freq.available_frequencies();
        let mut returns_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, Vec<f64>)> =
            HashMap::new();
        for freq in &available_freqs {
            let (labels, values) =
                resample_series_pct(&dates_array, &aligned_security_returns, *freq)?;
            returns_by_frequency.insert(*freq, (labels, values));
        }

        // Compute periodic performance stats
        let periodic_stats = compute_periodic_stats(
            &dates_array,
            &aligned_security_returns,
            aligned_benchmark_returns.as_ref(),
            self.risk_free_rate,
            self.confidence_level,
            interval,
        )?;

        Ok(TickerPerformanceStats {
            ticker_symbol: self.ticker.clone(),
            benchmark_symbol: self.benchmark_symbol.clone(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            dates_array,
            interval,
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            security_prices: security_prices.clone(),
            security_returns: aligned_security_returns.clone(),
            benchmark_returns: aligned_benchmark_returns,
            performance_stats,
            returns_by_frequency,
            periodic_stats,
        })
    }
}

// ---------------------------------------------------------------------------
// Shared portfolio data that is prepared once and reused by both optimization
// and performance analysis.
// ---------------------------------------------------------------------------

/// Pre-computed data shared between optimization and performance analysis.
#[derive(Debug, Clone)]
pub struct PortfolioData {
    pub ticker_symbols: Vec<String>,
    pub benchmark_symbol: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub interval: IntervalDays,
    pub dates_array: Vec<String>,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    /// Asset returns DataFrame (no timestamp column).
    pub portfolio_returns: DataFrame,
    /// Benchmark return series (None when no benchmark is configured).
    pub benchmark_returns: Option<Series>,
}

impl PortfolioData {
    /// The native (base) frequency of the data.
    pub fn native_frequency(&self) -> ReturnsFrequency {
        ReturnsFrequency::from_interval(&self.interval)
    }

    /// All frequencies available for resampling (native + coarser).
    pub fn available_frequencies(&self) -> Vec<ReturnsFrequency> {
        self.native_frequency().available_frequencies()
    }

    /// Which trailing-window periods are applicable for this data range.
    /// Uses the actual first/last trading dates from the data rather than the
    /// user-requested start/end dates, so that the set of periods is consistent
    /// with what `compute_periodic_stats_per_asset` actually computes.
    pub fn applicable_periods(&self) -> Vec<PerformancePeriod> {
        let actual_start = self
            .dates_array
            .first()
            .map(|s| s.as_str())
            .unwrap_or(&self.start_date);
        let actual_end = self
            .dates_array
            .last()
            .map(|s| s.as_str())
            .unwrap_or(&self.end_date);
        applicable_periods(actual_start, actual_end)
    }
}

/// Fetches and prepares all shared data needed by both optimization and
/// performance analysis.
pub async fn prepare_portfolio_data(
    tickers: &Tickers,
    benchmark_ticker: Option<&Ticker>,
) -> Result<PortfolioData, Box<dyn Error>> {
    let ticker_symbols: Vec<String> = tickers.tickers.iter().map(|x| x.ticker.clone()).collect();
    let benchmark_symbol = benchmark_ticker.map(|t| t.ticker.clone());

    // --- Step 1: Fetch all asset prices ---
    let mut price_frames: Vec<DataFrame> = Vec::new();
    for t in &tickers.tickers {
        let chart = t.get_chart().await?;
        let df = DataFrame::new(vec![
            chart.column("timestamp")?.clone(),
            chart
                .column(Column::AdjClose.as_str())?
                .clone()
                .with_name(t.ticker.as_str().into()),
        ])?;
        price_frames.push(df);
    }

    // Optionally fetch benchmark prices
    let bench_prices_df = if let Some(bt) = benchmark_ticker {
        let chart = bt.get_chart().await?;
        let df = DataFrame::new(vec![
            chart.column("timestamp")?.clone(),
            chart
                .column(Column::AdjClose.as_str())?
                .clone()
                .with_name("__benchmark__".into()),
        ])?;
        Some(df)
    } else {
        None
    };

    // --- Step 2: Full outer join all price frames on timestamp (keep all dates) ---
    let mut joined = price_frames
        .into_iter()
        .reduce(|acc, df| {
            acc.join(
                &df,
                ["timestamp"],
                ["timestamp"],
                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
                None,
            )
            .expect("Failed to join asset price frames")
        })
        .ok_or("No ticker price data available")?;

    // If benchmark present, full outer join it too
    if let Some(bench_df) = bench_prices_df {
        joined = joined.join(
            &bench_df,
            ["timestamp"],
            ["timestamp"],
            JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
            None,
        )?;
    }

    // Sort chronologically, then forward-fill and backward-fill prices
    joined = joined.sort(
        ["timestamp"],
        SortMultipleOptions::new().with_order_descending(false),
    )?;
    joined = joined.fill_null(FillNullStrategy::Forward(None))?;
    joined = joined.fill_null(FillNullStrategy::Backward(None))?;

    // --- Step 3: Extract dates ---
    let dates_ndt = joined
        .column("timestamp")?
        .datetime()?
        .into_no_null_iter()
        .map(|x| {
            DateTime::from_timestamp_millis(x)
                .expect("valid millis timestamp")
                .naive_local()
        })
        .collect::<Vec<NaiveDateTime>>();
    let interval = interval_days(dates_ndt.clone());
    let dates_array: Vec<String> = dates_ndt.iter().map(|x| x.to_string()).collect();

    // --- Step 4: Compute returns from aligned prices ---
    let mut returns_cols: Vec<polars::prelude::Column> = Vec::new();
    for sym in &ticker_symbols {
        let prices_col = joined.column(sym)?.f64()?.to_vec();
        let rets = prices_to_returns(&prices_col);
        returns_cols.push(Series::new(sym.as_str().into(), &rets).into());
    }
    let portfolio_returns = DataFrame::new(returns_cols)?;

    let benchmark_returns = if benchmark_symbol.is_some() {
        let bench_prices_col = joined.column("__benchmark__")?.f64()?.to_vec();
        let rets = prices_to_returns(&bench_prices_col);
        Some(Series::new("benchmark_returns".into(), &rets))
    } else {
        None
    };

    // Trim the first date (no return for it)
    let dates_array = dates_array[1..].to_vec();

    Ok(PortfolioData {
        ticker_symbols,
        benchmark_symbol,
        start_date: tickers.start_date.clone(),
        end_date: tickers.end_date.clone(),
        interval,
        dates_array,
        confidence_level: tickers.confidence_level,
        risk_free_rate: tickers.risk_free_rate,
        portfolio_returns,
        benchmark_returns,
    })
}

// ---------------------------------------------------------------------------
// Portfolio Optimization Result
// ---------------------------------------------------------------------------

/// Holds the results of a portfolio optimization run.
#[derive(Debug, Clone)]
pub struct PortfolioOptimizationResult {
    pub objective_function: ObjectiveFunction,
    pub optimization_method: String,
    pub constraints: Constraints,
    pub optimal_weights: Vec<f64>,
    pub optimal_return: f64,
    pub optimal_risk: f64,
    pub category_weights: Vec<(String, String, f64)>,
    /// Mean-variance efficient frontier `[return, risk]` pairs.
    /// Populated only for frontier-type objectives (MaxSharpe, MaxSortino); empty otherwise.
    pub efficient_frontier: Vec<Vec<f64>>,
    /// Component risk contribution of each asset under the optimal weights.
    /// `risk_contributions[i]` is asset i's contribution to total portfolio volatility.
    pub risk_contributions: Vec<f64>,
}

/// Runs portfolio optimization on the prepared data.
pub fn optimize_portfolio(
    data: &PortfolioData,
    objective_function: ObjectiveFunction,
    constraints: Option<Constraints>,
) -> Result<PortfolioOptimizationResult, Box<dyn Error>> {
    let fetched_symbols: Vec<String> = data
        .portfolio_returns
        .get_column_names()
        .iter()
        .map(|x| x.to_string())
        .collect();

    let constraints = filter_constraints(constraints, data.ticker_symbols.clone(), fetched_symbols);

    let mean_returns: Vec<f64> = data
        .portfolio_returns
        .get_columns()
        .iter()
        .map(|col| {
            col.f64()
                .unwrap_or_else(|_| panic!("Column '{}' is not Float64", col.name()))
                .mean()
                .unwrap_or(0.0)
        })
        .collect();
    let shrunk = shrink_covariance(&data.portfolio_returns, ShrinkageMethod::LedoitWolf)?;
    let cov_matrix = shrunk.matrix;

    // Convert annual risk-free rate to per-period rate so it is in the same
    // units as mean_returns (which are per-period decimal returns).
    let annual_days = 365.0 / data.interval.average;
    let per_period_rfr = (1.0 + data.risk_free_rate).powf(1.0 / annual_days) - 1.0;

    let opt_result = portfolio_optimization(
        &mean_returns,
        &cov_matrix,
        &data.portfolio_returns,
        per_period_rfr,
        data.confidence_level,
        objective_function,
        constraints.clone(),
    );

    let optimal_return = mean_portfolio_return(&opt_result.optimal_weights, &mean_returns);
    let optimal_risk = portfolio_std_dev(&opt_result.optimal_weights, &cov_matrix);

    Ok(PortfolioOptimizationResult {
        objective_function,
        optimization_method: opt_result.optimization_method,
        constraints,
        optimal_weights: opt_result.optimal_weights,
        optimal_return,
        optimal_risk,
        category_weights: opt_result.category_weights,
        efficient_frontier: opt_result.efficient_frontier,
        risk_contributions: opt_result.risk_contributions,
    })
}

// ---------------------------------------------------------------------------
// Portfolio Performance Stats
// ---------------------------------------------------------------------------

/// Holds the performance analysis results for a portfolio.
#[derive(Debug, Clone)]
pub struct PortfolioPerformanceStats {
    pub weights: Vec<f64>,
    pub starting_weights: Vec<f64>,
    pub ending_weights: Vec<f64>,
    pub starting_values: Vec<f64>,
    pub ending_values: Vec<f64>,
    /// Total portfolio value at the end of each period.
    pub portfolio_values: Vec<f64>,
    /// Per-asset values at the end of each period: `[num_rows][num_assets]`.
    pub asset_values_over_time: Vec<Vec<f64>>,
    pub portfolio_returns: Series,
    pub performance_stats: PerformanceStats,

    // -- Frequency-aware resampled data --
    /// Resampled percentage returns (asset DataFrame) keyed by frequency.
    /// Each entry is `(group_labels, resampled_df)`.
    pub returns_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, DataFrame)>,
    /// Resampled portfolio aggregate returns keyed by frequency.
    /// Each entry is `(group_labels, resampled_values)`.
    pub portfolio_returns_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, Vec<f64>)>,
    /// Resampled dollar values keyed by frequency.
    /// Each entry is `(group_labels, sampled_asset_values, sampled_portfolio_values)`.
    pub values_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, Vec<Vec<f64>>, Vec<f64>)>,

    // -- Periodic performance stats --
    /// Performance stats for each applicable trailing period (Full, 6M, 1Y, 3Y, 5Y, 10Y).
    pub periodic_stats: Vec<(PerformancePeriod, PerformanceStats)>,
    /// Per-asset performance stats for each applicable trailing period.
    /// Key = period, Value = one `PerformanceStats` per asset (same order as `weights`).
    pub periodic_stats_per_asset: HashMap<PerformancePeriod, Vec<PerformanceStats>>,

    // -- Rebalancing diagnostics --
    /// Rebalance events that occurred during the simulation (empty if no
    /// rebalancing strategy was configured).
    pub rebalance_events: Vec<RebalanceEvent>,

    // -- Transaction history --
    /// Enriched transaction events (rebalances + cash flows) with per-asset
    /// detail, cumulative TWR, and cumulative MWR.  Empty when no rebalancing
    /// or cash flows are configured.
    pub transaction_events: Vec<TransactionEvent>,

    /// Overall money-weighted return (annualised %) for the full simulation
    /// period.  `None` when there are no external cash flows or when the XIRR
    /// solver fails to converge.
    pub money_weighted_return: Option<f64>,
}

/// Maps a list of `Transaction` entries to a dense per-asset cash flow matrix indexed by row.
///
/// Returns `Vec<Vec<f64>>` of shape `[num_rows][num_assets]`, where each inner vector holds
/// the dollar cash flow for each asset on that row. Transactions whose dates or tickers
/// do not match any row/asset are silently ignored.
fn map_transactions_to_rows(
    transactions: &Option<Vec<Transaction>>,
    dates_array: &[String],
    fetched_symbols: &[String],
    num_rows: usize,
    num_assets: usize,
) -> Vec<Vec<f64>> {
    let mut asset_cash_flows = vec![vec![0.0; num_assets]; num_rows];

    if let Some(txns) = transactions {
        // Build a lookup from ticker symbol to asset index
        let ticker_to_index: std::collections::HashMap<&str, usize> = fetched_symbols
            .iter()
            .enumerate()
            .map(|(i, s)| (s.as_str(), i))
            .collect();

        // Build a lookup from date string to row index
        let date_to_index: std::collections::HashMap<&str, usize> = dates_array
            .iter()
            .enumerate()
            .map(|(i, d)| (d.as_str(), i))
            .collect();

        for txn in txns {
            // Resolve asset index
            let asset_idx = match ticker_to_index.get(txn.ticker.as_str()) {
                Some(&idx) => idx,
                None => continue,
            };

            // Resolve row index: try exact match first, then date-only prefix
            let row_idx = if let Some(&idx) = date_to_index.get(txn.date.as_str()) {
                idx
            } else {
                let mut found = None;
                for (i, d) in dates_array.iter().enumerate() {
                    if d.starts_with(&txn.date) {
                        found = Some(i);
                        break;
                    }
                }
                match found {
                    Some(idx) => idx,
                    None => continue,
                }
            };

            asset_cash_flows[row_idx][asset_idx] += txn.amount;
        }
    }

    asset_cash_flows
}

/// Computes portfolio performance statistics from the prepared data and a
/// set of weights (plus optional per-asset transactions, rebalancing, and
/// scheduled cash flows).
///
/// # Arguments
///
/// * `data` - Pre-computed shared portfolio data
/// * `weights` - Fractional portfolio weights (sum to ~1.0)
/// * `transactions` - Optional per-asset transactions
/// * `initial_values` - Optional per-asset dollar amounts (e.g. from the builder's `weights`).
///   When provided, value tracking uses actual dollar amounts. When `None`, value tracking
///   uses the fractional weights (so portfolio value starts at ~1.0).
/// * `rebalance_config` - Optional rebalancing configuration (target weights + strategy)
/// * `scheduled_cash_flows` - Optional scheduled cash flow definitions to expand
pub fn compute_performance(
    data: &PortfolioData,
    weights: &[f64],
    transactions: Option<Vec<Transaction>>,
    initial_values: Option<Vec<f64>>,
    rebalance_config: Option<&RebalanceConfig>,
    scheduled_cash_flows: Option<&[ScheduledCashFlow]>,
) -> Result<PortfolioPerformanceStats, Box<dyn Error>> {
    let fetched_symbols: Vec<String> = data
        .portfolio_returns
        .get_column_names()
        .iter()
        .map(|x| x.to_string())
        .collect();

    let num_rows = data.portfolio_returns.height();
    let num_assets = fetched_symbols.len();

    // Expand scheduled cash flows if provided
    let target_w = rebalance_config
        .map(|c| c.target_weights.clone())
        .unwrap_or_else(|| weights.to_vec());

    let (extra_transactions, rebalance_cash_flows) = if let Some(schedules) = scheduled_cash_flows {
        let expanded =
            expand_scheduled_cash_flows(schedules, &data.dates_array, &fetched_symbols, &target_w);
        (expanded.transactions, expanded.rebalance_cash_flows)
    } else {
        (Vec::new(), vec![0.0; num_rows])
    };

    // Merge user-supplied transactions with auto-generated ones
    let mut all_transactions = transactions.unwrap_or_default();
    all_transactions.extend(extra_transactions);

    let asset_cash_flows = map_transactions_to_rows(
        &Some(all_transactions),
        &data.dates_array,
        &fetched_symbols,
        num_rows,
        num_assets,
    );

    // Determine starting values for value tracking.
    // If initial_values are provided (from the builder's `weights`), use them directly so that
    // dollar-denominated tracking and transactions are dimensionally consistent.
    // Otherwise, fall back to fractional weights (portfolio starts at ~1.0).
    let starting_values = initial_values.unwrap_or_else(|| weights.to_vec());

    let PortfolioReturnsResult {
        portfolio_returns: portfolio_returns_series,
        portfolio_values,
        asset_values: asset_values_over_time,
        ending_values,
        rebalance_events,
        transaction_events,
    } = daily_portfolio_returns(
        &starting_values,
        &data.portfolio_returns,
        &asset_cash_flows,
        rebalance_config,
        &data.dates_array,
        &rebalance_cash_flows,
    )?;

    let performance_stats = PerformanceStats::compute_stats(
        portfolio_returns_series.clone(),
        data.benchmark_returns.clone(),
        data.risk_free_rate,
        data.confidence_level,
        data.interval,
    )?;

    // Compute starting and ending weights from values
    let starting_total: f64 = starting_values.iter().sum();
    let starting_weights: Vec<f64> = if starting_total > 0.0 {
        starting_values.iter().map(|v| v / starting_total).collect()
    } else {
        weights.to_vec()
    };

    let ending_total: f64 = ending_values.iter().sum();
    let ending_weights: Vec<f64> = if ending_total > 0.0 {
        ending_values.iter().map(|v| v / ending_total).collect()
    } else {
        weights.to_vec()
    };

    // -- Compute frequency-aware resampled data --
    let available_freqs = data.available_frequencies();
    let dates = &data.dates_array;

    let mut returns_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, DataFrame)> =
        HashMap::new();
    let mut portfolio_returns_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, Vec<f64>)> =
        HashMap::new();
    let mut values_by_frequency: HashMap<ReturnsFrequency, (Vec<String>, Vec<Vec<f64>>, Vec<f64>)> =
        HashMap::new();

    for freq in &available_freqs {
        // Resample asset percentage returns
        let (labels, resampled_df) = resample_returns_pct(dates, &data.portfolio_returns, *freq)?;
        returns_by_frequency.insert(*freq, (labels.clone(), resampled_df));

        // Resample portfolio aggregate returns
        let (port_labels, port_vals) =
            resample_series_pct(dates, &portfolio_returns_series, *freq)?;
        portfolio_returns_by_frequency.insert(*freq, (port_labels, port_vals));

        // Resample dollar values
        let num = dates.len();
        let av = &asset_values_over_time[..num.min(asset_values_over_time.len())];
        let pv = &portfolio_values[..num.min(portfolio_values.len())];
        let (val_labels, sampled_assets, sampled_portfolio) =
            resample_values_last(dates, av, pv, *freq);
        values_by_frequency.insert(*freq, (val_labels, sampled_assets, sampled_portfolio));
    }

    // -- Compute periodic performance stats --
    let periodic_stats = compute_periodic_stats(
        dates,
        &portfolio_returns_series,
        data.benchmark_returns.as_ref(),
        data.risk_free_rate,
        data.confidence_level,
        data.interval,
    )?;

    let periodic_stats_per_asset = compute_periodic_stats_per_asset(
        dates,
        &data.portfolio_returns,
        data.benchmark_returns.as_ref(),
        data.risk_free_rate,
        data.confidence_level,
        data.interval,
    )?;

    // -- Compute overall money-weighted return (XIRR) --
    let money_weighted_return = {
        let start_d = parse_naive_date(&data.dates_array[0]);
        let end_d = data.dates_array.last().and_then(|s| parse_naive_date(s));

        match (start_d, end_d) {
            (Some(sd), Some(ed)) if ed > sd => {
                let initial_total: f64 = starting_values.iter().sum();
                let mut cf_list: Vec<DatedCashFlow> = Vec::new();

                // Initial investment (investor outflow)
                cf_list.push(DatedCashFlow {
                    date: sd,
                    amount: -initial_total,
                });

                // All external cash flows during the simulation
                for (row_idx, row_flows) in asset_cash_flows.iter().enumerate() {
                    let per_asset_total: f64 = row_flows.iter().sum();
                    let rebal_flow = if row_idx < rebalance_cash_flows.len() {
                        rebalance_cash_flows[row_idx]
                    } else {
                        0.0
                    };
                    let total_cf = per_asset_total + rebal_flow;
                    if total_cf.abs() > 1e-12 {
                        if let Some(d) = data
                            .dates_array
                            .get(row_idx)
                            .and_then(|s| parse_naive_date(s))
                        {
                            cf_list.push(DatedCashFlow {
                                date: d,
                                amount: -total_cf, // addition → investor outflow (neg)
                            });
                        }
                    }
                }

                // Terminal value (investor inflow)
                let terminal_value: f64 = portfolio_values.last().copied().unwrap_or(initial_total);
                cf_list.push(DatedCashFlow {
                    date: ed,
                    amount: terminal_value,
                });

                xirr(&cf_list)
            }
            _ => None,
        }
    };

    Ok(PortfolioPerformanceStats {
        weights: weights.to_vec(),
        starting_weights,
        ending_weights,
        starting_values: starting_values.clone(),
        ending_values,
        portfolio_values,
        asset_values_over_time,
        portfolio_returns: portfolio_returns_series,
        performance_stats,
        returns_by_frequency,
        portfolio_returns_by_frequency,
        values_by_frequency,
        periodic_stats,
        periodic_stats_per_asset,
        rebalance_events,
        transaction_events,
        money_weighted_return,
    })
}
