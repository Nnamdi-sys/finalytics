use crate::ffi::{error_chain_string, rust_plot_to_py_plot};
use finalytics::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_polars::{PyDataFrame, PySeries};
use std::str::FromStr;
use tokio::task;

/// Helper to create a tokio runtime, mapped to a Python RuntimeError.
fn rt() -> PyResult<tokio::runtime::Runtime> {
    tokio::runtime::Runtime::new()
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to create async runtime: {e}")))
}

/// Create a new Ticker object
///
/// # Arguments
///
/// * `symbol` - `str` - The ticker symbol of the asset
/// * `start_date` - `str` - The start date of the time period in the format YYYY-MM-DD
/// * `end_date` - `str` - The end date of the time period in the format YYYY-MM-DD
/// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
/// * `benchmark_symbol` - `str` - The ticker symbol of the benchmark to compare against
/// * `confidence_level` - `float` - The confidence level for the VaR and ES calculations
/// * `risk_free_rate` - `float` - The risk-free rate to use in the calculations
///
/// # Optional Arguments (For Custom Data)
///
/// * `ticker_data` - `DataFrame` - A Polars DataFrame containing the ticker data
/// * `benchmark_data` - `DataFrame` - A Polars DataFrame containing the benchmark data
///
/// # Returns
///
/// `Ticker` - A Ticker object
///
/// # Example
///
/// ```python
/// import finalytics
///
/// ticker = finalytics.Ticker(symbol="AAPL", start_date="2020-01-01", end_date="2021-01-01", interval="1d",
/// benchmark_symbol="^GSPC", confidence_level=0.95, risk_free_rate=0.02)
/// ```
#[pyclass]
#[pyo3(name = "Ticker")]
pub struct PyTicker {
    pub ticker: Ticker,
}

#[pymethods]
impl PyTicker {
    #[new]
    #[pyo3(signature = (symbol, start_date=None, end_date=None, interval=None, benchmark_symbol=None,
    confidence_level=None, risk_free_rate=None, ticker_data=None, benchmark_data=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        symbol: &str,
        start_date: Option<String>,
        end_date: Option<String>,
        interval: Option<String>,
        benchmark_symbol: Option<String>,
        confidence_level: Option<f64>,
        risk_free_rate: Option<f64>,
        ticker_data: Option<PyDataFrame>,
        benchmark_data: Option<PyDataFrame>,
    ) -> PyResult<Self> {
        let default_start = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::days(365))
            .unwrap() // infallible: 365 days is always valid
            .format("%Y-%m-%d")
            .to_string();
        let default_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval_str = interval.unwrap_or("1d".to_string());
        let interval = Interval::from_str(&interval_str)
            .map_err(|_| PyRuntimeError::new_err(format!("Invalid interval: '{interval_str}'")))?;

        let ticker_data = match ticker_data {
            Some(data) => {
                let kline = KLINE::from_dataframe(symbol, &data.0).map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to parse ticker data for '{symbol}': {}",
                        error_chain_string(&*e)
                    ))
                })?;
                Some(kline)
            }
            None => None,
        };

        let benchmark_data = match benchmark_data {
            Some(data) => {
                let name = benchmark_symbol.clone().unwrap_or("Benchmark".to_string());
                let kline = KLINE::from_dataframe(&name, &data.0).map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to parse benchmark data for '{name}': {}",
                        error_chain_string(&*e)
                    ))
                })?;
                Some(kline)
            }
            None => None,
        };

        task::block_in_place(move || {
            let mut builder = Ticker::builder()
                .ticker(symbol)
                .start_date(&start_date.unwrap_or(default_start))
                .end_date(&end_date.unwrap_or(default_end))
                .interval(interval)
                .confidence_level(confidence_level.unwrap_or(0.95))
                .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                .ticker_data(ticker_data)
                .benchmark_data(benchmark_data);
            if let Some(ref sym) = benchmark_symbol {
                builder = builder.benchmark_symbol(sym);
            }
            let ticker = builder.build();
            Ok(PyTicker { ticker })
        })
    }

    /// Get the current ticker quote stats
    ///
    /// # Returns
    ///
    /// `dict` - The current ticker quote stats
    pub fn get_quote(&self) -> PyResult<Py<PyDict>> {
        task::block_in_place(move || {
            let rt = rt()?;
            let quote = rt.block_on(self.ticker.get_quote()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to fetch quote: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", quote.symbol)?;
                locals.set_item("Name", quote.name)?;
                locals.set_item("Exchange", quote.exchange)?;
                locals.set_item("Currency", quote.currency)?;
                locals.set_item("Timestamp", quote.timestamp)?;
                locals.set_item("Current Price", quote.price)?;
                locals.set_item("24H Volume", quote.volume)?;
                locals.set_item("24H Open", quote.open)?;
                locals.set_item("24H High", quote.high)?;
                locals.set_item("24H Low", quote.low)?;
                locals.set_item("24H Close", quote.close)?;
                Ok(locals.into())
            })
        })
    }

    /// Get summary technical and fundamental statistics for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the summary statistics
    pub fn get_summary_stats(&self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let rt = rt()?;
            let ticker_stats = rt
                .block_on(self.ticker.get_ticker_stats())
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to fetch summary stats: {}",
                        error_chain_string(&*e)
                    ))
                })?
                .to_dataframe()
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to convert summary stats to DataFrame: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            Ok(PyDataFrame(ticker_stats))
        })
    }

    /// Get the ohlcv data for the ticker for a given time period
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the ohlcv data
    pub fn get_price_history(&self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let rt = rt()?;
            let price_history = rt.block_on(self.ticker.get_chart()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to fetch price history: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Ok(PyDataFrame(price_history))
        })
    }

    /// Get the options chain for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the options chain
    pub fn get_options_chain(&self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let rt = rt()?;
            let options = rt.block_on(self.ticker.get_options()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to fetch options chain: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Ok(PyDataFrame(options.chain))
        })
    }

    /// Get the latest news for the given ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the ticker news headlines for given date range
    pub fn get_news(&self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let rt = rt()?;
            let news = rt.block_on(self.ticker.get_news()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to fetch news: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Ok(PyDataFrame(news))
        })
    }

    /// Get the Income Statement for the ticker
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the statement (annual or quarterly)
    /// * `formatted` - `Optional[bool]` - Whether to return the statement in a formatted manner (default is True)
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Income Statement
    #[pyo3(signature = (frequency, formatted=None))]
    pub fn get_income_statement(
        &self,
        frequency: &str,
        formatted: Option<bool>,
    ) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).map_err(|_| {
                PyRuntimeError::new_err(format!("Invalid frequency: '{frequency}'"))
            })?;
            let rt = rt()?;
            let df = rt
                .block_on(self.ticker.get_financials(
                    StatementType::IncomeStatement,
                    frequency,
                    formatted,
                ))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to fetch income statement: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get the Balance Sheet for the ticker
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the statement (annual or quarterly)
    /// * `formatted` - `Optional[bool]` - Whether to return the statement in a formatted manner (default is True)
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Balance Sheet
    #[pyo3(signature = (frequency, formatted=None))]
    pub fn get_balance_sheet(
        &self,
        frequency: &str,
        formatted: Option<bool>,
    ) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).map_err(|_| {
                PyRuntimeError::new_err(format!("Invalid frequency: '{frequency}'"))
            })?;
            let rt = rt()?;
            let df = rt
                .block_on(self.ticker.get_financials(
                    StatementType::BalanceSheet,
                    frequency,
                    formatted,
                ))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to fetch balance sheet: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get the Cashflow Statement for the ticker
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the statement (annual or quarterly)
    /// * `formatted` - `Optional[bool]` - Whether to return the statement in a formatted manner (default is True)
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Cashflow Statement
    #[pyo3(signature = (frequency, formatted=None))]
    pub fn get_cashflow_statement(
        &self,
        frequency: &str,
        formatted: Option<bool>,
    ) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).map_err(|_| {
                PyRuntimeError::new_err(format!("Invalid frequency: '{frequency}'"))
            })?;
            let rt = rt()?;
            let df = rt
                .block_on(self.ticker.get_financials(
                    StatementType::CashFlowStatement,
                    frequency,
                    formatted,
                ))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to fetch cashflow statement: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get the Financial Ratios for the ticker
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the statement (annual or quarterly)
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the Financial Ratios
    pub fn get_financial_ratios(&self, frequency: &str) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).map_err(|_| {
                PyRuntimeError::new_err(format!("Invalid frequency: '{frequency}'"))
            })?;
            let rt = rt()?;
            let df = rt
                .block_on(self.ticker.get_financials(
                    StatementType::FinancialRatios,
                    frequency,
                    None,
                ))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to fetch financial ratios: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            Ok(PyDataFrame(df))
        })
    }

    /// Get the implied volatility surface for the ticker options chain
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the implied volatility surface
    pub fn volatility_surface(&self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let rt = rt()?;
            let surface = rt.block_on(self.ticker.volatility_surface()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to compute volatility surface: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Ok(PyDataFrame(surface.ivols_df))
        })
    }

    /// Compute the performance statistics for the ticker
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the performance statistics
    pub fn performance_stats(&self) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let rt = rt()?;
            let performance_stats = rt.block_on(self.ticker.performance_stats()).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to compute performance stats: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", performance_stats.ticker_symbol)?;
                locals.set_item("Benchmark", performance_stats.benchmark_symbol)?;
                locals.set_item("Start Date", performance_stats.start_date)?;
                locals.set_item("End Date", performance_stats.end_date)?;
                locals.set_item("Interval", performance_stats.interval.average)?;
                locals.set_item("Confidence Level", performance_stats.confidence_level)?;
                locals.set_item("Risk Free Rate", performance_stats.risk_free_rate)?;
                locals.set_item(
                    "Daily Return",
                    performance_stats.performance_stats.daily_return,
                )?;
                locals.set_item(
                    "Daily Volatility",
                    performance_stats.performance_stats.daily_volatility,
                )?;
                locals.set_item(
                    "Total Return",
                    performance_stats.performance_stats.cumulative_return,
                )?;
                locals.set_item(
                    "Annualized Return",
                    performance_stats.performance_stats.annualized_return,
                )?;
                locals.set_item(
                    "Annualized Volatility",
                    performance_stats.performance_stats.annualized_volatility,
                )?;
                locals.set_item("Alpha", performance_stats.performance_stats.alpha)?;
                locals.set_item("Beta", performance_stats.performance_stats.beta)?;
                locals.set_item(
                    "Sharpe Ratio",
                    performance_stats.performance_stats.sharpe_ratio,
                )?;
                locals.set_item(
                    "Sortino Ratio",
                    performance_stats.performance_stats.sortino_ratio,
                )?;
                locals.set_item(
                    "Active Return",
                    performance_stats.performance_stats.active_return,
                )?;
                locals.set_item(
                    "Active Risk",
                    performance_stats.performance_stats.active_risk,
                )?;
                locals.set_item(
                    "Information Ratio",
                    performance_stats.performance_stats.information_ratio,
                )?;
                locals.set_item(
                    "Calmar Ratio",
                    performance_stats.performance_stats.calmar_ratio,
                )?;
                locals.set_item(
                    "Maximum Drawdown",
                    performance_stats.performance_stats.maximum_drawdown,
                )?;
                locals.set_item(
                    "Value at Risk",
                    performance_stats.performance_stats.value_at_risk,
                )?;
                locals.set_item(
                    "Expected Shortfall",
                    performance_stats.performance_stats.expected_shortfall,
                )?;
                locals.set_item(
                    "Security Prices",
                    PySeries(performance_stats.security_prices),
                )?;
                locals.set_item(
                    "Security Returns",
                    PySeries(performance_stats.security_returns),
                )?;
                match performance_stats.benchmark_returns {
                    Some(br) => locals.set_item("Benchmark Returns", PySeries(br))?,
                    None => locals.set_item("Benchmark Returns", py.None())?,
                };
                Ok(locals.into())
            })
        })
    }

    /// Display the performance chart for the ticker
    ///
    /// # Arguments
    ///
    /// * `height` - `optional int` - The height of the chart
    /// * `width` - `optional int` - The width of the chart
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (height=None, width=None))]
    pub fn performance_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let rt = rt()?;
            let plot = rt
                .block_on(self.ticker.performance_chart(height, width))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to generate performance chart: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert performance chart plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Display the candlestick chart for the ticker
    ///
    /// # Arguments
    ///
    /// * `height` - `optional int` - The height of the chart
    /// * `width` - `optional int` - The width of the chart
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (height=None, width=None))]
    pub fn candlestick_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let rt = rt()?;
            let plot = rt
                .block_on(self.ticker.candlestick_chart(height, width))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to generate candlestick chart: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert candlestick chart plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Display the options volatility surface, smile and term structure charts for the ticker
    ///
    /// # Arguments
    ///
    /// * `chart_type` - `str` - The type of chart to display (surface, smile, term_structure)
    /// * `height` - `optional int` - The height of the chart`
    /// * `width` - `optional int` - The width of the chart
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (chart_type, height=None, width=None))]
    pub fn options_chart(
        &self,
        chart_type: String,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let rt = rt()?;
            let options_chart = rt
                .block_on(self.ticker.options_charts(height, width))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to generate options charts: {}",
                        error_chain_string(&*e)
                    ))
                })?;

            let plot = match chart_type.as_str() {
                "surface" => options_chart.volatility_surface,
                "smile" => options_chart.volatility_smile,
                "term_structure" => options_chart.volatility_term_structure,
                other => {
                    return Err(PyRuntimeError::new_err(format!(
                    "Invalid chart type: '{other}'. Choose 'surface', 'smile', or 'term_structure'"
                )))
                }
            };

            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert options chart plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Display the news sentiment chart for the ticker
    ///
    /// # Arguments
    ///
    /// * `height` - `optional int` - The height of the chart
    /// * `width` - `optional int` - The width of the chart
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (height=None, width=None))]
    pub fn news_sentiment_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let rt = rt()?;
            let plot = rt
                .block_on(self.ticker.news_sentiment_chart(height, width))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to generate news sentiment chart: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert news sentiment chart plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Displays the analytics report for the ticker
    ///
    /// # Arguments
    ///
    /// * `report_type` - `optional str` - The type of report to display (performance, financials, options, news)
    /// * `display` - Optional str - Display mode ("notebook" to display in Jupyter, else displays to default web browser)
    #[pyo3(signature = (report_type=None, display=None))]
    pub fn report(&self, report_type: Option<String>, display: Option<String>) -> PyResult<()> {
        task::block_in_place(move || {
            let report_type = match report_type {
                Some(rt_str) => ReportType::from_str(&rt_str).map_err(|_| {
                    PyRuntimeError::new_err(format!("Invalid report type: '{rt_str}'"))
                })?,
                None => ReportType::Performance,
            };
            let rt = rt()?;
            let report = rt
                .block_on(self.ticker.report(Some(report_type)))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to generate report: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            if display.as_deref() == Some("notebook") {
                let html_content = report.to_html();
                Python::with_gil(|py| -> PyResult<()> {
                    let ipython_display = py.import("IPython.display")?;
                    let html_class = ipython_display.getattr("HTML")?;
                    let display_fn = ipython_display.getattr("display")?;
                    let html_obj = html_class.call1((html_content,))?;
                    display_fn.call1((html_obj,))?;
                    Ok(())
                })?;
            } else {
                report.show().map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to display report: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            }
            Ok(())
        })
    }
}
