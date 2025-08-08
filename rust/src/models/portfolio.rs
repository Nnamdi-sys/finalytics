use std::error::Error;
use crate::prelude::{Interval, Tickers, KLINE};
use crate::analytics::optimization::ObjectiveFunction;
use crate::analytics::performance::PortfolioPerformanceStats;

pub struct PortfolioBuilder {
    pub ticker_symbols: Vec<String>,
    pub benchmark_symbol: String,
    pub start_date: String,
    pub end_date: String,
    pub interval: Interval,
    pub confidence_level: f64,
    pub risk_free_rate: f64,
    pub objective_function: ObjectiveFunction,
    pub constraints: Option<Vec<(f64, f64)>>,
    pub weights: Option<Vec<f64>>,
    pub tickers_data: Option<Vec<KLINE>>,
    pub benchmark_data: Option<KLINE>,

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
            benchmark_symbol: String::new(),
            start_date: String::new(),
            end_date: String::new(),
            interval: Interval::OneDay,
            confidence_level: 0.95,
            risk_free_rate: 0.0,
            objective_function: ObjectiveFunction::MaxSharpe,
            constraints: None,
            weights: None,
            tickers_data: None,
            benchmark_data: None,
        }
    }

    pub fn ticker_symbols(mut self, ticker_symbols: Vec<&str>) -> PortfolioBuilder {
        self.ticker_symbols = ticker_symbols.iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn benchmark_symbol(mut self, benchmark_symbol: &str) -> PortfolioBuilder {
        self.benchmark_symbol = benchmark_symbol.to_string();
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

    pub fn objective_function(mut self, objective_function: ObjectiveFunction) -> PortfolioBuilder {
        self.objective_function = objective_function;
        self
    }

    pub fn constraints(mut self, constraints: Option<Vec<(f64, f64)>>) -> PortfolioBuilder {
        self.constraints = constraints;
        self
    }

    pub fn weights(mut self, weights: Option<Vec<f64>>) -> PortfolioBuilder {
        self.weights = weights;
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

    pub async fn build(self) -> Result<Portfolio, Box<dyn Error>> {
        let tickers = if self.tickers_data.is_some() {
            Tickers::builder()
                    .tickers_data(self.tickers_data)
                    .benchmark_data(self.benchmark_data)
                    .confidence_level(self.confidence_level)
                    .risk_free_rate(self.risk_free_rate)
                    .build()
            } else {
            Tickers::builder()
                .tickers(self.ticker_symbols.iter().map(|x| x.as_str()).collect())
                .benchmark_symbol(&self.benchmark_symbol)
                .start_date(&self.start_date)
                .end_date(&self.end_date)
                .interval(self.interval)
                .confidence_level(self.confidence_level)
                .risk_free_rate(self.risk_free_rate)
                .build()
        };

        let performance_stats = PortfolioPerformanceStats::performance_stats(
            tickers.clone(), tickers.benchmark_ticker.clone(), &tickers.start_date, &tickers.end_date,
            tickers.confidence_level, tickers.risk_free_rate, self.objective_function, self.constraints.clone(), self.weights).await?;
        Ok(Portfolio {
            tickers,
            performance_stats,
        })
    }
}

/// # Portfolio Struct
///
/// ### Description
///    - This is the Portfolio Analysis Module for the `Finalytics` Library.
///    - It provides methods for Portfolio Optimization and Performance Analysis.
///
/// ### Constructor
///    - The Portfolio struct is created using the `PortfolioBuilder` struct.
///
/// ### Example
///
/// ```rust
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///    // Construct the Portfolio Object
///     let ticker_symbols = Vec::from(["NVDA", "BRK-A", "AAPL", "MSFT", "BTC-USD"]);
///     let constraints = Some(vec![(0.0, 1.0); ticker_symbols.len()]);
///     let portfolio = Portfolio::builder().ticker_symbols(ticker_symbols)
///                                             .benchmark_symbol("^GSPC")
///                                             .start_date("2023-01-01")
///                                             .end_date("2023-12-31")
///                                             .interval(Interval::OneDay)
///                                             .confidence_level(0.95)
///                                             .risk_free_rate(0.02)
///                                             .objective_function(ObjectiveFunction::MaxSharpe)
///                                             .constraints(constraints)
///                                             .build().await?;
///
///    // Display Portfolio Performance Report
///    portfolio.report(Some(ReportType::Performance)).await?.show()?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Portfolio{
    pub tickers: Tickers,
    pub performance_stats: PortfolioPerformanceStats,
}

impl Portfolio{
    pub fn builder() -> PortfolioBuilder {
        PortfolioBuilder::new()
    }
}


