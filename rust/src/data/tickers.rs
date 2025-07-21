use std::error::Error;
use polars::prelude::*;
use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use crate::data::yahoo::config::TickerSummaryStats;
use crate::analytics::performance::TickerPerformanceStats;
use crate::prelude::{StatementFrequency, StatementType, TickerData, TickerPerformance, Tickers};

macro_rules! fetch_all {
    ($tickers:expr, $method:ident, $idx:expr $(, $param:expr)*) => {{
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
                let result = ticker.$method($($param), *).await;
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
                        Err(e) => eprintln!("Unable to stack {:?}: {}", &df, e),
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
    fn get_financials(&self, statement_type: StatementType, frequency: StatementFrequency) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
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

    /// Fetch the Financials for all tickers in the Tickers Struct
    async fn get_financials(&self, statement_type: StatementType, frequency: StatementFrequency) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), get_financials, 1, statement_type, frequency)
    }


    /// Fetch the Ticker Summary Stats Data for all tickers in the Tickers' Struct
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
                        Ok((ticker.ticker.clone(), stats))
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
        let mut all_stats: Vec<(String, TickerSummaryStats)> = Vec::new();

        for result in results {
            match result {
                Ok(Ok(stats)) => {
                    all_stats.push(stats);
                }
                Ok(Err(_)) => continue,
                Err(e) => eprintln!("Error in task: {e}"),
            }
        }

        let mut formatted_dfs = Vec::new();

        for (ticker, stats) in all_stats {

            let mut fmt_df = stats.to_dataframe()?;
            fmt_df.rename("Value", &ticker)?;
            formatted_dfs.push(fmt_df);
        }

        let combine = |dfs: Vec<DataFrame>| -> Result<DataFrame, Box<dyn Error>> {
            let mut combined = dfs.first().ok_or("No data")?.clone();
            for df in dfs.iter().skip(1) {
                combined = combined.left_join(df, ["Metric"], ["Metric"])?;
            }
            Ok(combined)
        };

        let mut stats = combine(formatted_dfs)?;
        let columns = stats.column("Metric")?.str()?.into_no_null_iter()
            .map(|x| x.to_string()).collect::<Vec<String>>();
        stats = stats.drop("Metric")?;
        let symbols = Series::new("Symbol", stats.get_column_names());
        let mut stats_df = stats.transpose(None, None)?;
        stats_df.set_column_names(&columns)?;
        let _ = stats_df.insert_column(0, symbols)?;

        // Drop columns where all values are empty strings
        let columns_to_drop: Vec<String> = stats_df
            .get_column_names()
            .iter()
            .filter(|&&name| name != "Symbol")
            .filter_map(|&col_name| {
                let col = stats_df.column(col_name).ok()?;
                let utf8 = col.str().ok()?;
                if utf8.into_no_null_iter().all(|val| val.trim().is_empty()) {
                    Some(col_name.to_string())
                } else {
                    None
                }
            })
            .collect();

        for col_name in columns_to_drop {
            stats_df = stats_df.drop(&col_name)?;
        }

        pb.finish_with_message("Done");

        Ok(stats_df)
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
                        if df.width() > 3 {
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
                Err(e) => eprintln!("Error in task: {e}"),
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
                        if let Ok(df) = DataFrame::new(vec![date_series, returns_series]) {
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
                Err(e) => eprintln!("Error in task: {e}"),
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
                Err(e) => eprintln!("Error in task: {e}"),
            }
        }

        let mut ticker_symbols: Vec<String> = vec![];
        let mut numeric_fields: Vec<Vec<f64>> = vec![vec![]; 16];

        for stat in &all_stats {
            ticker_symbols.push(stat.ticker_symbol.clone());
            numeric_fields[0].push(stat.performance_stats.daily_return);
            numeric_fields[1].push(stat.performance_stats.daily_volatility);
            numeric_fields[2].push(stat.performance_stats.cumulative_return);
            numeric_fields[3].push(stat.performance_stats.annualized_return);
            numeric_fields[4].push(stat.performance_stats.annualized_volatility);
            numeric_fields[5].push(stat.performance_stats.alpha);
            numeric_fields[6].push(stat.performance_stats.beta);
            numeric_fields[7].push(stat.performance_stats.sharpe_ratio);
            numeric_fields[8].push(stat.performance_stats.sortino_ratio);
            numeric_fields[9].push(stat.performance_stats.active_return);
            numeric_fields[10].push(stat.performance_stats.active_risk);
            numeric_fields[11].push(stat.performance_stats.information_ratio);
            numeric_fields[12].push(stat.performance_stats.calmar_ratio);
            numeric_fields[13].push(stat.performance_stats.maximum_drawdown);
            numeric_fields[14].push(stat.performance_stats.value_at_risk);
            numeric_fields[15].push(stat.performance_stats.expected_shortfall);
        }

        let df = DataFrame::new(vec![
            Series::new("Symbol", ticker_symbols),
            Series::new("Daily Return", numeric_fields[0].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Daily Volatility", numeric_fields[1].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Cumulative Return", numeric_fields[2].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Annualized Return", numeric_fields[3].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Annualized Volatility", numeric_fields[4].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Alpha", numeric_fields[5].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Beta", numeric_fields[6].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Sharpe Ratio", numeric_fields[7].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Sortino Ratio", numeric_fields[8].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Active Return", numeric_fields[9].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Active Risk", numeric_fields[10].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Information Ratio", numeric_fields[11].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Calmar Ratio", numeric_fields[12].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Maximum Drawdown", numeric_fields[13].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Value at Risk", numeric_fields[14].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
            Series::new("Expected Shortfall", numeric_fields[15].iter().map(|x| x.to_string()).collect::<Vec<String>>()),
        ])?;

        pb.finish_with_message("Done");

        Ok(df)
    }
}