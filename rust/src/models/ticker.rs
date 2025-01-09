use crate::data::config::Interval;


pub struct TickerBuilder {
    ticker: String,
    start_date: String,
    end_date: String,
    interval: Interval,
    benchmark_symbol: String,
    confidence_level: f64,
    risk_free_rate: f64,
}

impl TickerBuilder {
    pub fn new() -> TickerBuilder {
        TickerBuilder {
            ticker: String::new(),
            start_date: String::new(),
            end_date: String::new(),
            interval: Interval::OneDay,
            benchmark_symbol: String::from("^GSPC"),
            confidence_level: 0.95,
            risk_free_rate: 0.02,
        }
    }

    pub fn ticker(mut self, symbol: &str) -> TickerBuilder {
        self.ticker = symbol.to_string();
        self
    }

    pub fn start_date(mut self, start_date: &str) -> TickerBuilder {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(mut self, end_date: &str) -> TickerBuilder {
        self.end_date = end_date.to_string();
        self
    }

    pub fn interval(mut self, interval: Interval) -> TickerBuilder {
        self.interval = interval;
        self
    }

    pub fn benchmark_symbol(mut self, benchmark_symbol: &str) -> TickerBuilder {
        self.benchmark_symbol = benchmark_symbol.to_string();
        self
    }

    pub fn confidence_level(mut self, confidence_level: f64) -> TickerBuilder {
        self.confidence_level = confidence_level;
        self
    }

    pub fn risk_free_rate(mut self, risk_free_rate: f64) -> TickerBuilder {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn build(self) -> Ticker {
        Ticker {
            ticker: self.ticker,
            start_date: self.start_date,
            end_date: self.end_date,
            interval: self.interval,
            benchmark_symbol: self.benchmark_symbol,
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
        }
    }
}


/// # Ticker Struct
///
/// ### Description
///    - This is the Security Analysis Module for the `Finalytics` Library.
///    - It provides methods to:
///      - Fetch Ticker Data from Yahoo Finance
///      - Perform Fundamental Analysis, Technical Analysis, Options Volatility Analysis and News Sentiment Analysis
///      - Display HTML Reports of Analytics Data, Charts and Results
///
/// ### Constructor
///    - The `Ticker` struct can be instantiated using the `TickerBuilder` struct.
///
/// ### Example
///
/// ```rust
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///
///  // Instantiate the Ticker Object
/// let ticker = TickerBuilder::new().ticker("AAPL")
///                                     .start_date("2023-01-01")
///                                     .end_date("2023-12-31")
///                                     .interval(Interval::OneDay)
///                                     .benchmark_symbol("^GSPC")
///                                     .confidence_level(0.95)
///                                     .risk_free_rate(0.02)
///                                     .build();
///
///  // Display Ticker Reports
///  ticker.report(Some(ReportType::Performance)).await?.show()?;
///  ticker.report(Some(ReportType::Financials)).await?.show()?;
///  ticker.report(Some(ReportType::Options)).await?.show()?;
///  ticker.report(Some(ReportType::News)).await?.show()?;
///
///
///  Ok(())
///
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Ticker {
    pub ticker: String,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub benchmark_symbol: String,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
}















