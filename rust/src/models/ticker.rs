use crate::prelude::{Interval, KLINE};

pub struct TickerBuilder {
    ticker: String,
    start_date: String,
    end_date: String,
    interval: Interval,
    benchmark_symbol: Option<String>,
    confidence_level: f64,
    risk_free_rate: f64,
    ticker_data: Option<KLINE>,
    benchmark_data: Option<KLINE>,
}

impl Default for TickerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TickerBuilder {
    pub fn new() -> TickerBuilder {
        TickerBuilder {
            ticker: String::new(),
            start_date: String::new(),
            end_date: String::new(),
            interval: Interval::OneDay,
            benchmark_symbol: None,
            confidence_level: 0.95,
            risk_free_rate: 0.0,
            ticker_data: None,
            benchmark_data: None,
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
        self.benchmark_symbol = Some(benchmark_symbol.to_string());
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

    pub fn ticker_data(mut self, ticker_data: Option<KLINE>) -> TickerBuilder {
        self.ticker_data = ticker_data;
        self
    }

    pub fn benchmark_data(mut self, benchmark_data: Option<KLINE>) -> TickerBuilder {
        self.benchmark_data = benchmark_data;
        self
    }

    pub fn build(self) -> Ticker {
        let (ticker, start_date, end_date) = if let Some(ticker_data) = self.ticker_data.clone() {
            (
                ticker_data.ticker.clone(),
                ticker_data.start_date().clone(),
                ticker_data.end_date().clone(),
            )
        } else {
            (self.ticker.clone(), self.start_date, self.end_date)
        };

        // Resolve benchmark symbol and data.
        // If explicit benchmark_data is provided, use it.
        // Else if benchmark_symbol is set (and no custom ticker_data), use symbol for later fetch.
        // Otherwise, no benchmark.
        let (benchmark_symbol, benchmark_data): (Option<String>, Option<KLINE>) =
            if let Some(benchmark_data) = self.benchmark_data {
                (Some(benchmark_data.ticker.clone()), Some(benchmark_data))
            } else if let Some(ref sym) = self.benchmark_symbol {
                (Some(sym.clone()), None)
            } else {
                (None, None)
            };

        let benchmark_ticker = benchmark_symbol.as_ref().map(|sym| {
            Box::new(Ticker {
                ticker: sym.clone(),
                start_date: start_date.clone(),
                end_date: end_date.clone(),
                interval: self.interval,
                benchmark_symbol: None,
                confidence_level: self.confidence_level,
                risk_free_rate: self.risk_free_rate,
                ticker_data: benchmark_data.clone(),
                benchmark_data: None,
                benchmark_ticker: None,
            })
        });

        Ticker {
            ticker,
            start_date: start_date.clone(),
            end_date: end_date.clone(),
            interval: self.interval,
            benchmark_symbol,
            confidence_level: self.confidence_level,
            risk_free_rate: self.risk_free_rate,
            ticker_data: self.ticker_data,
            benchmark_data,
            benchmark_ticker,
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
/// let ticker = Ticker::builder().ticker("AAPL")
///                             .start_date("2023-01-01")
///                             .end_date("2023-12-31")
///                             .interval(Interval::OneDay)
///                             .benchmark_symbol("^GSPC")
///                             .confidence_level(0.95)
///                             .risk_free_rate(0.02)
///                             .build();
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
    pub benchmark_symbol: Option<String>,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub ticker_data: Option<KLINE>,
    pub benchmark_data: Option<KLINE>,
    pub benchmark_ticker: Option<Box<Ticker>>,
}

impl Ticker {
    pub fn builder() -> TickerBuilder {
        TickerBuilder::new()
    }
}
