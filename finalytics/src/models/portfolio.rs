use std::error::Error;
use crate::data::config::Interval;
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
    pub max_iterations: u64,
    pub objective_function: ObjectiveFunction,
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
            risk_free_rate: 0.02,
            max_iterations: 1000,
            objective_function: ObjectiveFunction::MaxSharpe,
        }
    }

    pub fn ticker_symbols(&mut self, ticker_symbols: Vec<&str>) -> &mut PortfolioBuilder {
        self.ticker_symbols = ticker_symbols.iter().map(|x| x.to_string()).collect();
        self
    }

    pub fn benchmark_symbol(&mut self, benchmark_symbol: &str) -> &mut PortfolioBuilder {
        self.benchmark_symbol = benchmark_symbol.to_string();
        self
    }

    pub fn start_date(&mut self, start_date: &str) -> &mut PortfolioBuilder {
        self.start_date = start_date.to_string();
        self
    }

    pub fn end_date(&mut self, end_date: &str) -> &mut PortfolioBuilder {
        self.end_date = end_date.to_string();
        self
    }

    pub fn interval(&mut self, interval: Interval) -> &mut PortfolioBuilder {
        self.interval = interval;
        self
    }

    pub fn confidence_level(&mut self, confidence_level: f64) -> &mut PortfolioBuilder {
        self.confidence_level = confidence_level;
        self
    }

    pub fn risk_free_rate(&mut self, risk_free_rate: f64) -> &mut PortfolioBuilder {
        self.risk_free_rate = risk_free_rate;
        self
    }

    pub fn max_iterations(&mut self, max_iterations: u64) -> &mut PortfolioBuilder {
        self.max_iterations = max_iterations;
        self
    }

    pub fn objective_function(&mut self, objective_function: ObjectiveFunction) -> &mut PortfolioBuilder {
        self.objective_function = objective_function;
        self
    }

    pub async fn build(&mut self) -> Result<Portfolio, Box<dyn Error>> {
        let performance_stats = PortfolioPerformanceStats::new(
            self.ticker_symbols.clone(), &self.benchmark_symbol, &self.start_date, &self.end_date, self.interval,
            self.confidence_level, self.risk_free_rate, self.max_iterations, self.objective_function).await?.compute_stats()?;
        Ok(Portfolio {
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
/// ```
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///    // Construct the Portfolio Object
///     let ticker_symbols = Vec::from(["NVDA", "BRK-A", "AAPL", "MSFT", "BTC-USD"]);
///     let portfolio = PortfolioBuilder::new().ticker_symbols(ticker_symbols)
///                                             .benchmark_symbol("^GSPC")
///                                             .start_date("2023-01-01")
///                                             .end_date("2023-12-31")
///                                             .interval(Interval::OneDay)
///                                             .confidence_level(0.95)
///                                             .risk_free_rate(0.02)
///                                             .max_iterations(1000)
///                                             .objective_function(ObjectiveFunction::MaxSharpe)
///                                             .build().await?;
///
///     // Display Portfolio Optimization Results
///     println!("{:?}", portfolio.performance_stats);
///
///    // Display Portfolio Analytics Charts
///     portfolio.optimization_chart(800, 1200)?.show();
///     portfolio.performance_chart(800, 1200)?.show();
///     portfolio.asset_returns_chart(800, 1200)?.show();
///     portfolio.performance_stats_table(800, 1200)?.show();
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Portfolio{
    pub performance_stats: PortfolioPerformanceStats,
}



