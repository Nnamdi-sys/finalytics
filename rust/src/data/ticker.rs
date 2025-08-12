use polars::prelude::*;
use std::error::Error;
use crate::data::yahoo;
use crate::data::google;
use crate::models::ticker::Ticker;
use crate::data::yahoo::config::{Options, Quote, StatementFrequency, StatementType, TickerSummaryStats};

pub trait TickerData {
    fn get_quote(&self) -> impl std::future::Future<Output = Result<Quote, Box<dyn Error>>>;
    fn get_ticker_stats(&self) -> impl std::future::Future<Output = Result<TickerSummaryStats, Box<dyn Error>>>;
    fn get_chart(&self) -> impl std::future::Future<Output =  Result<DataFrame, Box<dyn Error>>>;
    fn get_options(&self) -> impl std::future::Future<Output = Result<Options, Box<dyn Error>>>;
    fn get_financials(&self, statement_type: StatementType, frequency: StatementFrequency, formatted: Option<bool>) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
    fn get_news(&self) -> impl std::future::Future<Output = Result<DataFrame, Box<dyn Error>>>;
}

impl TickerData for Ticker {
    /// Fetches Current Ticker Price from Yahoo Finance
    async fn get_quote(&self) -> Result<Quote, Box<dyn Error>> {
        yahoo::api::get_quote(&self.ticker).await
    }

    /// Fetches Ticker Current Summary Stats from Yahoo Finance
    async fn get_ticker_stats(&self) -> Result<TickerSummaryStats, Box<dyn Error>> {
        yahoo::api::get_ticker_stats(&self.ticker).await
    }


    /// Returns the Ticker OHLCV Data from given source
    async fn get_chart(&self) -> Result<DataFrame, Box<dyn Error>> {
        if let Some(ticker_data) = &self.ticker_data {
            ticker_data.clone().to_dataframe()
        } else {
            yahoo::api::get_chart(&self.ticker, &self.start_date, &self.end_date, self.interval).await
        }
    }


    /// Returns Ticker Option Chain Data from Yahoo Finance for all available expirations
    async fn get_options(&self) -> Result<Options, Box<dyn Error>> {
        yahoo::api::get_options(&self.ticker).await
    }


    /// Returns Ticker Financials from Yahoo Finance for a given statement type and frequency
    async fn get_financials(
        &self,
        statement_type: StatementType,
        frequency: StatementFrequency,
        formatted: Option<bool>
    ) -> Result<DataFrame, Box<dyn Error>> {
        yahoo::api::get_financials(&self.ticker, statement_type, frequency, formatted).await
    }

    /// Returns Ticker News from Google Web Search
    async fn get_news(&self) -> Result<DataFrame, Box<dyn Error>> {
        google::api::get_news(&self.ticker, &self.start_date, &self.end_date).await
    }
}