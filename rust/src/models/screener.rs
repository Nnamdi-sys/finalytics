use std::error::Error;
use polars::frame::DataFrame;
use crate::data::yahoo::screeners::builder::ScreenerBuilder;
use crate::prelude::{DataTable, DataTableDisplay, DataTableFormat, QuoteType, Tickers, TickersData};


/// Screener Struct
///
/// ### Description
/// - Provides an interface for building and executing stock screeners using Yahoo Finance's API
/// - Supports multiple quote types (equities, ETFs, mutual funds, indices, cryptocurrencies)
/// - Allows filtering, sorting, and pagination of financial instruments
/// - Automatically handles API limitations by chunking large requests
///
/// ### Constructor
/// - Use `Screener::builder()` to create a `ScreenerBuilder` with chainable configuration methods
///
/// ### Example: Comprehensive Screening
/// ```rust
/// use std::error::Error;
/// use finalytics::prelude::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     // 1. Large-cap NASDAQ stocks
///     let equity_screener = Screener::builder()
///         .quote_type(QuoteType::Equity)
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::Equity(EquityScreener::Exchange),
///             Exchange::NASDAQ.as_ref()
///         ))
///         .sort_by(
///             ScreenerMetric::Equity(EquityScreener::MarketCapIntraday),
///             true
///         )
///         .size(100)
///         .build()
///         .await?;
///     equity_screener.overview().show()?;
///
///     // 2. Large-cap growth mutual funds
///     let mutual_fund_screener = Screener::builder()
///         .quote_type(QuoteType::MutualFund)
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::MutualFund(MutualFundScreener::FundsByCategory),
///             FundCategory::LargeGrowth.as_ref()
///         ))
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::MutualFund(MutualFundScreener::Region),
///             Region::UnitedStates.as_ref()
///         ))
///         .sort_by(
///             ScreenerMetric::MutualFund(MutualFundScreener::TrailingYTDReturn),
///             true
///         )
///         .size(100)
///         .build()
///         .await?;
///     mutual_fund_screener.overview().show()?;
///
///     // 3. Technology sector ETFs
///     let etf_screener = Screener::builder()
///         .quote_type(QuoteType::Etf)
///         .add_filter(ScreenerFilter::Gte(
///             ScreenerMetric::Etf(EtfScreener::FundNetAssets),
///             1_000_000_000.0
///         ))
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::Etf(EtfScreener::FundsByCategory),
///             FundCategory::Technology.as_ref()
///         ))
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::Etf(EtfScreener::Region),
///             Region::UnitedStates.as_ref()
///         ))
///         .sort_by(
///             ScreenerMetric::Etf(EtfScreener::TrailingYTDReturn),
///             true
///         )
///         .size(100)
///         .build()
///         .await?;
///     etf_screener.overview().show()?;
///
///     // 4. High-volume market indices
///     let index_screener = Screener::builder()
///         .quote_type(QuoteType::Index)
///         .add_filter(ScreenerFilter::Gte(
///             ScreenerMetric::Index(IndexScreener::AvgVol3Month),
///             1_000_000_000.0
///         ))
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::Index(IndexScreener::Region),
///             Region::UnitedStates.as_ref()
///         ))
///         .sort_by(
///             ScreenerMetric::Index(IndexScreener::Week52PricePercentChange),
///             true
///         )
///         .size(100)
///         .build()
///         .await?;
///     index_screener.overview().show()?;
///
///     // 5. Top 100 USD-quoted cryptocurrencies
///     let crypto_screener = Screener::builder()
///         .quote_type(QuoteType::Crypto)
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::Crypto(CryptoScreener::Exchange),
///             Exchange::Cryptocurrencies.as_ref()
///         ))
///         .add_filter(ScreenerFilter::EqStr(
///             ScreenerMetric::Crypto(CryptoScreener::Currency),
///             "USD"
///         ))
///         .sort_by(
///             ScreenerMetric::Crypto(CryptoScreener::MarketCapIntraday),
///             true
///         )
///         .size(100)
///         .build()
///         .await?;
///     crypto_screener.overview().show()?;
///
///     Ok(())
/// }
/// ```
pub struct Screener {
    pub quote_type: QuoteType,
    pub symbols: Vec<String>,
    pub result: DataFrame
}

impl Screener {
    pub fn builder() -> ScreenerBuilder {
        ScreenerBuilder::default()
    }
    pub fn overview(&self) -> DataTable {
        self.result.to_datatable("screener_overview", true, DataTableFormat::Number)
    }
    pub async fn metrics(&self) -> Result<DataTable, Box<dyn Error>> {
        let symbols = self.symbols.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
        let metrics = Tickers::builder().tickers(symbols).build().get_ticker_stats().await?;
        Ok(metrics.to_datatable("screener_metrics", true, DataTableFormat::Number))
    }
}