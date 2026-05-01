use crate::ffi::error_chain_string;
use finalytics::data::yahoo::screeners::builder::ScreenerBuilder;
use finalytics::prelude::{
    CryptoScreener, EquityScreener, EtfScreener, FutureScreener, IndexScreener, MutualFundScreener,
    QuoteType, Screener, ScreenerFilter, ScreenerMetric,
};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use std::str::FromStr;
use tokio::task;

/// Helper to create a tokio runtime, mapped to a Python RuntimeError.
fn rt() -> PyResult<tokio::runtime::Runtime> {
    tokio::runtime::Runtime::new()
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to create async runtime: {e}")))
}

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
    screener: Screener,
}

#[pymethods]
impl PyScreener {
    #[new]
    #[pyo3(signature = (quote_type, filters, sort_field=None, sort_descending=true, offset=0, size=250))]
    pub fn new(
        quote_type: &str,
        filters: Vec<String>,
        sort_field: Option<String>,
        sort_descending: bool,
        offset: usize,
        size: usize,
    ) -> PyResult<Self> {
        let quote_type = QuoteType::from_str(quote_type).map_err(|_| {
            PyRuntimeError::new_err(format!(
                "Invalid quote type: '{quote_type}'. Choose from: EQUITY, MUTUALFUND, ETF, INDEX, FUTURE, CRYPTO"
            ))
        })?;
        let filters = filters
            .into_iter()
            .map(ScreenerFilter::Custom)
            .collect::<Vec<_>>();
        let sort_field = match sort_field {
            Some(f) => {
                let metric = match quote_type {
                    QuoteType::Equity => {
                        ScreenerMetric::Equity(EquityScreener::from_str(&f).map_err(|_| {
                            PyRuntimeError::new_err(format!("Invalid equity sort field: '{f}'"))
                        })?)
                    }
                    QuoteType::MutualFund => ScreenerMetric::MutualFund(
                        MutualFundScreener::from_str(&f).map_err(|_| {
                            PyRuntimeError::new_err(format!(
                                "Invalid mutual fund sort field: '{f}'"
                            ))
                        })?,
                    ),
                    QuoteType::Etf => {
                        ScreenerMetric::Etf(EtfScreener::from_str(&f).map_err(|_| {
                            PyRuntimeError::new_err(format!("Invalid ETF sort field: '{f}'"))
                        })?)
                    }
                    QuoteType::Index => {
                        ScreenerMetric::Index(IndexScreener::from_str(&f).map_err(|_| {
                            PyRuntimeError::new_err(format!("Invalid index sort field: '{f}'"))
                        })?)
                    }
                    QuoteType::Future => {
                        ScreenerMetric::Future(FutureScreener::from_str(&f).map_err(|_| {
                            PyRuntimeError::new_err(format!("Invalid future sort field: '{f}'"))
                        })?)
                    }
                    QuoteType::Crypto => {
                        ScreenerMetric::Crypto(CryptoScreener::from_str(&f).map_err(|_| {
                            PyRuntimeError::new_err(format!("Invalid crypto sort field: '{f}'"))
                        })?)
                    }
                };
                Some(metric)
            }
            None => None,
        };
        let screener_builder = ScreenerBuilder {
            quote_type: Some(quote_type),
            filters,
            sort_field,
            sort_descending,
            offset,
            size,
        };

        task::block_in_place(move || {
            let rt = rt()?;
            let screener = rt.block_on(screener_builder.build()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to build screener: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Ok(PyScreener { screener })
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
    pub fn overview(&self) -> PyDataFrame {
        let overview = self.screener.result.clone();
        PyDataFrame(overview)
    }

    /// Get a Polars DataFrame containing detailed metrics for screened instruments
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame with detailed metrics
    pub fn metrics(&self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let rt = rt()?;
            let metrics = rt.block_on(self.screener.metrics()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to fetch screener metrics: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Ok(PyDataFrame(metrics.data))
        })
    }

    /// Display the overview and metrics DataFrames as DataTables in the web browser or Jupyter Notebook
    ///
    /// # Arguments
    ///
    /// * `display` - Optional str - Display mode ("notebook" to display in Jupyter, else uses default web browser)
    #[pyo3(signature = (display=None))]
    pub fn display(&self, display: Option<String>) -> PyResult<()> {
        task::block_in_place(move || {
            let overview = self.screener.overview();
            let rt = rt()?;
            let metrics = rt.block_on(self.screener.metrics()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to fetch screener metrics: {}",
                    error_chain_string(&*e)
                ))
            })?;

            if display.as_deref() == Some("notebook") {
                Python::with_gil(|py| -> PyResult<()> {
                    let ipython_display = py.import("IPython.display")?;
                    let html_class = ipython_display.getattr("HTML")?;
                    let display_fn = ipython_display.getattr("display")?;

                    // Display overview HTML
                    let overview_html = overview.to_html().map_err(|e| {
                        PyRuntimeError::new_err(format!(
                            "Failed to render overview HTML: {}",
                            error_chain_string(&*e)
                        ))
                    })?;
                    let overview_html_obj = html_class.call1((overview_html,))?;
                    display_fn.call1((overview_html_obj,))?;

                    // Display metrics HTML
                    let metrics_html = metrics.to_html().map_err(|e| {
                        PyRuntimeError::new_err(format!(
                            "Failed to render metrics HTML: {}",
                            error_chain_string(&*e)
                        ))
                    })?;
                    let metrics_html_obj = html_class.call1((metrics_html,))?;
                    display_fn.call1((metrics_html_obj,))?;

                    Ok(())
                })?;
            } else {
                overview.show().map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to display overview: {}",
                        error_chain_string(&*e)
                    ))
                })?;
                metrics.show().map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to display metrics: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            }
            Ok(())
        })
    }
}
