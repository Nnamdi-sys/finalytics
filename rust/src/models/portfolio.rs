use crate::analytics::optimization::{Constraints, ObjectiveFunction};
use crate::analytics::performance::{
    compute_performance, optimize_portfolio, prepare_portfolio_data, PortfolioData,
    PortfolioOptimizationResult, PortfolioPerformanceStats,
};
use crate::analytics::statistics::RebalanceConfig;
use crate::prelude::{Interval, Tickers, KLINE};
use std::error::Error;

// ---------------------------------------------------------------------------
// Schedule Frequency (shared by rebalancing and cash flows)
// ---------------------------------------------------------------------------

/// Calendar frequency used for both rebalancing triggers and scheduled cash flows.
///
/// Period boundaries are detected at the first trading day of each new period:
/// - `Monthly`: first trading day of each calendar month
/// - `Quarterly`: first trading day of Jan, Apr, Jul, Oct
/// - `SemiAnnually`: first trading day of Jan, Jul
/// - `Annually`: first trading day of each calendar year
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleFrequency {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}

impl std::fmt::Display for ScheduleFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScheduleFrequency::Monthly => write!(f, "Monthly"),
            ScheduleFrequency::Quarterly => write!(f, "Quarterly"),
            ScheduleFrequency::SemiAnnually => write!(f, "Semi-Annually"),
            ScheduleFrequency::Annually => write!(f, "Annually"),
        }
    }
}

// ---------------------------------------------------------------------------
// Rebalancing
// ---------------------------------------------------------------------------

/// Strategy that determines when the portfolio is rebalanced back to target weights.
///
/// During performance simulation, at each time step the system checks whether a
/// rebalance should be triggered. When triggered, all asset values are
/// redistributed to match the target weight vector while keeping the total
/// portfolio value constant (no external cash is added or removed).
///
/// # Variants
///
/// * `Calendar(ScheduleFrequency)` — rebalance on a fixed calendar schedule
/// * `Threshold(f64)` — rebalance when **any** asset's weight drifts more than
///   `threshold` (absolute) from its target. E.g. `0.05` means 5%.
/// * `CalendarOrThreshold(ScheduleFrequency, f64)` — rebalance on the earlier of
///   the calendar trigger **or** the threshold breach.
#[derive(Debug, Clone)]
pub enum RebalanceStrategy {
    /// Rebalance on a fixed calendar schedule.
    Calendar(ScheduleFrequency),
    /// Rebalance when any asset's weight drifts more than `threshold` from target.
    Threshold(f64),
    /// Rebalance on the calendar schedule **or** when the threshold is breached,
    /// whichever comes first.
    CalendarOrThreshold(ScheduleFrequency, f64),
}

impl std::fmt::Display for RebalanceStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RebalanceStrategy::Calendar(freq) => write!(f, "Calendar ({freq})"),
            RebalanceStrategy::Threshold(t) => write!(f, "Threshold ({:.1}%)", t * 100.0),
            RebalanceStrategy::CalendarOrThreshold(freq, t) => {
                write!(f, "Calendar ({freq}) or Threshold ({:.1}%)", t * 100.0)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Scheduled Cash Flows
// ---------------------------------------------------------------------------

/// Determines how a scheduled cash flow is distributed across portfolio assets.
///
/// # Variants
///
/// * `ProRata` — distribute according to target weights (from optimization or
///   initial allocation).
/// * `Rebalance` — direct new money to the most **underweight** assets (for
///   additions) or withdraw from the most **overweight** assets (for withdrawals).
///   This achieves partial rebalancing without selling/buying overweight/underweight
///   positions explicitly.
/// * `Custom(Vec<f64>)` — distribute according to a user-supplied weight vector.
///   The vector must have the same length as the number of assets and should sum
///   to 1.0 for additions (for withdrawals the signs are handled automatically).
#[derive(Debug, Clone)]
pub enum CashFlowAllocation {
    /// Distribute according to target (optimized or initial) weights.
    ProRata,
    /// Direct cash to underweight assets (additions) or withdraw from overweight
    /// assets (withdrawals) to move closer to target weights.
    Rebalance,
    /// Distribute according to a custom weight vector.
    Custom(Vec<f64>),
}

/// A recurring cash flow schedule for the portfolio.
///
/// Generates automatic additions or withdrawals at regular intervals during the
/// performance simulation. Multiple schedules can be combined (e.g. monthly
/// contributions + quarterly withdrawals).
///
/// # Fields
///
/// * `amount` — Dollar amount per occurrence. Positive for additions, negative
///   for withdrawals.
/// * `frequency` — How often the cash flow occurs.
/// * `start_date` — Optional start date (`"YYYY-MM-DD"`). If `None`, starts from
///   the first portfolio date.
/// * `end_date` — Optional end date (`"YYYY-MM-DD"`). If `None`, continues until
///   the last portfolio date.
/// * `allocation` — How the cash flow is distributed across assets.
///
/// # Example
///
/// ```rust
/// use finalytics::prelude::*;
///
/// // $2,000/month DCA distributed by target weights
/// let dca = ScheduledCashFlow {
///     amount: 2_000.0,
///     frequency: ScheduleFrequency::Monthly,
///     start_date: None,
///     end_date: None,
///     allocation: CashFlowAllocation::ProRata,
/// };
///
/// // $5,000/quarter withdrawal from overweight assets
/// let withdrawal = ScheduledCashFlow {
///     amount: -5_000.0,
///     frequency: ScheduleFrequency::Quarterly,
///     start_date: Some("2024-06-01".to_string()),
///     end_date: None,
///     allocation: CashFlowAllocation::Rebalance,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ScheduledCashFlow {
    /// Dollar amount per occurrence: positive = addition, negative = withdrawal.
    pub amount: f64,
    /// How often this cash flow occurs.
    pub frequency: ScheduleFrequency,
    /// Optional start date. If `None`, starts from the first portfolio date.
    pub start_date: Option<String>,
    /// Optional end date. If `None`, continues until the last portfolio date.
    pub end_date: Option<String>,
    /// How the cash flow is distributed across assets.
    pub allocation: CashFlowAllocation,
}

// ---------------------------------------------------------------------------
// Transactions
// ---------------------------------------------------------------------------

/// Represents a per-asset transaction (addition or withdrawal) at a specific date.
///
/// # Fields
///
/// * `date` - Date string for the transaction (e.g. "2024-01-15" or "2024-01-15 00:00:00")
/// * `ticker` - Ticker symbol of the asset
/// * `amount` - Dollar amount: positive for additions, negative for withdrawals
#[derive(Debug, Clone)]
pub struct Transaction {
    pub date: String,
    pub ticker: String,
    pub amount: f64,
}

// ---------------------------------------------------------------------------
// Schedule expansion helpers
// ---------------------------------------------------------------------------

/// Returns `true` when `curr_date` is the first trading day of a new period
/// relative to `prev_date`, according to the given `ScheduleFrequency`.
///
/// Date strings must start with `"YYYY-MM-DD"` (may be longer, e.g. with time).
pub fn is_period_boundary(prev_date: &str, curr_date: &str, freq: ScheduleFrequency) -> bool {
    let prev_year: u32 = prev_date[..4].parse().unwrap_or(0);
    let curr_year: u32 = curr_date[..4].parse().unwrap_or(0);
    let prev_month: u32 = prev_date[5..7].parse().unwrap_or(0);
    let curr_month: u32 = curr_date[5..7].parse().unwrap_or(0);

    match freq {
        ScheduleFrequency::Monthly => curr_month != prev_month || curr_year != prev_year,
        ScheduleFrequency::Quarterly => {
            let prev_q = (prev_month.saturating_sub(1)) / 3;
            let curr_q = (curr_month.saturating_sub(1)) / 3;
            curr_q != prev_q || curr_year != prev_year
        }
        ScheduleFrequency::SemiAnnually => {
            let prev_h = (prev_month.saturating_sub(1)) / 6;
            let curr_h = (curr_month.saturating_sub(1)) / 6;
            curr_h != prev_h || curr_year != prev_year
        }
        ScheduleFrequency::Annually => curr_year != prev_year,
    }
}

/// Check whether a date string falls within the `[start, end]` window of a
/// `ScheduledCashFlow`. Both bounds are optional (open-ended).
fn date_in_range(date: &str, start: &Option<String>, end: &Option<String>) -> bool {
    let d = &date[..date.len().min(10)];
    if let Some(ref s) = start {
        let s = &s[..s.len().min(10)];
        if d < s {
            return false;
        }
    }
    if let Some(ref e) = end {
        let e = &e[..e.len().min(10)];
        if d > e {
            return false;
        }
    }
    true
}

/// Result of expanding scheduled cash flows for the simulation.
///
/// - `transactions`: per-asset `Transaction` entries for `ProRata` / `Custom`
///   allocations that can be merged with the user's explicit transactions.
/// - `rebalance_cash_flows`: per-row aggregate dollar amounts for
///   `Rebalance`-allocated cash flows. These are distributed inside the
///   simulation loop based on the current portfolio state.
pub struct ExpandedCashFlows {
    pub transactions: Vec<Transaction>,
    pub rebalance_cash_flows: Vec<f64>,
}

/// Expands one or more `ScheduledCashFlow` definitions into concrete per-row
/// data that the simulation can consume.
///
/// # Arguments
///
/// * `schedules` — the scheduled cash flow definitions
/// * `dates` — the portfolio date strings (one per simulation row)
/// * `tickers` — asset ticker symbols (same order as columns)
/// * `target_weights` — target allocation weights (same order as `tickers`)
pub fn expand_scheduled_cash_flows(
    schedules: &[ScheduledCashFlow],
    dates: &[String],
    tickers: &[String],
    target_weights: &[f64],
) -> ExpandedCashFlows {
    let n = dates.len();
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut rebalance_flows = vec![0.0_f64; n];

    for sched in schedules {
        // Walk through dates and detect period boundaries
        for row in 1..n {
            if !is_period_boundary(&dates[row - 1], &dates[row], sched.frequency) {
                continue;
            }
            if !date_in_range(&dates[row], &sched.start_date, &sched.end_date) {
                continue;
            }

            match &sched.allocation {
                CashFlowAllocation::ProRata => {
                    // Distribute according to target weights
                    for (i, ticker) in tickers.iter().enumerate() {
                        let amt = sched.amount * target_weights[i];
                        if amt.abs() > 1e-10 {
                            transactions.push(Transaction {
                                date: dates[row].clone(),
                                ticker: ticker.clone(),
                                amount: amt,
                            });
                        }
                    }
                }
                CashFlowAllocation::Custom(weights) => {
                    // Distribute according to custom weights
                    for (i, ticker) in tickers.iter().enumerate() {
                        let w = weights.get(i).copied().unwrap_or(0.0);
                        let amt = sched.amount * w;
                        if amt.abs() > 1e-10 {
                            transactions.push(Transaction {
                                date: dates[row].clone(),
                                ticker: ticker.clone(),
                                amount: amt,
                            });
                        }
                    }
                }
                CashFlowAllocation::Rebalance => {
                    // Aggregate — will be distributed inside the simulation loop
                    rebalance_flows[row] += sched.amount;
                }
            }
        }
    }

    ExpandedCashFlows {
        transactions,
        rebalance_cash_flows: rebalance_flows,
    }
}

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

pub struct PortfolioBuilder {
    pub ticker_symbols: Vec<String>,
    pub benchmark_symbol: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub tickers_data: Option<Vec<KLINE>>,
    pub benchmark_data: Option<KLINE>,
    pub objective_function: ObjectiveFunction,
    pub constraints: Option<Constraints>,
    pub weights: Option<Vec<f64>>,
    pub transactions: Option<Vec<Transaction>>,
    pub rebalance_strategy: Option<RebalanceStrategy>,
    pub scheduled_cash_flows: Option<Vec<ScheduledCashFlow>>,
}

impl Default for PortfolioBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PortfolioBuilder {
    pub fn new() -> PortfolioBuilder {
        PortfolioBuilder {
            ticker_symbols: Vec::new(),
            benchmark_symbol: None,
            start_date: String::new(),
            end_date: String::new(),
            interval: Interval::OneDay,
            confidence_level: 0.95,
            risk_free_rate: 0.0,
            tickers_data: None,
            benchmark_data: None,
            objective_function: ObjectiveFunction::MaxSharpe,
            constraints: None,
            weights: None,
            transactions: None,
            rebalance_strategy: None,
            scheduled_cash_flows: None,
        }
    }

    pub fn ticker_symbols(mut self, ticker_symbols: Vec<&str>) -> PortfolioBuilder {
        self.ticker_symbols = ticker_symbols.iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn benchmark_symbol(mut self, benchmark_symbol: &str) -> PortfolioBuilder {
        self.benchmark_symbol = Some(benchmark_symbol.to_string());
        self
    }

    pub fn start_date(mut self, start_date: &str) -> PortfolioBuilder {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(mut self, end_date: &str) -> PortfolioBuilder {
        self.end_date = end_date.to_string();
        self
    }

    pub fn interval(mut self, interval: Interval) -> PortfolioBuilder {
        self.interval = interval;
        self
    }

    pub fn confidence_level(mut self, confidence_level: f64) -> PortfolioBuilder {
        self.confidence_level = confidence_level;
        self
    }

    pub fn risk_free_rate(mut self, risk_free_rate: f64) -> PortfolioBuilder {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn tickers_data(mut self, tickers_data: Option<Vec<KLINE>>) -> PortfolioBuilder {
        self.tickers_data = tickers_data;
        self
    }

    pub fn benchmark_data(mut self, benchmark_data: Option<KLINE>) -> PortfolioBuilder {
        self.benchmark_data = benchmark_data;
        self
    }

    pub fn objective_function(mut self, objective_function: ObjectiveFunction) -> PortfolioBuilder {
        self.objective_function = objective_function;
        self
    }

    pub fn constraints(mut self, constraints: Option<Constraints>) -> PortfolioBuilder {
        self.constraints = constraints;
        self
    }

    /// Set explicit per-asset dollar amounts (weights) for the portfolio.
    ///
    /// When provided (without calling `optimize()`), the portfolio computes
    /// performance statistics directly from these dollar amounts. The fractional
    /// weights are derived as `weights[i] / sum(weights)`.
    pub fn weights(mut self, weights: Vec<f64>) -> PortfolioBuilder {
        self.weights = Some(weights);
        self
    }

    /// Set ad-hoc per-asset transactions (additions / withdrawals).
    ///
    /// These are applied during the performance simulation on the dates
    /// specified in each `Transaction`. They are independent of — and can be
    /// combined with — `scheduled_cash_flows`.
    pub fn transactions(mut self, transactions: Vec<Transaction>) -> PortfolioBuilder {
        self.transactions = Some(transactions);
        self
    }

    pub fn rebalance_strategy(mut self, strategy: Option<RebalanceStrategy>) -> PortfolioBuilder {
        self.rebalance_strategy = strategy;
        self
    }

    pub fn scheduled_cash_flows(
        mut self,
        flows: Option<Vec<ScheduledCashFlow>>,
    ) -> PortfolioBuilder {
        self.scheduled_cash_flows = flows;
        self
    }

    pub async fn build(self) -> Result<Portfolio, Box<dyn Error>> {
        let tickers = if self.tickers_data.is_some() {
            let mut builder = Tickers::builder()
                .tickers_data(self.tickers_data)
                .benchmark_data(self.benchmark_data)
                .confidence_level(self.confidence_level)
                .risk_free_rate(self.risk_free_rate);
            if let Some(ref sym) = self.benchmark_symbol {
                builder = builder.benchmark_symbol(sym);
            }
            builder.build()
        } else {
            let mut builder = Tickers::builder()
                .tickers(self.ticker_symbols.iter().map(|x| x.as_str()).collect())
                .start_date(&self.start_date)
                .end_date(&self.end_date)
                .interval(self.interval)
                .confidence_level(self.confidence_level)
                .risk_free_rate(self.risk_free_rate);
            if let Some(ref sym) = self.benchmark_symbol {
                builder = builder.benchmark_symbol(sym);
            }
            builder.build()
        };

        let data = prepare_portfolio_data(&tickers, tickers.benchmark_ticker.as_ref()).await?;

        // Build a RebalanceConfig if a strategy was provided.
        // At build time we don't have optimized weights yet, so for allocation-only
        // portfolios we derive target weights from the provided weights.
        let rebalance_config = self.rebalance_strategy.as_ref().map(|strategy| {
            let target_weights = if let Some(ref alloc) = self.weights {
                let total: f64 = alloc.iter().sum();
                if total > 0.0 {
                    alloc.iter().map(|v| v / total).collect()
                } else {
                    vec![
                        1.0 / data.portfolio_returns.width() as f64;
                        data.portfolio_returns.width()
                    ]
                }
            } else {
                // Equal weight fallback
                vec![1.0 / data.portfolio_returns.width() as f64; data.portfolio_returns.width()]
            };
            RebalanceConfig {
                target_weights,
                strategy: strategy.clone(),
            }
        });

        // Auto-compute performance stats when weights are provided
        let performance_stats = if let Some(ref alloc) = self.weights {
            let num_assets = data.portfolio_returns.width();
            let total: f64 = alloc.iter().sum();
            if total > 0.0 && alloc.len() == num_assets {
                let fractional: Vec<f64> = alloc.iter().map(|v| v / total).collect();
                let transactions = self.transactions.clone();
                let initial_values = Some(alloc.clone());
                compute_performance(
                    &data,
                    &fractional,
                    transactions,
                    initial_values,
                    rebalance_config.as_ref(),
                    self.scheduled_cash_flows.as_deref(),
                )
                .ok()
            } else {
                None
            }
        } else {
            None
        };

        Ok(Portfolio {
            tickers,
            data,
            objective_function: self.objective_function,
            constraints: self.constraints,
            weights: self.weights,
            transactions: self.transactions,
            rebalance_strategy: self.rebalance_strategy,
            scheduled_cash_flows: self.scheduled_cash_flows,
            optimization_result: None,
            performance_stats,
        })
    }
}

/// # Portfolio Struct
///
/// ### Description
///    - This is the Portfolio Analysis Module for the `Finalytics` Library.
///    - It provides methods for Portfolio Optimization and Performance Analysis.
///    - Configuration (objective function, constraints, weights,
///      transactions, rebalancing, scheduled cash flows) is set via the builder.
///    - Call `optimize()` to find optimal weights **and** compute in-sample performance
///      statistics in one step. After `optimize()`, both `optimization_result` and
///      `performance_stats` are populated.
///    - Optionally call `update_dates()` / `update_data()` to swap in new data, then
///      call `performance_stats()` to evaluate the same weights out-of-sample.
///    - Call `performance_stats()` directly (without optimization) to evaluate explicit
///      weights (dollar allocations).
///
/// ### Workflows
///
/// **Optimization (in-sample stats included automatically):**
/// ```rust,no_run
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut portfolio = Portfolio::builder()
///         .ticker_symbols(vec!["NVDA", "AAPL", "MSFT", "BTC-USD"])
///         .benchmark_symbol("^GSPC")
///         .start_date("2022-01-01")
///         .end_date("2023-01-01")
///         .interval(Interval::OneDay)
///         .confidence_level(0.95)
///         .risk_free_rate(0.02)
///         .objective_function(ObjectiveFunction::MaxSharpe)
///         .build().await?;
///
///     // Optimize — in-sample performance stats are computed automatically
///     portfolio.optimize()?;
///     portfolio.report(Some(ReportType::Optimization)).await?.show()?;
///
///     Ok(())
/// }
/// ```
///
/// **Optimization -> Out-of-sample evaluation (Yahoo data):**
/// ```rust,no_run
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut portfolio = Portfolio::builder()
///         .ticker_symbols(vec!["NVDA", "AAPL", "MSFT", "BTC-USD"])
///         .benchmark_symbol("^GSPC")
///         .start_date("2022-01-01")
///         .end_date("2023-01-01")
///         .interval(Interval::OneDay)
///         .confidence_level(0.95)
///         .risk_free_rate(0.02)
///         .objective_function(ObjectiveFunction::MaxSharpe)
///         .build().await?;
///
///     // Optimize on 2022 data (in-sample stats available immediately)
///     portfolio.optimize()?;
///
///     // Update to 2023 data for out-of-sample evaluation
///     portfolio.update_dates("2023-01-01", "2024-01-01").await?;
///     portfolio.performance_stats()?;
///     portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///     Ok(())
/// }
/// ```
///
/// **Custom KLINE data -> Optimization -> Out-of-sample evaluation:**
/// ```rust,no_run
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let train = vec![
///         KLINE::from_csv("AAPL", "aapl_train.csv")?,
///         KLINE::from_csv("MSFT", "msft_train.csv")?,
///     ];
///     let train_bench = KLINE::from_csv("^GSPC", "gspc_train.csv")?;
///
///     let mut portfolio = Portfolio::builder()
///         .tickers_data(Some(train))
///         .benchmark_data(Some(train_bench))
///         .objective_function(ObjectiveFunction::MaxSharpe)
///         .confidence_level(0.95)
///         .risk_free_rate(0.02)
///         .build().await?;
///
///     // Optimize on training data (in-sample stats available immediately)
///     portfolio.optimize()?;
///
///     // Load evaluation-period data and update
///     let eval = vec![
///         KLINE::from_csv("AAPL", "aapl_eval.csv")?,
///         KLINE::from_csv("MSFT", "msft_eval.csv")?,
///     ];
///     let eval_bench = KLINE::from_csv("^GSPC", "gspc_eval.csv")?;
///     portfolio.update_data(eval, Some(eval_bench)).await?;
///     portfolio.performance_stats()?;
///     portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///     Ok(())
/// }
/// ```
///
/// **Explicit weights -> Direct evaluation:**
/// ```rust,no_run
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let mut portfolio = Portfolio::builder()
///         .ticker_symbols(vec!["NVDA", "AAPL", "MSFT", "BTC-USD"])
///         .benchmark_symbol("^GSPC")
///         .start_date("2023-01-01")
///         .end_date("2024-01-01")
///         .interval(Interval::OneDay)
///         .confidence_level(0.95)
///         .risk_free_rate(0.02)
///         .weights(vec![40_000.0, 30_000.0, 20_000.0, 10_000.0])
///         .build().await?;
///
///     portfolio.performance_stats()?;
///     portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///     Ok(())
/// }
/// ```
///
#[derive(Debug, Clone)]
pub struct Portfolio {
    pub tickers: Tickers,
    pub data: PortfolioData,
    pub objective_function: ObjectiveFunction,
    pub constraints: Option<Constraints>,
    /// Per-asset dollar amounts for the portfolio weights (allocation).
    pub weights: Option<Vec<f64>>,
    /// Ad-hoc per-asset transactions (additions / withdrawals).
    pub transactions: Option<Vec<Transaction>>,
    pub rebalance_strategy: Option<RebalanceStrategy>,
    pub scheduled_cash_flows: Option<Vec<ScheduledCashFlow>>,
    pub optimization_result: Option<PortfolioOptimizationResult>,
    pub performance_stats: Option<PortfolioPerformanceStats>,
}

impl Portfolio {
    pub fn builder() -> PortfolioBuilder {
        PortfolioBuilder::new()
    }

    /// Internal constructor for use by `Tickers` convenience methods.
    pub(crate) fn new_raw(
        tickers: Tickers,
        data: PortfolioData,
        objective_function: ObjectiveFunction,
        constraints: Option<Constraints>,
        weights: Option<Vec<f64>>,
        transactions: Option<Vec<Transaction>>,
    ) -> Portfolio {
        Portfolio {
            tickers,
            data,
            objective_function,
            constraints,
            weights,
            transactions,
            rebalance_strategy: None,
            scheduled_cash_flows: None,
            optimization_result: None,
            performance_stats: None,
        }
    }

    /// Run portfolio optimization and compute in-sample performance statistics.
    ///
    /// Finds optimal weights using the builder's objective function and constraints,
    /// then immediately evaluates those weights on the current data. After this call,
    /// both `optimization_result` and `performance_stats` are populated.
    ///
    /// To evaluate the same weights out-of-sample, call `update_dates()` or
    /// `update_data()` followed by `performance_stats()`.
    ///
    /// # Returns
    ///
    /// * `&PortfolioOptimizationResult` reference to the stored optimization result
    pub fn optimize(&mut self) -> Result<&PortfolioOptimizationResult, Box<dyn Error>> {
        let result = optimize_portfolio(
            &self.data,
            self.objective_function,
            self.constraints.clone(),
        )?;
        self.optimization_result = Some(result);

        // Build rebalance config using optimized weights
        let weights = &self
            .optimization_result
            .as_ref()
            .ok_or("BUG: optimization_result was just set but is None")?
            .optimal_weights;
        let rebalance_config = self
            .rebalance_strategy
            .as_ref()
            .map(|strategy| RebalanceConfig {
                target_weights: weights.to_vec(),
                strategy: strategy.clone(),
            });

        // Compute in-sample performance stats using the optimal weights
        let txns = self.transactions.clone();
        let initial_values = self.weights.clone();
        let perf = compute_performance(
            &self.data,
            weights,
            txns,
            initial_values,
            rebalance_config.as_ref(),
            self.scheduled_cash_flows.as_deref(),
        )?;
        self.performance_stats = Some(perf);

        Ok(self
            .optimization_result
            .as_ref()
            .ok_or("BUG: optimization_result was just set but is None")?)
    }

    /// Update the portfolio's date range and re-fetch data for the new period.
    ///
    /// This is for portfolios built from Yahoo Finance data (i.e. **not** custom
    /// `tickers_data`/`benchmark_data`). It rebuilds all underlying ticker and
    /// benchmark data for the new date range. The optimization result (weights) is
    /// preserved so they can be evaluated out-of-sample on the new period.
    ///
    /// After calling this method, call `performance_stats()` to evaluate the
    /// optimized weights on the new data.
    ///
    /// # Arguments
    ///
    /// * `start_date` - New start date (e.g. "2024-01-01")
    /// * `end_date` - New end date (e.g. "2024-12-31")
    ///
    /// # Errors
    ///
    /// Returns an error if the portfolio was built from custom data. Use
    /// `update_data()` instead in that case.
    pub async fn update_dates(
        &mut self,
        start_date: &str,
        end_date: &str,
    ) -> Result<(), Box<dyn Error>> {
        if self.tickers.tickers_data.is_some() {
            return Err(
                "update_dates() is not supported for portfolios built from custom data. \
                 Use update_data() to supply new KLINE data for the evaluation period."
                    .into(),
            );
        }

        let same_dates = self.data.start_date == start_date && self.data.end_date == end_date;

        if !same_dates {
            let symbols: Vec<&str> = self
                .tickers
                .tickers
                .iter()
                .map(|t| t.ticker.as_str())
                .collect();
            let mut builder = Tickers::builder()
                .tickers(symbols)
                .start_date(start_date)
                .end_date(end_date)
                .interval(self.tickers.interval)
                .confidence_level(self.tickers.confidence_level)
                .risk_free_rate(self.tickers.risk_free_rate);
            if let Some(ref sym) = self.tickers.benchmark_symbol {
                builder = builder.benchmark_symbol(sym);
            }
            let new_tickers = builder.build();

            let new_data =
                prepare_portfolio_data(&new_tickers, new_tickers.benchmark_ticker.as_ref()).await?;

            self.tickers = new_tickers;
            self.data = new_data;
        }

        // Always clear previous performance stats — caller must invoke
        // performance_stats() to re-evaluate on the (possibly new) data.
        self.performance_stats = None;

        Ok(())
    }

    /// Update the portfolio's underlying data with new KLINE datasets.
    ///
    /// This is for portfolios built from custom data (`tickers_data`/`benchmark_data`).
    /// It rebuilds all underlying ticker and benchmark data from the supplied KLINE
    /// vectors. The optimization result (weights) is preserved so they can be evaluated
    /// out-of-sample on the new data.
    ///
    /// After calling this method, call `performance_stats()` to evaluate the
    /// optimized weights on the new data.
    ///
    /// # Arguments
    ///
    /// * `tickers_data` - New KLINE data for each asset (same tickers, different period)
    /// * `benchmark_data` - New KLINE data for the benchmark
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use finalytics::prelude::*;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let train = vec![
    ///     KLINE::from_csv("AAPL", "aapl_train.csv")?,
    ///     KLINE::from_csv("MSFT", "msft_train.csv")?,
    /// ];
    /// let train_bench = KLINE::from_csv("^GSPC", "gspc_train.csv")?;
    ///
    /// let mut portfolio = Portfolio::builder()
    ///     .tickers_data(Some(train))
    ///     .benchmark_data(Some(train_bench))
    ///     .objective_function(ObjectiveFunction::MaxSharpe)
    ///     .build().await?;
    ///
    /// portfolio.optimize()?;
    ///
    /// // Load evaluation-period data and update
    /// let eval = vec![
    ///     KLINE::from_csv("AAPL", "aapl_eval.csv")?,
    ///     KLINE::from_csv("MSFT", "msft_eval.csv")?,
    /// ];
    /// let eval_bench = KLINE::from_csv("^GSPC", "gspc_eval.csv")?;
    /// portfolio.update_data(eval, Some(eval_bench)).await?;
    /// portfolio.performance_stats()?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_data(
        &mut self,
        tickers_data: Vec<KLINE>,
        benchmark_data: Option<KLINE>,
    ) -> Result<(), Box<dyn Error>> {
        let mut builder = Tickers::builder()
            .tickers_data(Some(tickers_data))
            .confidence_level(self.tickers.confidence_level)
            .risk_free_rate(self.tickers.risk_free_rate);
        if let Some(bd) = benchmark_data {
            builder = builder.benchmark_data(Some(bd));
        }
        let new_tickers = builder.build();

        let new_data =
            prepare_portfolio_data(&new_tickers, new_tickers.benchmark_ticker.as_ref()).await?;

        self.tickers = new_tickers;
        self.data = new_data;

        // Clear previous performance stats — caller must invoke
        // performance_stats() to re-evaluate on the new data.
        self.performance_stats = None;

        Ok(())
    }

    /// Compute (or recompute) portfolio performance statistics.
    ///
    /// The weights are resolved as follows:
    /// - If `optimize()` was called: uses the optimal weights from the optimization result.
    /// - Otherwise: uses `weights` (must be `Some`).
    ///
    /// `optimize()` already computes in-sample performance stats automatically.
    /// This method is useful for:
    /// - Evaluating explicit weights (no optimization).
    /// - Recomputing stats after `update_dates()` or `update_data()` for
    ///   out-of-sample evaluation.
    ///
    /// # Returns
    ///
    /// * `&PortfolioPerformanceStats` reference to the stored result
    pub fn performance_stats(&mut self) -> Result<&PortfolioPerformanceStats, Box<dyn Error>> {
        let num_assets = self.data.portfolio_returns.width();

        // Resolve weights from optimization result or builder weights
        let resolved_weights = if let Some(ref opt) = self.optimization_result {
            opt.optimal_weights.clone()
        } else if let Some(ref w) = self.weights {
            let total: f64 = w.iter().sum();
            if total <= 0.0 {
                return Err("Total weights must be greater than zero".into());
            }
            if w.len() != num_assets {
                return Err(format!(
                    "Weights length ({}) must match the number of assets ({})",
                    w.len(),
                    num_assets
                )
                .into());
            }
            w.iter().map(|v| v / total).collect()
        } else {
            return Err("No weights or optimization result available. Either call \
                 .weights() on the builder, or call optimize() first."
                .into());
        };

        // Transactions are always taken from the portfolio's own field
        let transactions = self.transactions.clone();

        if resolved_weights.len() != num_assets {
            return Err(format!(
                "Weights length ({}) must match the number of assets ({})",
                resolved_weights.len(),
                num_assets
            )
            .into());
        }

        // Build rebalance config
        let rebalance_config = self
            .rebalance_strategy
            .as_ref()
            .map(|strategy| RebalanceConfig {
                target_weights: resolved_weights.clone(),
                strategy: strategy.clone(),
            });

        let initial_values = self.weights.clone();
        let result = compute_performance(
            &self.data,
            &resolved_weights,
            transactions,
            initial_values,
            rebalance_config.as_ref(),
            self.scheduled_cash_flows.as_deref(),
        )?;
        self.performance_stats = Some(result);
        Ok(self.performance_stats.as_ref().unwrap())
    }
}
