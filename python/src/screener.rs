use std::str::FromStr;
use pyo3::prelude::*;
use tokio::task;
use finalytics::data::yahoo::screeners::builder::ScreenerBuilder;
use finalytics::prelude::{CryptoScreener, EquityScreener, EtfScreener, FutureScreener, IndexScreener,
                          MutualFundScreener, QuoteType, Screener, ScreenerFilter, ScreenerMetric};
use crate::ffi::rust_df_to_py_df;


/// Create a new Screener object
///
/// # Arguments
///
/// * `quote_type` - `str` - The type of financial instrument to screen ("EQUITY", "MUTUALFUND", "ETF", "INDEX", "FUTURE", "CRYPTO")
/// * `filters` - `List[str]` - A list of JSON strings specifying filter criteria
/// * `sort_field` - `str` - The metric to sort by
/// * `sort_descending` - `bool` - Whether to sort in descending order
/// * `offset` - `int` - The starting index of results to return
/// * `size` - `int` - The maximum number of results to return
///
/// # Returns
///
/// `Screener` - A Screener object
///
/// # Example
///
/// ```
/// from finalytics import Screener
///
/// screener = Screener(
///     quote_type="EQUITY",
///     filters=[
///         '{"operator": "eq", "operands": ["exchange", "NMS"]}'
///     ],
///     sort_field="intradaymarketcap",
///     sort_descending=True,
///     offset=0,
///     size=100
/// )
/// print(screener.overview())
/// print(screener.metrics())
/// ```
/// 
/// # Note
/// The `filters` parameter should be a list of JSON strings, each representing a filter condition. The format for each filter is:
///  ```json
/// {
///      "operator": "<op>",
///      "operands": ["<metric>", <value>[, <value2>]]
///  }
/// 
/// where `<op>` can be "eq", "gte", "lte", "gt", "lt", or "btwn". 
/// 
/// The `<metric>` should be a valid screener metric.
/// A full list of screener metrics for each category can be found at:
/// https://github.com/Nnamdi-sys/finalytics/tree/main/rust/src/data/yahoo/screeners/screeners.json
#[allow(unused)]
#[pyclass]
#[pyo3(name = "Screener")]
pub struct PyScreener {
    screener: Screener
}

#[pymethods]
impl PyScreener {
    #[new]
    #[pyo3(signature = (quote_type, filters, sort_field=None, sort_descending=true, offset=0, size=250))]
    pub fn new(quote_type: &str, filters: Vec<String>, sort_field: Option<String>, 
               sort_descending: bool, offset: usize, size: usize) -> Self {
        let quote_type = QuoteType::from_str(quote_type).unwrap();
        let filters = filters.into_iter()
            .map(ScreenerFilter::Custom)
            .collect::<Vec<_>>();
        let sort_field = sort_field.map(|f| {
            match quote_type {
                QuoteType::Equity => ScreenerMetric::Equity(EquityScreener::from_str(&f).unwrap()),
                QuoteType::MutualFund => ScreenerMetric::MutualFund(MutualFundScreener::from_str(&f).unwrap()),
                QuoteType::Etf => ScreenerMetric::Etf(EtfScreener::from_str(&f).unwrap()),
                QuoteType::Index => ScreenerMetric::Index(IndexScreener::from_str(&f).unwrap()),
                QuoteType::Future => ScreenerMetric::Future(FutureScreener::from_str(&f).unwrap()),
                QuoteType::Crypto => ScreenerMetric::Crypto(CryptoScreener::from_str(&f).unwrap()),
            }
        });
        let screener_builder = ScreenerBuilder {
            quote_type: Some(quote_type),
            filters,
            sort_field,
            sort_descending,
            offset,
            size,   
        };
        
        task::block_in_place(move || {
            let screener = tokio::runtime::Runtime::new().unwrap().block_on(
                screener_builder.build()).unwrap();
            PyScreener {
                screener
            }
        })
    }

    /// Get the list of ticker symbols matching the screener criteria
    ///
    /// # Returns
    ///
    /// `List[str]` - A list of ticker symbols (e.g., ["AAPL", "MSFT", "GOOGL"])
    pub fn symbols(&self) -> Vec<String> {
        self.screener.symbols.clone()
    }

    /// Get a Polars DataFrame containing the overview of screened instruments
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame with overview data
    pub fn overview(&self) -> PyObject {
        let overview = &self.screener.result;
        rust_df_to_py_df(overview).unwrap()
    }

    /// Get a Polars DataFrame containing detailed metrics for screened instruments
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame with detailed metrics
    pub fn metrics(&self) -> PyObject {
        let metrics = tokio::runtime::Runtime::new().unwrap().block_on(self.screener.metrics()).unwrap().data;
        rust_df_to_py_df(&metrics).unwrap()
    }

    /// Display the overview and metrics DataFrames as DataTables in the web browser
    pub fn display(&self) {
        let overview = self.screener.overview();
        overview.show().unwrap();
        let metrics = tokio::runtime::Runtime::new().unwrap().block_on(self.screener.metrics()).unwrap();
        metrics.show().unwrap();
    }
}

