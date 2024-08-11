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
//! ```rust
//! use finalytics::prelude::*;
//! use polars::prelude::*;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!         // Instantiate a Multiple Ticker Object
//!         let ticker_symbols = Vec::from(["NVDA", "GOOG", "AAPL", "MSFT","BTC-USD"]);
//!
//!         let tickers = TickersBuilder::new()
//!             .tickers(ticker_symbols)
//!             .start_date("2020-01-01")
//!             .end_date("2024-01-01")
//!             .interval(Interval::OneDay)
//!             .benchmark_symbol("^GSPC")
//!             .confidence_level(0.95)
//!             .risk_free_rate(0.02)
//!             .build();
//!
//!         // Calculate the Performance Statistics
//!         let performance_stats = tickers.performance_stats().await?;
//!         println!("{:?}", performance_stats);
//!
//!         // Display the Security Analysis Charts
//!         tickers.returns_chart(None, None).await?.show();
//!         tickers.returns_matrix(None, None).await?.show();
//!
//!         // Perform a Portfolio Optimization
//!         let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None).await?;
//!         println!("{:?}", portfolio.performance_stats);
//!
//!         // Display the Portfolio Optimization Charts
//!         portfolio.performance_stats_table(None, None)?.show();
//!         portfolio.optimization_chart(None, None)?.show();
//!         portfolio.performance_chart(None, None)?.show();
//!         portfolio.asset_returns_chart(None, None)?.show();
//!
//!         Ok(())
//!     }
//! ```



pub mod models;
pub mod analytics;
pub mod charts;
pub mod utils;
pub mod data;


pub mod prelude {

    // Structs
    pub use crate::models::ticker::Ticker;
    pub use crate::models::tickers::Tickers;
    pub use crate::models::portfolio::Portfolio;


    // Enums
    pub use crate::data::config::Interval;
    pub use crate::analytics::technicals::Column;
    pub use crate::analytics::optimization::ObjectiveFunction;


    // Builders
    pub use crate::models::ticker::TickerBuilder;
    pub use crate::models::tickers::TickersBuilder;
    pub use crate::models::portfolio::PortfolioBuilder;


    // Traits
    pub use crate::data::ticker::TickerData;
    pub use crate::charts::frame::PolarsPlot;
    pub use crate::charts::ticker::TickerCharts;
    pub use crate::charts::portfolio::PortfolioCharts;
    pub use crate::analytics::fundamentals::Financials;
    pub use crate::analytics::performance::TickerPerformance;
    pub use crate::analytics::stochastics::VolatilitySurface;
    pub use crate::analytics::technicals::TechnicalIndicators;

    #[cfg(feature = "kaleido")]
    pub use crate::utils::chart_utils::PlotImage;

}
