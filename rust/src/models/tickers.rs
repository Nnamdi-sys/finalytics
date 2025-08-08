use std::error::Error;
use crate::analytics::performance::PortfolioPerformanceStats;
use crate::prelude::{Interval, ObjectiveFunction, Portfolio, Ticker, KLINE};


pub struct TickersBuilder {
    tickers: Vec<String>,
    start_date: String,
    end_date: String,
    interval: Interval,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
    tickers_data: Option<Vec<KLINE>>,
    benchmark_data: Option<KLINE>,
}

impl Default for TickersBuilder {
    fn default() -> Self {
        Self::new()
    }
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
            risk_free_rate: 0.0,
            tickers_data: None,
            benchmark_data: None,
        }
    }

    pub fn tickers(mut self, tickers: Vec<&str>) -> TickersBuilder {
        self.tickers = tickers.iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn start_date(mut self, start_date: &str) -> TickersBuilder {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(mut self, end_date: &str) -> TickersBuilder {
        self.end_date = end_date.to_string();
        self
    }

    pub fn interval(mut self, interval: Interval) -> TickersBuilder {
        self.interval = interval;
        self
    }

    pub fn benchmark_symbol(mut self, benchmark_symbol: &str) -> TickersBuilder {
        self.benchmark_symbol = benchmark_symbol.to_string();
        self
    }

    pub fn confidence_level(mut self, confidence_level: f64) -> TickersBuilder {
        self.confidence_level = confidence_level;
        self
    }

    pub fn risk_free_rate(mut self, risk_free_rate: f64) -> TickersBuilder {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn tickers_data(mut self, tickers_data: Option<Vec<KLINE>>) -> TickersBuilder {
        self.tickers_data = tickers_data;
        self
    }

    pub fn benchmark_data(mut self, benchmark_data: Option<KLINE>) -> TickersBuilder {
        self.benchmark_data = benchmark_data;
        self
    }

    pub fn build(self) -> Tickers {
        let benchmark_ticker = if let Some(benchmark_data) = self.benchmark_data.clone() {
            Ticker::builder()
                .ticker_data(Some(benchmark_data.clone()))
                .confidence_level(self.confidence_level)
                .risk_free_rate(self.risk_free_rate)
                .build()
        } else {
            Ticker::builder()
                .ticker(&self.benchmark_symbol)
                .start_date(&self.start_date)
                .end_date(&self.end_date)
                .interval(self.interval)
                .confidence_level(self.confidence_level)
                .risk_free_rate(self.risk_free_rate)
                .build()
        };


        let tickers = if let Some(tickers_data) = self.tickers_data.clone() {
            tickers_data.clone().into_iter().map(|x|
                Ticker::builder()
                    .ticker_data(Some(x.clone()))
                    .benchmark_data(benchmark_ticker.ticker_data.clone())
                    .confidence_level(self.confidence_level)
                    .risk_free_rate(self.risk_free_rate)
                    .build()
            ).collect::<Vec<Ticker>>()
        } else {
            self.tickers.clone().into_iter().map(|x|
                Ticker::builder().ticker(&x)
                    .start_date(&self.start_date)
                    .end_date(&self.end_date)
                    .interval(self.interval)
                    .benchmark_symbol(&self.benchmark_symbol)
                    .confidence_level(self.confidence_level)
                    .risk_free_rate(self.risk_free_rate)
                    .build()
            ).collect::<Vec<Ticker>>()
        };

        Tickers {
            tickers: tickers.clone(),
            start_date: tickers[0].start_date.clone(),
            end_date: tickers[0].end_date.clone(),
            interval: tickers[0].interval,
            benchmark_symbol: benchmark_ticker.ticker.clone(),
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            tickers_data: self.tickers_data,
            benchmark_data: self.benchmark_data,
            benchmark_ticker
        }
    }

}


impl Tickers {
    pub fn builder() -> TickersBuilder {
        TickersBuilder::new()
    }
    
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
    /// - `weights` - The Portfolio Weights (If weights are provided, they will be used instead of the optimization algorithm)
    ///
    /// ### Returns
    ///
    /// - A `Portfolio` Struct
    pub async fn optimize(&self,
                          objective_function: Option<ObjectiveFunction>,
                          constraints: Option<Vec<(f64, f64)>>,
                          weights: Option<Vec<f64>>) -> Result<Portfolio, Box<dyn Error>> {
        let objective_function = objective_function.unwrap_or(ObjectiveFunction::MaxSharpe);
        let performance_stats = PortfolioPerformanceStats::performance_stats(
            self.clone(), self.benchmark_ticker.clone(), &self.start_date, &self.end_date,
            self.confidence_level, self.risk_free_rate, objective_function, constraints, weights).await?;
        Ok(Portfolio {
            tickers: self.clone(),
            performance_stats,
        })
    }
}

/// Tickers Struct
///
/// ### Description
/// - This is the main Interface for the `Finalytics` Library.
/// - It provides methods to:
///     - fetch data for multiple tickers asynchronously.
///     - compute performance statistics for multiple tickers and display HTML reports.
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
///    let tickers = Tickers::builder()
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
///    let ticker = tickers.clone().get_ticker("NVDA").await?;
///    ticker.report(Some(ReportType::Performance)).await?.show()?;
///    ticker.report(Some(ReportType::Financials)).await?.show()?;
///    ticker.report(Some(ReportType::Options)).await?.show()?;
///    ticker.report(Some(ReportType::News)).await?.show()?;
///
///    // Generate a Multiple Ticker Report
///    tickers.report(Some(ReportType::Performance)).await?.show()?;
///
///    // Perform a Portfolio Optimization
///    let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None, None).await?;
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
    pub tickers_data: Option<Vec<KLINE>>,
    pub benchmark_data: Option<KLINE>,
    pub benchmark_ticker: Ticker,
}
