use std::error::Error;
use polars::prelude::*;
use plotly::layout::Axis;
use futures::future::join_all;
use plotly::common::{ColorScalePalette, Mode, Title};
use plotly::{HeatMap, Layout, Plot, Scatter};
use indicatif::{ProgressBar, ProgressStyle};
use crate::data::config::TickerSummaryStats;
use crate::analytics::statistics::{correlation_matrix, cumulative_returns_list};
use crate::analytics::performance::TickerPerformanceStats;
use crate::prelude::{Financials, Interval, ObjectiveFunction, Portfolio, PortfolioBuilder, Ticker, TickerBuilder, TickerData, TickerPerformance};

macro_rules! fetch_all {
    ($tickers:expr, $method:ident, $idx:expr) => {{
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
                let result = ticker.$method().await;
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

pub struct TickersBuilder {
    tickers: Vec<String>,
    start_date: String,
    end_date: String,
    interval: Interval,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
}

impl TickersBuilder {
    pub fn new() -> TickersBuilder {
        TickersBuilder {
            tickers: Vec::new(),
            start_date: String::new(),
            end_date: String::new(),
            interval: Interval::OneDay,
            benchmark_symbol: String::from("^GSPC"),
            confidence_level: 0.95,
            risk_free_rate: 0.02,
        }
    }

    pub fn tickers(&mut self, tickers: Vec<&str>) -> &mut TickersBuilder {
        self.tickers = tickers.iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn start_date(&mut self, start_date: &str) -> &mut TickersBuilder {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(&mut self, end_date: &str) -> &mut TickersBuilder {
        self.end_date = end_date.to_string();
        self
    }

    pub fn interval(&mut self, interval: Interval) -> &mut TickersBuilder {
        self.interval = interval;
        self
    }

    pub fn benchmark_symbol(&mut self, benchmark_symbol: &str) -> &mut TickersBuilder {
        self.benchmark_symbol = benchmark_symbol.to_string();
        self
    }

    pub fn confidence_level(&mut self, confidence_level: f64) -> &mut TickersBuilder {
        self.confidence_level = confidence_level;
        self
    }

    pub fn risk_free_rate(&mut self, risk_free_rate: f64) -> &mut TickersBuilder {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn build(&self) -> Tickers {
        Tickers {
            tickers: self.tickers.clone().into_iter().map(|x|
                TickerBuilder::new().ticker(&x)
                    .start_date(&self.start_date)
                    .end_date(&self.end_date)
                    .interval(self.interval)
                    .benchmark_symbol(&self.benchmark_symbol)
                    .confidence_level(self.confidence_level)
                    .risk_free_rate(self.risk_free_rate)
                    .build()
            ).collect(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            interval: self.interval,
            benchmark_symbol: self.benchmark_symbol.clone(),
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
        }
    }
}


impl Tickers {

    /// Fetch the OHLCV Data for all tickers in the Tickers Struct
    pub async fn get_chart(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), get_chart, 1)
    }

    /// Fetch the Historical News Headlines for all tickers in the Tickers Struct
    pub async fn get_news(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), get_news, 1)
    }

    /// Fetch the Income Statement Data for all tickers in the Tickers Struct
    pub async fn income_statement(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), income_statement, 1)
    }

    /// Fetch the Balance Sheet Data for all tickers in the Tickers Struct
    pub async fn balance_sheet(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), balance_sheet, 1)
    }

    /// Fetch the Cashflow Statement Data for all tickers in the Tickers Struct
    pub async fn cashflow_statement(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), cashflow_statement, 1)
    }

    /// Fetch the Financial Ratios Data for all tickers in the Tickers Struct
    pub async fn financial_ratios(&self) -> Result<DataFrame, Box<dyn Error>> {
        fetch_all!(self.tickers.clone(), financial_ratios, 1)
    }

    /// Fetch the Ticker Summary Stats Data for all tickers in the Tickers Struct
    pub async fn get_ticker_stats(&self) -> Result<DataFrame, Box<dyn Error>> {
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
    pub async fn get_options(&self) -> Result<DataFrame, Box<dyn Error>> {
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
    pub async fn returns(&self) -> Result<DataFrame, Box<dyn Error>> {
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

        joint_df = joint_df.fill_null(FillNullStrategy::Forward(None))?;
        joint_df = joint_df.fill_null(FillNullStrategy::Backward(None))?;

        Ok(joint_df)
    }

    /// Display a Cumulative Returns Chart for all tickers in the Tickers Struct
    pub async fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let symbols = self.tickers.iter().map(|x| x.ticker.clone()).collect::<Vec<String>>();
        let asset_returns = self.returns().await?;
        let dates = asset_returns.column("timestamp")?.str()?.into_no_null_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let mut plot = Plot::new();

        for symbol in symbols {
            match asset_returns.column(&symbol) {
                Ok(returns_series) => {
                    let returns = returns_series.f64().unwrap().to_vec()
                        .iter().map(|x| x.unwrap_or_default()).collect::<Vec<f64>>();
                    let cum_returns = cumulative_returns_list(returns.clone());
                    let cum_returns_trace = Scatter::new(dates.clone(), cum_returns.clone())
                        .name(format!("{}", symbol))
                        .mode(Mode::Lines);
                    plot.add_trace(cum_returns_trace);
                }
                Err(e) => {
                    eprintln!("Unable to fetch returns for {}: {}", symbol, e);
                }
            }
        }

        let layout = Layout::new()
            .height(height.unwrap_or(800))
            .width(width.unwrap_or(1200))
            .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Tickers Cumulative Returns</span>"))
            .y_axis(
                Axis::new()
                    .title(Title::from("Cumulative Returns"))
                    .tick_format(".0%")
            );

        plot.set_layout(layout);
        Ok(plot)
    }

    pub async fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> Result<Plot, Box<dyn Error>> {
        let mut returns = self.returns().await?;
        let _ = returns.drop_in_place("timestamp");
        let labels = returns.get_column_names().iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let corr_matrix = correlation_matrix(&returns)?;
        let corr_matrix = corr_matrix.outer_iter()
            .map(|row| row.to_vec())
            .collect();
        let heatmap = HeatMap::new(labels.to_vec(), labels.to_vec(), corr_matrix)
            .zmin(-1.0)
            .zmax(1.0)
            .color_scale(ColorScalePalette::Jet.into());

        let mut plot = Plot::new();
        plot.add_trace(heatmap);
        plot.set_layout(
            Layout::new()
                .title(Title::from("<span style=\"font-weight:bold; color:darkgreen;\">Returns Correlation Matrix</span>"))
                .height(height.unwrap_or(800))
                .width(width.unwrap_or(1200))
        );

        Ok(plot)
    }

    /// Fetch the performance statistics for all tickers in the Tickers Struct
    pub async fn performance_stats(&self) -> Result<DataFrame, Box<dyn Error>> {
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

        let mut fields: Vec<Vec<AnyValue>> = vec![vec![]; 17];

        for stat in &all_stats {
            fields[0].push(AnyValue::String(&stat.ticker_symbol));
            fields[1].push(AnyValue::Float64(stat.performance_stats.daily_return));
            fields[2].push(AnyValue::Float64(stat.performance_stats.daily_volatility));
            fields[3].push(AnyValue::Float64(stat.performance_stats.cumulative_return));
            fields[4].push(AnyValue::Float64(stat.performance_stats.annualized_return));
            fields[5].push(AnyValue::Float64(stat.performance_stats.annualized_volatility));
            fields[6].push(AnyValue::Float64(stat.performance_stats.alpha));
            fields[7].push(AnyValue::Float64(stat.performance_stats.beta));
            fields[8].push(AnyValue::Float64(stat.performance_stats.sharpe_ratio));
            fields[9].push(AnyValue::Float64(stat.performance_stats.sortino_ratio));
            fields[10].push(AnyValue::Float64(stat.performance_stats.active_return));
            fields[11].push(AnyValue::Float64(stat.performance_stats.active_risk));
            fields[12].push(AnyValue::Float64(stat.performance_stats.information_ratio));
            fields[13].push(AnyValue::Float64(stat.performance_stats.calmar_ratio));
            fields[14].push(AnyValue::Float64(stat.performance_stats.maximum_drawdown));
            fields[15].push(AnyValue::Float64(stat.performance_stats.value_at_risk));
            fields[16].push(AnyValue::Float64(stat.performance_stats.expected_shortfall));
        }

        let df = DataFrame::new(vec![
            Series::new("symbol", fields[0].clone()),
            Series::new("daily_return", fields[1].clone()),
            Series::new("daily_volatility", fields[2].clone()),
            Series::new("cumulative_return", fields[3].clone()),
            Series::new("annualized_return", fields[4].clone()),
            Series::new("annualized_volatility", fields[5].clone()),
            Series::new("alpha", fields[6].clone()),
            Series::new("beta", fields[7].clone()),
            Series::new("sharpe_ratio", fields[8].clone()),
            Series::new("sortino_ratio", fields[9].clone()),
            Series::new("active_return", fields[10].clone()),
            Series::new("active_risk", fields[11].clone()),
            Series::new("information_ratio", fields[12].clone()),
            Series::new("calmar_ratio", fields[13].clone()),
            Series::new("maximum_drawdown", fields[14].clone()),
            Series::new("value_at_risk", fields[15].clone()),
            Series::new("expected_shortfall", fields[16].clone()),
        ])?;

        pb.finish_with_message("Done");

        Ok(df)
    }

    /// Fetch a single Ticker Struct from the Tickers Struct
    pub async fn get_ticker(self, symbol: &str) -> Result<Ticker, Box<dyn Error>> {
        let ticker = self.tickers.iter().find(|x| x.ticker == symbol);
        match ticker {
            Some(t) => Ok(t.clone()),
            None => Err("Ticker not found".into())
        }
    }

    /// Optimize a Portfolio of multiple tickers within the Tickers Struct
    pub async fn optimize(&self, objective_function: Option<ObjectiveFunction>, constraints: Option<Vec<(f64, f64)>>) -> Result<Portfolio, Box<dyn Error>> {
        let symbols = self.tickers.iter().map(|x| &*x.ticker).collect::<Vec<&str>>();
        PortfolioBuilder::new()
            .ticker_symbols(symbols)
            .benchmark_symbol(&self.benchmark_symbol)
            .start_date(&self.start_date)
            .end_date(&self.end_date)
            .interval(self.interval)
            .confidence_level(self.confidence_level)
            .risk_free_rate(self.risk_free_rate)
            .objective_function(objective_function.unwrap_or(ObjectiveFunction::MaxSharpe))
            .constraints(constraints)
            .build().await
    }
}

/// Tickers Struct
///
/// ### Description
/// - This is the main Interface for the `Finalytics` Library.
/// - It provides methods to:
///     - fetch data for multiple tickers in an asynchronous manner.
///     - compute the tickers returns, performance stats, and display the cumulative returns chart.
///     - initialize the Ticker and Portfolio Structs, providing an interface for calling their respective methods.
///
/// ### Data Methods
/// - `get_stats` - Fetches Ticker Summary Stats Data for multiple tickers
/// - `get_chart` - Fetches Ticker OHLCV Data for multiple tickers
/// - `get_news` - Fetches Ticker News Data for multiple tickers
/// - `get_options` - Fetches Ticker Option Chain Data for multiple tickers
/// - `income_statement` - Fetches Ticker Income Statement Data for multiple tickers
/// - `balance_sheet` - Fetches Ticker Balance Sheet Data for multiple tickers
/// - `cashflow_statement` - Fetches Ticker Cashflow Statement Data for multiple tickers
/// - `financial_ratios` - Fetches Ticker Financial Ratios Data for multiple tickers
///
/// ### Performance Methods
/// - `returns` - Fetches Ticker Returns Data for multiple tickers
/// - `returns_chart` - Displays the Cumulative Returns Chart for multiple tickers
/// - `returns_matrix` - Displays the Returns Correlation Matrix for multiple tickers
/// - `performance_stats` - Fetches Ticker Performance Stats Data for multiple tickers
///
/// ### Interface Methods
/// - `get_ticker` - Fetches a single Ticker Struct from the Tickers Struct
/// - `optimize` - Optimizes a Portfolio of multiple tickers, returning a Portfolio Struct
///
/// ### Constructor
/// - The `Tickers` struct can be instantiated using the `TickersBuilder` struct.
///
/// ### Example
/// ```rust
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///    let symbols = vec!["AAPL", "MSFT", "NVDA", "BTC-USD"];
///    let tickers = TickersBuilder::new()
///        .tickers(symbols)
///        .start_date("2023-01-01")
///        .end_date("2023-12-31")
///        .interval(Interval::OneDay)
///        .benchmark_symbol("^GSPC")
///        .confidence_level(0.95)
///        .risk_free_rate(0.02)
///        .build();
///
///   // Fetch Data for Multiple Tickers
///   let df = tickers.get_ticker_stats().await.unwrap();
///   println!("{:?}", df);
///   let df = tickers.get_chart().await.unwrap();
///   println!("{:?}", df);
///   let df = tickers.get_options().await.unwrap();
///   println!("{:?}", df);
///   let df = tickers.get_news().await.unwrap();
///   println!("{:?}", df);
///   let df = tickers.income_statement().await.unwrap();
///   println!("{:?}", df);
///   let df = tickers.balance_sheet().await.unwrap();
///   println!("{:?}", df);
///   let df = tickers.cashflow_statement().await.unwrap();
///   println!("{:?}", df);
///   let df = tickers.financial_ratios().await.unwrap();
///   println!("{:?}", df);
///
///  // Compute Performance Stats for Multiple Tickers
///  let df = tickers.returns().await.unwrap();
///  println!("{:?}", df);
///  let plot = tickers.returns_chart(None, None).await.unwrap();
///  plot.show();
///  let plot = tickers.returns_matrix(None, None).await.unwrap();
///  plot.show();
///  let df = tickers.performance_stats().await.unwrap();
///  println!("{:?}", df);
///
///  // Fetch a Single Ticker from the Tickers Struct
///  let ticker = tickers.clone().get_ticker("AAPL").await.unwrap();
///  let performance_chart = ticker.performance_chart(None, None).await.unwrap();
///  performance_chart.show();
///
///  // Optimize a Portfolio of Multiple Tickers
///  let portfolio = tickers.clone().optimize(None, None).await.unwrap();
///  println!("{:?}", portfolio.performance_stats);
///  let optimization_chart = portfolio.performance_chart(None, None).unwrap();
///  optimization_chart.show();
///
///
/// }
/// ```

#[derive(Debug, Clone)]
pub struct Tickers {
    pub tickers: Vec<Ticker>,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub benchmark_symbol: String,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
}
