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
//! ##### [Portfolio](https://docs.rs/finalytics/latest/finalytics/models/portfolio/struct.Portfolio.html) - Optimize and analyze a portfolio of tickers
//! ##### [Screener](https://docs.rs/finalytics/latest/finalytics/models/screener/struct.Screener.html) - Screen for stocks, ETFs, indices, mutual funds, futures and cryptocurrencies
//!
//! ## Example
//!
//! ```rust,no_run
//! use finalytics::prelude::*;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // 1. Screener — Large-Cap NASDAQ Technology Stocks with ROE >= 15%
//!     let screener = Screener::builder()
//!         .quote_type(QuoteType::Equity)
//!         .add_filter(ScreenerFilter::EqStr(
//!             ScreenerMetric::Equity(EquityScreener::Exchange),
//!             Exchange::NASDAQ.as_ref(),
//!         ))
//!         .add_filter(ScreenerFilter::EqStr(
//!             ScreenerMetric::Equity(EquityScreener::Sector),
//!             Sector::Technology.as_ref(),
//!         ))
//!         .add_filter(ScreenerFilter::Gte(
//!             ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
//!             10_000_000_000.0,
//!         ))
//!         .add_filter(ScreenerFilter::Gte(
//!             ScreenerMetric::Equity(EquityScreener::ReturnOnEquity),
//!             0.15,
//!         ))
//!         .sort_by(
//!             ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
//!             true,
//!         )
//!         .size(10)
//!         .build()
//!         .await?;
//!
//!     screener.overview().show()?;
//!     screener.metrics().await?.show()?;
//!
//!     // 2. Ticker
//!     let ticker = Ticker::builder()
//!         .ticker("AAPL")
//!         .start_date("2023-01-01")
//!         .end_date("2024-12-31")
//!         .interval(Interval::OneDay)
//!         .benchmark_symbol("^GSPC")
//!         .confidence_level(0.95)
//!         .risk_free_rate(0.02)
//!         .build();
//!
//!     ticker.report(Some(ReportType::Performance)).await?.show()?;
//!     ticker.report(Some(ReportType::Financials)).await?.show()?;
//!     ticker.report(Some(ReportType::Options)).await?.show()?;
//!     ticker.report(Some(ReportType::News)).await?.show()?;
//!
//!     // 3. Tickers
//!     let tickers = Tickers::builder()
//!         .tickers(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
//!         .start_date("2023-01-01")
//!         .end_date("2024-12-31")
//!         .interval(Interval::OneDay)
//!         .benchmark_symbol("^GSPC")
//!         .confidence_level(0.95)
//!         .risk_free_rate(0.02)
//!         .build();
//!
//!     tickers.report(Some(ReportType::Performance)).await?.show()?;
//!
//!     // 4. Portfolio — Optimization with Out-of-Sample Evaluation
//!     let mut portfolio = Portfolio::builder()
//!         .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
//!         .benchmark_symbol("^GSPC")
//!         .start_date("2022-01-01")
//!         .end_date("2024-12-31")
//!         .interval(Interval::OneDay)
//!         .confidence_level(0.95)
//!         .risk_free_rate(0.02)
//!         .objective_function(ObjectiveFunction::MaxSharpe)
//!         .build().await?;
//!
//!     portfolio.optimize()?;
//!     portfolio.report(Some(ReportType::Optimization)).await?.show()?;
//!
//!     portfolio.update_dates("2025-01-01", "2026-01-01").await?;
//!     portfolio.performance_stats()?;
//!     portfolio.report(Some(ReportType::Performance)).await?.show()?;
//!
//!     Ok(())
//! }
//! ```

pub mod analytics;
pub mod charts;
pub mod data;
pub mod error;
pub mod models;
pub mod reports;
pub mod utils;

pub mod prelude {

    // ── Error Handling ──────────────────────────────────────────────────
    pub use crate::error::FinalyticsError;
    pub use crate::error::{
        column_to_vec_f64, require_min_length, series_to_optional_vec_f64, series_to_vec_f64,
    };

    // ── Core Models ─────────────────────────────────────────────────────
    // The primary types users instantiate and interact with.
    pub use crate::models::kline::KLINE;
    pub use crate::models::portfolio::Portfolio;
    pub use crate::models::screener::Screener;
    pub use crate::models::ticker::Ticker;
    pub use crate::models::tickers::Tickers;

    // ── Ticker Traits ───────────────────────────────────────────────────
    // Capabilities available on Ticker / Tickers.
    pub use crate::analytics::performance::TickerPerformance;
    pub use crate::analytics::stochastics::VolatilitySurface;
    pub use crate::analytics::technicals::TechnicalIndicators;
    pub use crate::charts::ticker::TickerCharts;
    pub use crate::charts::tickers::TickersCharts;
    pub use crate::data::ticker::TickerData;
    pub use crate::data::tickers::TickersData;

    // ── Portfolio Traits ────────────────────────────────────────────────
    // Capabilities available on Portfolio.
    pub use crate::charts::portfolio::PortfolioCharts;

    // ── Portfolio Configuration ─────────────────────────────────────────
    // Builder inputs for portfolio construction, optimization, rebalancing,
    // and cash flow scheduling.
    pub use crate::analytics::optimization::CategoricalWeights;
    pub use crate::analytics::optimization::Constraints;
    pub use crate::analytics::optimization::ObjectiveFunction;
    pub use crate::models::portfolio::CashFlowAllocation;
    pub use crate::models::portfolio::RebalanceStrategy;
    pub use crate::models::portfolio::ScheduleFrequency;
    pub use crate::models::portfolio::ScheduledCashFlow;
    pub use crate::models::portfolio::Transaction;

    // ── Portfolio Results ────────────────────────────────────────────────
    // Types returned by optimization, performance computation, and
    // simulation (rebalancing / cash flow tracking).
    pub use crate::analytics::performance::PortfolioData;
    pub use crate::analytics::performance::PortfolioOptimizationResult;
    pub use crate::analytics::performance::PortfolioPerformanceStats;
    pub use crate::analytics::statistics::DatedCashFlow;
    pub use crate::analytics::statistics::RebalanceConfig;
    pub use crate::analytics::statistics::RebalanceEvent;
    pub use crate::analytics::statistics::TransactionEvent;
    pub use crate::analytics::statistics::TransactionEventType;

    // ── Performance & Statistics ─────────────────────────────────────────
    // Enums and helpers for time-series analysis and return metrics.
    pub use crate::analytics::statistics::PerformancePeriod;
    pub use crate::analytics::statistics::ReturnsFrequency;
    pub use crate::analytics::statistics::ShrinkageMethod;
    pub use crate::analytics::statistics::ShrunkCovariance;
    pub use crate::utils::date_utils::IntervalDays;

    // ── Data & Configuration ────────────────────────────────────────────
    // Interval, frequency, and column selectors for data fetching.
    pub use crate::analytics::technicals::Column;
    pub use crate::data::yahoo::config::Interval;
    pub use crate::data::yahoo::config::StatementFrequency;
    pub use crate::data::yahoo::config::StatementType;

    // ── Reports & Display ───────────────────────────────────────────────
    // Report generation, table rendering, and display helpers.
    pub use crate::reports::report::Report;
    pub use crate::reports::report::ReportType;
    pub use crate::reports::table::DataTable;
    pub use crate::reports::table::DataTableDisplay;
    pub use crate::reports::table::DataTableFormat;

    // ── Screeners ───────────────────────────────────────────────────────
    // Yahoo Finance screener builders, filters, and category types.
    pub use crate::data::yahoo::screeners::{
        CryptoScreener, EquityScreener, EtfScreener, Exchange, FundCategory, FundFamily,
        FutureScreener, IndexScreener, Industry, MutualFundScreener, PeerGroup, QuoteType, Region,
        ScreenerBuilder, ScreenerFilter, ScreenerMetric, Sector,
    };

    // ── External Re-exports ─────────────────────────────────────────────
    // Strum derive helpers used by the public enums.
    pub use strum::{EnumProperty, IntoEnumIterator, VariantArray, VariantIterator, VariantNames};

    // ── Conditional ─────────────────────────────────────────────────────
    #[cfg(feature = "kaleido")]
    pub use crate::utils::chart_utils::PlotImage;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::error::Error;

    // ── 1. Screener — Large-Cap NASDAQ Technology Stocks with ROE >= 15% ────

    #[tokio::test]
    async fn test_screener() -> Result<(), Box<dyn Error>> {
        let screener = Screener::builder()
            .quote_type(QuoteType::Equity)
            .add_filter(ScreenerFilter::EqStr(
                ScreenerMetric::Equity(EquityScreener::Exchange),
                Exchange::NASDAQ.as_ref(),
            ))
            .add_filter(ScreenerFilter::EqStr(
                ScreenerMetric::Equity(EquityScreener::Sector),
                Sector::Technology.as_ref(),
            ))
            .add_filter(ScreenerFilter::Gte(
                ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
                10_000_000_000.0,
            ))
            .add_filter(ScreenerFilter::Gte(
                ScreenerMetric::Equity(EquityScreener::ReturnOnEquity),
                0.15,
            ))
            .sort_by(
                ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
                true,
            )
            .size(10)
            .build()
            .await?;

        screener.overview().show()?;
        screener.metrics().await?.show()?;

        Ok(())
    }

    // ── 2. Ticker — Single security analysis with all report types ──────────

    #[tokio::test]
    async fn test_ticker() -> Result<(), Box<dyn Error>> {
        let ticker = Ticker::builder()
            .ticker("AAPL")
            .start_date("2023-01-01")
            .end_date("2024-12-31")
            .interval(Interval::OneDay)
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .build();

        ticker.report(Some(ReportType::Performance)).await?.show()?;
        ticker.report(Some(ReportType::Financials)).await?.show()?;
        ticker.report(Some(ReportType::Options)).await?.show()?;
        ticker.report(Some(ReportType::News)).await?.show()?;

        Ok(())
    }

    // ── 3. Tickers — Multiple securities analysis ───────────────────────────

    #[tokio::test]
    async fn test_tickers() -> Result<(), Box<dyn Error>> {
        let tickers = Tickers::builder()
            .tickers(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
            .start_date("2023-01-01")
            .end_date("2024-12-31")
            .interval(Interval::OneDay)
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .build();

        tickers
            .report(Some(ReportType::Performance))
            .await?
            .show()?;

        Ok(())
    }

    // ── 4. Portfolio — Optimization with Out-of-Sample Evaluation ────────────

    #[tokio::test]

    // Optimize on 2023-2024 data (in-sample)
    async fn test_portfolio_optimization_oos() -> Result<(), Box<dyn Error>> {
        let mut portfolio = Portfolio::builder()
            .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
            .benchmark_symbol("^GSPC")
            .start_date("2023-01-01")
            .end_date("2024-12-31")
            .interval(Interval::OneDay)
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .objective_function(ObjectiveFunction::MaxSharpe)
            .build()
            .await?;

        portfolio.optimize()?;
        portfolio
            .report(Some(ReportType::Optimization))
            .await?
            .show()?;

        // Update to 2025 data for out-of-sample evaluation
        portfolio.update_dates("2025-01-01", "2026-01-01").await?;
        portfolio.performance_stats()?;
        portfolio
            .report(Some(ReportType::Performance))
            .await?
            .show()?;

        Ok(())
    }

    // ── 5. Portfolio — Optimization with Weight & Categorical Constraints ─────
    //
    // Demonstrates how to cap individual asset allocations and enforce
    // sector / asset-class exposure limits simultaneously.
    //
    // Portfolio: AAPL, MSFT, NVDA (Tech · Equity)
    //            JPM              (Finance · Equity)
    //            XOM              (Energy · Equity)
    //            BTC-USD          (Crypto · Crypto)
    //
    // Asset-weight bounds  — every asset stays between its own min/max.
    // Sector constraints   — Tech ≤ 60 %, Finance ≤ 30 %, Energy ≤ 20 %, Crypto ≤ 25 %.
    // Asset-class bounds   — Equities 70–95 %, Crypto 5–30 %.

    #[tokio::test]
    async fn test_portfolio_optimization_constraints() -> Result<(), Box<dyn Error>> {
        let constraints = Constraints {
            // Per-asset (lower_bound, upper_bound) in the same order as ticker_symbols.
            asset_weights: Some(vec![
                (0.05, 0.40), // AAPL
                (0.05, 0.40), // MSFT
                (0.05, 0.40), // NVDA
                (0.05, 0.30), // JPM
                (0.05, 0.20), // XOM
                (0.05, 0.25), // BTC-USD
            ]),
            categorical_weights: Some(vec![
                // ── Sector constraint ──────────────────────────────────────
                CategoricalWeights {
                    name: "Sector".to_string(),
                    category_per_symbol: vec![
                        "Tech".to_string(),    // AAPL
                        "Tech".to_string(),    // MSFT
                        "Tech".to_string(),    // NVDA
                        "Finance".to_string(), // JPM
                        "Energy".to_string(),  // XOM
                        "Crypto".to_string(),  // BTC-USD
                    ],
                    // (category, lower_bound, upper_bound)
                    weight_per_category: vec![
                        ("Tech".to_string(), 0.30, 0.60),
                        ("Finance".to_string(), 0.05, 0.30),
                        ("Energy".to_string(), 0.05, 0.20),
                        ("Crypto".to_string(), 0.05, 0.25),
                    ],
                },
                // ── Asset-class constraint ─────────────────────────────────
                CategoricalWeights {
                    name: "Asset Class".to_string(),
                    category_per_symbol: vec![
                        "Equity".to_string(), // AAPL
                        "Equity".to_string(), // MSFT
                        "Equity".to_string(), // NVDA
                        "Equity".to_string(), // JPM
                        "Equity".to_string(), // XOM
                        "Crypto".to_string(), // BTC-USD
                    ],
                    weight_per_category: vec![
                        ("Equity".to_string(), 0.70, 0.95),
                        ("Crypto".to_string(), 0.05, 0.30),
                    ],
                },
            ]),
        };

        let mut portfolio = Portfolio::builder()
            .ticker_symbols(vec!["AAPL", "MSFT", "NVDA", "JPM", "XOM", "BTC-USD"])
            .benchmark_symbol("^GSPC")
            .start_date("2023-01-01")
            .end_date("2024-12-31")
            .interval(Interval::OneDay)
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .objective_function(ObjectiveFunction::MaxSharpe)
            .constraints(Some(constraints))
            .build()
            .await?;

        portfolio.optimize()?;
        portfolio
            .report(Some(ReportType::Optimization))
            .await?
            .show()?;

        Ok(())
    }

    // ── 6. Portfolio — Explicit Allocation with Rebalancing and DCA ──────────

    #[tokio::test]
    async fn test_portfolio_allocation_rebalancing_dca() -> Result<(), Box<dyn Error>> {
        let mut portfolio_alloc = Portfolio::builder()
            .ticker_symbols(vec!["AAPL", "MSFT", "NVDA", "BTC-USD"])
            .benchmark_symbol("^GSPC")
            .start_date("2023-01-01")
            .end_date("2024-12-31")
            .interval(Interval::OneDay)
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .weights(vec![25_000.0, 25_000.0, 25_000.0, 25_000.0])
            .rebalance_strategy(Some(RebalanceStrategy::Calendar(
                ScheduleFrequency::Quarterly,
            )))
            .scheduled_cash_flows(Some(vec![ScheduledCashFlow {
                amount: 2_000.0,
                frequency: ScheduleFrequency::Monthly,
                start_date: None,
                end_date: None,
                allocation: CashFlowAllocation::ProRata,
            }]))
            .build()
            .await?;

        portfolio_alloc.performance_stats()?;
        portfolio_alloc
            .report(Some(ReportType::Performance))
            .await?
            .show()?;

        Ok(())
    }

    // ── 6. Custom Data (KLINE) — Load CSV data and use with Ticker, Tickers, Portfolio ──

    #[tokio::test]
    async fn test_custom_data() -> Result<(), Box<dyn Error>> {
        // Load data from CSV files
        let aapl = KLINE::from_csv("AAPL", "../examples/datasets/aapl.csv")?;
        let msft = KLINE::from_csv("MSFT", "../examples/datasets/msft.csv")?;
        let nvda = KLINE::from_csv("NVDA", "../examples/datasets/nvda.csv")?;
        let goog = KLINE::from_csv("GOOG", "../examples/datasets/goog.csv")?;
        let btcusd = KLINE::from_csv("BTC-USD", "../examples/datasets/btcusd.csv")?;
        let gspc = KLINE::from_csv("^GSPC", "../examples/datasets/gspc.csv")?;

        // Single Ticker from custom data
        let custom_ticker = Ticker::builder()
            .ticker("AAPL")
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .ticker_data(Some(aapl.clone()))
            .benchmark_data(Some(gspc.clone()))
            .build();

        custom_ticker
            .report(Some(ReportType::Performance))
            .await?
            .show()?;

        // Multiple Tickers from custom data
        let custom_tickers = Tickers::builder()
            .tickers(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .tickers_data(Some(vec![
                nvda.clone(),
                goog.clone(),
                aapl.clone(),
                msft.clone(),
                btcusd.clone(),
            ]))
            .benchmark_data(Some(gspc.clone()))
            .build();

        custom_tickers
            .report(Some(ReportType::Performance))
            .await?
            .show()?;

        // Portfolio optimization from custom data
        let mut custom_portfolio = Portfolio::builder()
            .ticker_symbols(vec!["NVDA", "GOOG", "AAPL", "MSFT", "BTC-USD"])
            .benchmark_symbol("^GSPC")
            .confidence_level(0.95)
            .risk_free_rate(0.02)
            .objective_function(ObjectiveFunction::MaxSharpe)
            .tickers_data(Some(vec![nvda, goog, aapl, msft, btcusd]))
            .benchmark_data(Some(gspc))
            .build()
            .await?;

        custom_portfolio.optimize()?;
        custom_portfolio
            .report(Some(ReportType::Optimization))
            .await?
            .show()?;

        Ok(())
    }
}
