//! # Finalytics
//!
//! Welcome to `finalytics`, financial analytics in Rust!
//!
//! `finalytics` is a Rust library designed for retrieving financial data and performing security analysis and portfolio optimization.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! finalytics = "*"
//! ```
//!
//! Or run the following command:
//!
//! ```bash
//! cargo install finalytics
//! ```
//!
//! ## Models
//! These are the main Interfaces for accessing the `finalytics` library methods
//! ##### [Ticker](https://docs.rs/finalytics/latest/finalytics/models/ticker/struct.Ticker.html) - Retrieve and analyze ticker data
//! ##### [Tickers](https://docs.rs/finalytics/latest/finalytics/models/tickers/struct.Tickers.html) - Retrieve and analyze multiple tickers
//! ##### [Portfolio](https://docs.rs/finalytics/latest/finalytics/models/portfolio/struct.Portfolio.html) - Optimize a portfolio of tickers
//!
//! ## Example
//!
//!
//! ```no_run
//! use finalytics::prelude::*;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!         // Instantiate a Multiple Ticker Object
//!         let ticker_symbols = Vec::from(["NVDA", "GOOG", "AAPL", "MSFT","BTC-USD"]);
//!
//!         let tickers = TickersBuilder::new()
//!             .tickers(ticker_symbols)
//!             .start_date("2023-01-01")
//!             .end_date("2024-12-31")
//!             .interval(Interval::OneDay)
//!             .benchmark_symbol("^GSPC")
//!             .confidence_level(0.95)
//!             .risk_free_rate(0.02)
//!             .build();
//!
//!        // Generate a Single Ticker Report
//!         let ticker = tickers.clone().get_ticker("AAPL").await?;
//!         ticker.report(Some(ReportType::Performance)).await?.show()?;
//!         ticker.report(Some(ReportType::Financials)).await?.show()?;
//!         ticker.report(Some(ReportType::Options)).await?.show()?;
//!
//!         // Generate a Multiple Ticker Report
//!         tickers.report(Some(ReportType::Performance)).await?.show()?;
//!
//!         // Perform a Portfolio Optimization
//!         let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None).await?;
//!
//!         // Generate a Portfolio Report
//!         portfolio.report(Some(ReportType::Performance)).await?.show()?;
//!
//!         Ok(())
//!     }
//! ```



pub mod models;
pub mod analytics;
pub mod charts;
pub mod utils;
pub mod data;
pub mod reports;

pub mod prelude {

    // Structs
    pub use crate::models::ticker::Ticker;
    pub use crate::models::tickers::Tickers;
    pub use crate::models::portfolio::Portfolio;
    pub use crate::reports::table::DataTable;


    // Enums
    pub use crate::data::config::Interval;
    pub use crate::data::config::StatementType;
    pub use crate::data::config::StatementFrequency;
    pub use crate::analytics::technicals::Column;
    pub use crate::analytics::optimization::ObjectiveFunction;
    pub use crate::reports::table::TableType;
    pub use crate::reports::report::ReportType;


    // Builders
    pub use crate::models::ticker::TickerBuilder;
    pub use crate::models::tickers::TickersBuilder;
    pub use crate::models::portfolio::PortfolioBuilder;


    // Traits
    pub use crate::data::ticker::TickerData;
    pub use crate::data::tickers::TickersData;
    pub use crate::charts::ticker::TickerCharts;
    pub use crate::charts::tickers::TickersCharts;
    pub use crate::charts::portfolio::PortfolioCharts;
    pub use crate::analytics::fundamentals::Financials;
    pub use crate::analytics::performance::TickerPerformance;
    pub use crate::analytics::stochastics::VolatilitySurface;
    pub use crate::analytics::technicals::TechnicalIndicators;
    pub use crate::reports::report::Report;

    // Utils
    #[cfg(feature = "kaleido")]
    pub use crate::utils::chart_utils::PlotImage;

}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::error::Error;

    #[ignore]
    #[tokio::test]
    async fn test_finalytics_reports() -> Result<(), Box<dyn Error>> {
        // Instantiate a Multiple Ticker Object
        let ticker_symbols = vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"];

        let tickers = TickersBuilder::new()
            .tickers(ticker_symbols.clone())
            .start_date("2023-01-01")
            .end_date("2024-12-31")
            .interval(Interval::OneDay)
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .build();

        // Generate a Single Ticker Report
        let ticker = tickers.clone().get_ticker("AAPL").await?;
        ticker.report(Some(ReportType::Performance)).await?.show()?;
        ticker.report(Some(ReportType::Financials)).await?.show()?;
        ticker.report(Some(ReportType::Options)).await?.show()?;
        ticker.report(Some(ReportType::News)).await?.show()?;

        // Generate a Multiple Ticker Report
        tickers.report(Some(ReportType::Performance)).await?.show()?;

        // Perform a Portfolio Optimization
        let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None).await?;

        // Generate a Portfolio Report
        portfolio.report(Some(ReportType::Performance)).await?.show()?;

        Ok(())
    }
}

