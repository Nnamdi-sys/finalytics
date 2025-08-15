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
//! cargo add finalytics
//! ```
//!
//! ## Models
//! These are the main Interfaces for accessing the `finalytics` library methods
//! ##### [KLINE](https://docs.rs/finalytics/latest/finalytics/models/kline/struct.KLINE.html) - load historical price data from csv, json or dataframe
//! ##### [Ticker](https://docs.rs/finalytics/latest/finalytics/models/ticker/struct.Ticker.html) - Retrieve and analyze ticker data
//! ##### [Tickers](https://docs.rs/finalytics/latest/finalytics/models/tickers/struct.Tickers.html) - Retrieve and analyze multiple tickers
//! ##### [Portfolio](https://docs.rs/finalytics/latest/finalytics/models/portfolio/struct.Portfolio.html) - Optimize a portfolio of tickers
//! ##### [Screener](https://docs.rs/finalytics/latest/finalytics/models/screener/struct.Screener.html) - Screen for stocks, ETFs, indices, mutual funds, futures and cryptocurrencies
//!
//! ## Example
//!
//!
//! ```rust
//! use finalytics::prelude::*;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!         // Screen for the Top 20 Large-Cap NASDAQ Stocks
//!         let equity_screener = Screener::builder()
//!                  .quote_type(QuoteType::Equity)
//!                  .add_filter(ScreenerFilter::EqStr(
//!                      ScreenerMetric::Equity(EquityScreener::Exchange),
//!                      Exchange::NASDAQ.as_ref()
//!                  ))
//!                  .sort_by(
//!                      ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
//!                      true
//!                  )
//!                  .size(20)
//!                  .build()
//!                  .await?;
//!
//!         equity_screener.overview().show()?;
//!         equity_screener.metrics().await?.show()?;
//!
//!         // Instantiate a Multiple Ticker Object
//!         let ticker_symbols = equity_screener.symbols.iter()
//!             .map(|x| x.as_str()).collect::<Vec<&str>>();
//!
//!         let tickers = Tickers::builder()
//!             .tickers(ticker_symbols.clone())
//!             .start_date("2023-01-01")
//!             .end_date("2024-12-31")
//!             .interval(Interval::OneDay)
//!             .benchmark_symbol("^GSPC")
//!             .confidence_level(0.95)
//!             .risk_free_rate(0.02)
//!             .build();
//!
//!        // Generate a Single Ticker Report
//!         let symbol = ticker_symbols.first().unwrap();
//!         let ticker = tickers.clone().get_ticker(symbol).await?;
//!         ticker.report(Some(ReportType::Performance)).await?.show()?;
//!         ticker.report(Some(ReportType::Financials)).await?.show()?;
//!         ticker.report(Some(ReportType::Options)).await?.show()?;
//!         ticker.report(Some(ReportType::News)).await?.show()?;
//!
//!         // Generate a Multiple Ticker Report
//!         tickers.report(Some(ReportType::Performance)).await?.show()?;
//!
//!         // Perform a Portfolio Optimization
//!         let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None, None).await?;
//!
//!         // Generate a Portfolio Report
//!        portfolio.report(Some(ReportType::Performance)).await?.show()?;
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
    pub use crate::models::kline::KLINE;
    pub use crate::models::ticker::Ticker;
    pub use crate::models::tickers::Tickers;
    pub use crate::models::portfolio::Portfolio;
    pub use crate::models::screener::Screener;
    pub use crate::reports::table::DataTable;
    pub use crate::utils::date_utils::IntervalDays;
    pub use crate::analytics::optimization::Constraints;
    pub use crate::analytics::optimization::CategoricalWeights;


    // Enums
    pub use crate::data::yahoo::config::Interval;
    pub use crate::data::yahoo::config::StatementType;
    pub use crate::data::yahoo::config::StatementFrequency;
    pub use crate::analytics::technicals::Column;
    pub use crate::analytics::optimization::ObjectiveFunction;
    pub use crate::reports::table::DataTableFormat;
    pub use crate::reports::report::ReportType;
    
    // Traits
    pub use crate::data::ticker::TickerData;
    pub use crate::data::tickers::TickersData;
    pub use crate::charts::ticker::TickerCharts;
    pub use crate::charts::tickers::TickersCharts;
    pub use crate::charts::portfolio::PortfolioCharts;
    pub use crate::analytics::performance::TickerPerformance;
    pub use crate::analytics::stochastics::VolatilitySurface;
    pub use crate::analytics::technicals::TechnicalIndicators;
    pub use crate::reports::table::DataTableDisplay;
    pub use crate::reports::report::Report;
    #[cfg(feature = "kaleido")]
    pub use crate::utils::chart_utils::PlotImage;

    // Strum
    pub use strum::{EnumProperty,
                    VariantNames,
                    IntoEnumIterator,
                    VariantArray,
                    VariantIterator};

    // Screeners
    pub use crate::data::yahoo::screeners::{
        QuoteType,
        ScreenerBuilder,
        ScreenerFilter,
        ScreenerMetric,
        CryptoScreener,
        EquityScreener,
        EtfScreener,
        IndexScreener,
        MutualFundScreener,
        FutureScreener,
        Sector,
        Industry,
        Region,
        Exchange,
        PeerGroup,
        FundFamily,
        FundCategory
    };
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::error::Error;

    #[ignore]   
    #[tokio::test]
    async fn finalytics_test() -> Result<(), Box<dyn Error>> {
        // Screen for Large-Cap NASDAQ Stocks
        let equity_screener = Screener::builder()
                 .quote_type(QuoteType::Equity)
                 .add_filter(ScreenerFilter::EqStr(
                     ScreenerMetric::Equity(EquityScreener::Exchange),
                     Exchange::NASDAQ.as_ref()
                 ))
                 .sort_by(
                     ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
                     true
                 )
                 .size(10)
                 .build()
                 .await?;

        equity_screener.overview().show()?;
        equity_screener.metrics().await?.show()?;

        // Instantiate a Multiple Ticker Object
        let ticker_symbols = equity_screener.symbols.iter()
            .map(|x| x.as_str()).collect::<Vec<&str>>();

        let tickers = Tickers::builder()
            .tickers(ticker_symbols.clone())
            .start_date("2023-01-01")
            .end_date("2024-12-31")
            .interval(Interval::OneDay)
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .build();

       // Generate a Single Ticker Report
        let symbol = ticker_symbols.first().unwrap();
        let ticker = tickers.clone().get_ticker(symbol).await?;
        ticker.report(Some(ReportType::Performance)).await?.show()?;
        ticker.report(Some(ReportType::Financials)).await?.show()?;
        ticker.report(Some(ReportType::Options)).await?.show()?;
        ticker.report(Some(ReportType::News)).await?.show()?;

        // Generate a Multiple Ticker Report
        tickers.report(Some(ReportType::Performance)).await?.show()?;

        // Perform a Portfolio Optimization
        let portfolio = tickers.optimize(Some(ObjectiveFunction::MaxSharpe), None, None).await?;

        // Generate a Portfolio Report
        portfolio.report(Some(ReportType::Performance)).await?.show()?;

        Ok(())
    }
}

