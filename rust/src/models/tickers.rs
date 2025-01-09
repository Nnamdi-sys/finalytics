use std::error::Error;
use crate::prelude::{Interval, ObjectiveFunction, Portfolio, PortfolioBuilder, Ticker, TickerBuilder};


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
    /// Fetch a single Ticker Struct from the Tickers Struct
    ///
    /// ### Arguments
    /// - `symbol` - The Ticker Symbol
    ///
    /// ### Returns
    ///
    /// - A `Ticker` Struct
    pub async fn get_ticker(self, symbol: &str) -> Result<Ticker, Box<dyn Error>> {
        let ticker = self.tickers.iter().find(|x| x.ticker == symbol);
        match ticker {
            Some(t) => Ok(t.clone()),
            None => Err("Ticker not found".into())
        }
    }

    /// Optimize a Portfolio of multiple tickers within the Tickers Struct
    ///
    /// ### Arguments
    /// - `objective_function` - The Objective Function to optimize the Portfolio
    /// - `constraints` - The Portfolio Constraints
    ///
    /// ### Returns
    ///
    /// - A `Portfolio` Struct
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
///     - compute performance statistics for multiple tickers and display html reports.
///     - initialize the Ticker and Portfolio Structs, providing an interface for calling their respective methods.
///
/// ### Constructor
/// - The `Tickers` struct can be instantiated using the `TickersBuilder` struct.
///
/// ### Example
/// ```rust
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>>  {
///   // Instantiate a Tickers Object
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
///   // Generate a Single Ticker Report
///    let ticker = tickers.clone().get_ticker("AAPL").await?;
///    ticker.report(Some(ReportType::Performance)).await?.show()?;
///    ticker.report(Some(ReportType::Financials)).await?.show()?;
///    ticker.report(Some(ReportType::Options)).await?.show()?;
///    ticker.report(Some(ReportType::News)).await?.show()?;
///
///    // Generate a Multiple Ticker Report
///    tickers.report(Some(ReportType::Performance)).await?.show()?;
///
///    // Perform a Portfolio Optimization
///    let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None).await?;
///
///    // Generate a Portfolio Report
///    portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///    Ok(())
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
