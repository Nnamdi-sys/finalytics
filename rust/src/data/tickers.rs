use std::error::Error;
use polars::prelude::*;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use crate::data::config::TickerSummaryStats;
use crate::analytics::performance::TickerPerformanceStats;
use crate::prelude::{Financials, StatementFrequency, TickerData, TickerPerformance, Tickers};

macro_rules! fetch_all {
    ($tickers:expr, $method:ident, $idx:expr $(, $param:expr)?) => {{
        let mut futures = Vec::new();
        let tickers = $tickers.clone();
        let total_tickers = tickers.len();
        let pb = ProgressBar::new(total_tickers as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
                .progress_chars("#>-"),
        );

        for ticker in tickers.into_iter() {
            let ticker = ticker.clone();
            let fut = tokio::task::spawn(async move {
                let result = ticker.$method($($param)?).await;
                match result {
                    Ok(mut df) => {
                        let symbol_series = Series::new("symbol", vec![ticker.ticker.clone(); df.height()]);
                        if df.width() > $idx {
                            let _ = df.insert_column($idx, symbol_series);
                            Ok(df)
                        } else {
                            eprintln!("No Data for {}", &ticker.ticker);
                            Err(format!("No Data for {}", &ticker.ticker))
                        }
                    }
                    Err(e) => {
                        eprintln!("Error Fetching Data for {}: {}", &ticker.ticker, e);
                        Err(format!("Error Fetching Data for {}: {}", &ticker.ticker, e))
                    }
                }
            });

            futures.push(fut);
        }

        let results = join_all(futures).await;
        let mut joint_df = DataFrame::default();

        for result in results {
            match result {
                Ok(Ok(df)) => {
                    match joint_df.vstack(&df) {
                        Ok(jdf) => joint_df = jdf,
                        Err(e) => eprintln!("Unable to Vstack {:?}: {}", &df, e),
                    }
                }
                Ok(Err(_)) => continue,
                Err(e) => eprintln!("Error in task: {}", e),
            }
        }

        pb.finish_with_message(format!("Done"));
        Ok(joint_df)
    }};
}

pub trait TickersData {
    fn get_chart(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn get_news(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn income_statement(&self, frequency: StatementFrequency) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn balance_sheet(&self, frequency: StatementFrequency) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn cashflow_statement(&self, frequency: StatementFrequency) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn financial_ratios(&self, frequency: StatementFrequency) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn get_ticker_stats(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn get_options(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn returns(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn performance_stats(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
}


impl TickersData for Tickers {
    /// Fetch the OHLCV Data for all tickers in the Tickers Struct
    async fn get_chart(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), get_chart, 1)
    }

    /// Fetch the Historical News Headlines for all tickers in the Tickers Struct
    async fn get_news(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), get_news, 1)
    }

    /// Fetch the Income Statement Data for all tickers in the Tickers Struct
    async fn income_statement(&self, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), income_statement, 1, frequency)
    }

    /// Fetch the Balance Sheet Data for all tickers in the Tickers Struct
    async fn balance_sheet(&self, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), balance_sheet, 1, frequency)
    }

    /// Fetch the Cashflow Statement Data for all tickers in the Tickers Struct
    async fn cashflow_statement(&self, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), cashflow_statement, 1, frequency)
    }

    /// Fetch the Financial Ratios Data for all tickers in the Tickers Struct
    async fn financial_ratios(&self, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), financial_ratios, 1, frequency)
    }

    /// Fetch the Ticker Summary Stats Data for all tickers in the Tickers Struct
    async fn get_ticker_stats(&self) -> Result<DataFrame, Box<dyn Error>> {
        let mut futures = Vec::new();
        let total_tickers = self.tickers.len();
        let pb = ProgressBar::new(total_tickers as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
                .progress_chars("#>-"),
        );

        for ticker in self.tickers.clone().into_iter() {
            let fut = tokio::task::spawn(async move {
                match ticker.get_ticker_stats().await {
                    Ok(stats) => {
                        Ok(stats)
                    }
                    Err(e) => {
                        eprintln!("Error Fetching Ticker Stats for {}: {}", &ticker.ticker, e);
                        Err(format!("Error Fetching Ticker Stats for {}: {}", &ticker.ticker, e))
                    }
                }
            });

            futures.push(fut);
        }

        let results = join_all(futures).await;
        let mut all_stats: Vec<TickerSummaryStats> = Vec::new();

        for result in results {
            match result {
                Ok(Ok(stats)) => {
                    all_stats.push(stats);
                }
                Ok(Err(_)) => continue,
                Err(e) => eprintln!("Error in task: {}", e),
            }
        }

        let mut fields: Vec<Vec<AnyValue>> = vec![vec![]; 29]; // We have 28 fields excluding 'symbol'

        for stat in &all_stats {
            fields[0].push(AnyValue::String(&stat.symbol));
            fields[1].push(AnyValue::String(&stat.long_name));
            fields[2].push(AnyValue::String(&stat.full_exchange_name));
            fields[3].push(AnyValue::String(&stat.currency));
            fields[4].push(AnyValue::Int64(stat.regular_market_time));
            fields[5].push(AnyValue::Float64(stat.regular_market_price));
            fields[6].push(AnyValue::Float64(stat.regular_market_change_percent));
            fields[7].push(AnyValue::Float64(stat.regular_market_volume));
            fields[8].push(AnyValue::Float64(stat.regular_market_open));
            fields[9].push(AnyValue::Float64(stat.regular_market_day_high));
            fields[10].push(AnyValue::Float64(stat.regular_market_day_low));
            fields[11].push(AnyValue::Float64(stat.regular_market_previous_close));
            fields[12].push(AnyValue::Float64(stat.fifty_two_week_high));
            fields[13].push(AnyValue::Float64(stat.fifty_two_week_low));
            fields[14].push(AnyValue::Float64(stat.fifty_two_week_change_percent));
            fields[15].push(AnyValue::Float64(stat.fifty_day_average));
            fields[16].push(AnyValue::Float64(stat.two_hundred_day_average));
            fields[17].push(AnyValue::Float64(stat.trailing_eps));
            fields[18].push(AnyValue::Float64(stat.current_eps));
            fields[19].push(AnyValue::Float64(stat.eps_forward));
            fields[20].push(AnyValue::Float64(stat.trailing_pe));
            fields[21].push(AnyValue::Float64(stat.current_pe));
            fields[22].push(AnyValue::Float64(stat.forward_pe));
            fields[23].push(AnyValue::Float64(stat.dividend_rate));
            fields[24].push(AnyValue::Float64(stat.dividend_yield));
            fields[25].push(AnyValue::Float64(stat.book_value));
            fields[26].push(AnyValue::Float64(stat.price_to_book));
            fields[27].push(AnyValue::Float64(stat.market_cap));
            fields[28].push(AnyValue::Float64(stat.shares_outstanding));
        }

        let df = DataFrame::new(vec![
            Series::new("symbol", fields[0].clone()),
            Series::new("long_name", fields[1].clone()),
            Series::new("full_exchange_name", fields[2].clone()),
            Series::new("currency", fields[3].clone()),
            Series::new("regular_market_time", fields[4].clone()),
            Series::new("regular_market_price", fields[5].clone()),
            Series::new("regular_market_change_percent", fields[6].clone()),
            Series::new("regular_market_volume", fields[7].clone()),
            Series::new("regular_market_open", fields[8].clone()),
            Series::new("regular_market_day_high", fields[9].clone()),
            Series::new("regular_market_day_low", fields[10].clone()),
            Series::new("regular_market_previous_close", fields[11].clone()),
            Series::new("fifty_two_week_high", fields[12].clone()),
            Series::new("fifty_two_week_low", fields[13].clone()),
            Series::new("fifty_two_week_change_percent", fields[14].clone()),
            Series::new("fifty_day_average", fields[15].clone()),
            Series::new("two_hundred_day_average", fields[16].clone()),
            Series::new("trailing_eps", fields[17].clone()),
            Series::new("current_eps", fields[18].clone()),
            Series::new("eps_forward", fields[19].clone()),
            Series::new("trailing_pe", fields[20].clone()),
            Series::new("current_pe", fields[21].clone()),
            Series::new("forward_pe", fields[22].clone()),
            Series::new("dividend_rate", fields[23].clone()),
            Series::new("dividend_yield", fields[24].clone()),
            Series::new("book_value", fields[25].clone()),
            Series::new("price_to_book", fields[26].clone()),
            Series::new("market_cap", fields[27].clone()),
            Series::new("shares_outstanding", fields[28].clone()),
        ])?;

        pb.finish_with_message("Done");
        Ok(df)
    }

    /// Fetch the Options Chain Data for all tickers in the Tickers Struct
    async fn get_options(&self) -> Result<DataFrame, Box<dyn Error>> {
        let mut futures = Vec::new();
        let total_tickers = self.tickers.len();
        let pb = ProgressBar::new(total_tickers as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
                .progress_chars("#>-"),
        );

        for ticker in self.tickers.clone().into_iter() {
            let fut = tokio::task::spawn(async move {
                match ticker.get_options().await {
                    Ok(options) => {
                        let mut df = options.chain;
                        let symbol_series = Series::new("symbol", vec![ticker.ticker.clone(); df.height()]);
                        return if df.width() > 3 {
                            let _ = df.insert_column(3, symbol_series);
                            Ok(df)
                        } else {
                            eprintln!("No Options Data for {}", &ticker.ticker);
                            Err(format!("No Options Data for {}", &ticker.ticker))
                        }
                    }
                    Err(e) => {
                        eprintln!("Error Fetching Options Data for {}: {}", &ticker.ticker, e);
                        Err(format!("Error Fetching Options Data for {}: {}", &ticker.ticker, e))
                    }
                }
            });

            futures.push(fut);
        }

        let results = join_all(futures).await;
        let mut joint_df = DataFrame::default();

        for result in results {
            match result {
                Ok(Ok(df)) => {
                    joint_df = joint_df.vstack(&df)?;
                }
                Ok(Err(_)) => continue,
                Err(e) => eprintln!("Error in task: {}", e),
            }
        }

        pb.finish_with_message("Done");

        Ok(joint_df)
    }

    /// Compute the Returns for all tickers in the Tickers Struct
    async fn returns(&self) -> Result<DataFrame, Box<dyn Error>> {
        let mut futures = Vec::new();
        let total_tickers = self.tickers.len();
        let pb = ProgressBar::new(total_tickers as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
                .progress_chars("#>-"),
        );

        for ticker in self.tickers.clone().into_iter() {
            let fut = tokio::task::spawn(async move {
                match ticker.performance_stats().await {
                    Ok(stats) => {
                        let date_series = Series::new("timestamp", stats.dates_array);
                        let returns_series = Series::new(&ticker.ticker, stats.security_returns);
                        return if let Ok(df) = DataFrame::new(vec![date_series, returns_series]) {
                            Ok(df)
                        } else {
                            eprintln!("No Returns Data for {}", &ticker.ticker);
                            Err(format!("No Returns Data for {}", &ticker.ticker))
                        }
                    }
                    Err(e) => {
                        eprintln!("No Returns Data for {}: {}", &ticker.ticker, e);
                        Err(format!("No Returns Data for {}: {}", &ticker.ticker, e))
                    }
                }
            });

            futures.push(fut);
        }

        let results = join_all(futures).await;
        let mut joint_df = DataFrame::default();

        for result in results {
            match result {
                Ok(Ok(df)) => {
                    if joint_df.width() == 0 {
                        joint_df = df;
                    } else {
                        joint_df = joint_df
                            .join(
                                &df,
                                &["timestamp"],
                                &["timestamp"],
                                JoinArgs::new(JoinType::Full).with_coalesce(JoinCoalesce::CoalesceColumns),
                            )?;
                    }
                }
                Ok(Err(_)) => continue,
                Err(e) => eprintln!("Error in task: {}", e),
            }
        }

        pb.finish_with_message("Done");

        joint_df = joint_df.fill_null(FillNullStrategy::Zero)?;

        Ok(joint_df)
    }

    /// Fetch the performance statistics for all tickers in the Tickers Struct
    async fn performance_stats(&self) -> Result<DataFrame, Box<dyn Error>> {
        let mut futures = Vec::new();
        let total_tickers = self.tickers.len();
        let pb = ProgressBar::new(total_tickers as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
                .progress_chars("#>-"),
        );

        for ticker in self.tickers.clone().into_iter() {
            let fut = tokio::task::spawn(async move {
                match ticker.performance_stats().await {
                    Ok(stats) => {
                        Ok(stats)
                    }
                    Err(e) => {
                        eprintln!("No Returns Data for {}: {}", &ticker.ticker, e);
                        Err(format!("No Returns Data for {}: {}", &ticker.ticker, e))
                    }
                }
            });

            futures.push(fut);
        }

        let results = join_all(futures).await;
        let mut all_stats: Vec<TickerPerformanceStats> = Vec::new();

        for result in results {
            match result {
                Ok(Ok(stats)) => {
                    all_stats.push(stats);
                }
                Ok(Err(_)) => continue,
                Err(e) => eprintln!("Error in task: {}", e),
            }
        }

        let mut fields: Vec<Vec<String>> = vec![vec![]; 17];

        for stat in &all_stats {
            fields[0].push(stat.ticker_symbol.clone());
            fields[1].push(format!("{:.2}%", stat.performance_stats.daily_return));
            fields[2].push(format!("{:.2}%", stat.performance_stats.daily_volatility));
            fields[3].push(format!("{:.2}%", stat.performance_stats.cumulative_return));
            fields[4].push(format!("{:.2}%", stat.performance_stats.annualized_return));
            fields[5].push(format!("{:.2}%", stat.performance_stats.annualized_volatility));
            fields[6].push(format!("{:.2}", stat.performance_stats.alpha));
            fields[7].push(format!("{:.2}", stat.performance_stats.beta));
            fields[8].push(format!("{:.2}", stat.performance_stats.sharpe_ratio));
            fields[9].push(format!("{:.2}", stat.performance_stats.sortino_ratio));
            fields[10].push(format!("{:.2}%", stat.performance_stats.active_return));
            fields[11].push(format!("{:.2}%", stat.performance_stats.active_risk));
            fields[12].push(format!("{:.2}", stat.performance_stats.information_ratio));
            fields[13].push(format!("{:.2}", stat.performance_stats.calmar_ratio));
            fields[14].push(format!("{:.2}%", stat.performance_stats.maximum_drawdown));
            fields[15].push(format!("{:.2}%", stat.performance_stats.value_at_risk));
            fields[16].push(format!("{:.2}%", stat.performance_stats.expected_shortfall));
        }

        let df = DataFrame::new(vec![
            Series::new("Symbol", fields[0].clone()),
            Series::new("Daily Return", fields[1].clone()),
            Series::new("Daily Volatility", fields[2].clone()),
            Series::new("Cumulative Return", fields[3].clone()),
            Series::new("Annualized Return", fields[4].clone()),
            Series::new("Annualized Volatility", fields[5].clone()),
            Series::new("Alpha", fields[6].clone()),
            Series::new("Beta", fields[7].clone()),
            Series::new("Sharpe Ratio", fields[8].clone()),
            Series::new("Sortino Ratio", fields[9].clone()),
            Series::new("Active Return", fields[10].clone()),
            Series::new("Active Risk", fields[11].clone()),
            Series::new("Information Ratio", fields[12].clone()),
            Series::new("Calmar Ratio", fields[13].clone()),
            Series::new("Maximum Drawdown", fields[14].clone()),
            Series::new("Value at Risk", fields[15].clone()),
            Series::new("Expected Shortfall", fields[16].clone()),
        ])?;

        pb.finish_with_message("Done");

        Ok(df)
    }
}