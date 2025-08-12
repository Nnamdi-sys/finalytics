use std::str::FromStr;
use tokio::task;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_polars::{PyDataFrame, PySeries};
use finalytics::prelude::*;
use crate::ffi::rust_plot_to_py_plot;

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
    pub ticker: Ticker
}

#[pymethods]
impl PyTicker {
    #[new]
    #[pyo3(signature = (symbol, start_date=None, end_date=None, interval=None, benchmark_symbol=None,
    confidence_level=None, risk_free_rate=None, ticker_data=None, benchmark_data=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(symbol: &str, start_date: Option<String>, end_date: Option<String>, interval: Option<String>, benchmark_symbol: Option<String>,
    confidence_level: Option<f64>, risk_free_rate: Option<f64>, ticker_data: Option<PyDataFrame>, benchmark_data: Option<PyDataFrame>) -> Self {
        let default_start = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(365))
            .unwrap().format("%Y-%m-%d").to_string();
        let default_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval = Interval::from_str(&interval.unwrap_or("1d".to_string())).unwrap();
        task::block_in_place(move || {
            let ticker_data = ticker_data.map(|data| KLINE::from_dataframe(symbol, &data.0).unwrap());
            let benchmark_data = benchmark_data.map(|data| KLINE::from_dataframe(
                &benchmark_symbol.clone().unwrap_or("Benchmark".to_string()),
                &data.0).unwrap());
            let ticker = Ticker::builder()
                .ticker(symbol)
                .start_date(&start_date.unwrap_or(default_start))
                .end_date(&end_date.unwrap_or(default_end))
                .interval(interval)
                .benchmark_symbol(&benchmark_symbol.unwrap_or("^GSPC".to_string()))
                .confidence_level(confidence_level.unwrap_or(0.95))
                .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                .ticker_data(ticker_data)
                .benchmark_data(benchmark_data)
                .build();
            PyTicker {
                ticker
            }
        })
    }

    /// Get the current ticker quote stats
    ///
    /// # Returns
    ///
    /// `dict` - The current ticker quote stats
    pub fn get_quote(&self) -> Py<PyDict> {
        task::block_in_place(move || {
            let quote = tokio::runtime::Runtime::new().unwrap().block_on(self.ticker.get_quote()).unwrap();
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", quote.symbol).unwrap();
                locals.set_item("Name", quote.name).unwrap();
                locals.set_item("Exchange", quote.exchange).unwrap();
                locals.set_item("Currency", quote.currency).unwrap();
                locals.set_item("Timestamp", quote.timestamp).unwrap();
                locals.set_item("Current Price", quote.price).unwrap();
                locals.set_item("24H Volume", quote.volume).unwrap();
                locals.set_item("24H Open", quote.open).unwrap();
                locals.set_item("24H High", quote.high).unwrap();
                locals.set_item("24H Low", quote.low).unwrap();
                locals.set_item("24H Close", quote.close).unwrap();
                locals.into()
            })
        })
    }

    /// Get summary technical and fundamental statistics for the ticker
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the summary statistics
    pub fn get_summary_stats(&self) -> PyDataFrame {
        task::block_in_place(move || {
            let ticker_stats = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_ticker_stats()
            ).unwrap().to_dataframe().unwrap();
            PyDataFrame(ticker_stats)
        })
    }

    /// Get the ohlcv data for the ticker for a given time period
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the ohlcv data
    pub fn get_price_history(&self) -> PyDataFrame {
        task::block_in_place(move || {
            let price_history = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_chart()
            ).unwrap();
            PyDataFrame(price_history)
        })
    }

    /// Get the options chain for the ticker
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the options chain
    pub fn get_options_chain(&self) -> PyDataFrame {
        task::block_in_place(move || {
            let options_chain = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_options()
            ).unwrap().chain;
            PyDataFrame(options_chain)
        })
    }

    /// Get the latest news for the given ticker
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the ticker news headlines for given date range
    pub fn get_news(&self) -> PyDataFrame {
        task::block_in_place(move || {
            let news = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_news()
            ).unwrap();

            PyDataFrame(news)
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
    pub fn get_income_statement(&self, frequency: &str, formatted: Option<bool>) -> PyDataFrame {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let income_statement = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_financials(StatementType::IncomeStatement, frequency, formatted)).unwrap();
            PyDataFrame(income_statement)
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
    pub fn get_balance_sheet(&self, frequency: &str, formatted: Option<bool>) -> PyDataFrame {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let balance_sheet = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_financials(StatementType::BalanceSheet, frequency, formatted)).unwrap();
            PyDataFrame(balance_sheet)
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
    pub fn get_cashflow_statement(&self, frequency: &str, formatted: Option<bool>) -> PyDataFrame {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let cashflow_statement = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_financials(StatementType::CashFlowStatement, frequency, formatted)).unwrap();
            PyDataFrame(cashflow_statement)
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
    pub fn get_financial_ratios(&self, frequency: &str) -> PyDataFrame {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let ratios = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.get_financials(StatementType::FinancialRatios, frequency, None)).unwrap();
            PyDataFrame(ratios)
        })
    }

    /// Get the implied volatility surface for the ticker options chain
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the implied volatility surface
    pub fn volatility_surface(&self) -> PyDataFrame {
        task::block_in_place(move || {
            let volatility_surface = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.volatility_surface()).unwrap();
            PyDataFrame(volatility_surface.ivols_df)
        })
    }

    /// Compute the performance statistics for the ticker
    ///
    /// # Returns
    ///
    /// `dict` - A dictionary containing the performance statistics
    pub fn performance_stats(&self) -> PyObject {
        task::block_in_place(move || {
            let performance_stats = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.performance_stats()).unwrap();
            Python::with_gil(|py| {
                let locals = PyDict::new(py);
                locals.set_item("Symbol", performance_stats.ticker_symbol).unwrap();
                locals.set_item("Benchmark", performance_stats.benchmark_symbol).unwrap();
                locals.set_item("Start Date", performance_stats.start_date).unwrap();
                locals.set_item("End Date", performance_stats.end_date).unwrap();
                locals.set_item("Interval", performance_stats.interval.average).unwrap();
                locals.set_item("Confidence Level", performance_stats.confidence_level).unwrap();
                locals.set_item("Risk Free Rate", performance_stats.risk_free_rate).unwrap();
                locals.set_item("Daily Return", performance_stats.performance_stats.daily_return).unwrap();
                locals.set_item("Daily Volatility", performance_stats.performance_stats.daily_volatility).unwrap();
                locals.set_item("Total Return", performance_stats.performance_stats.cumulative_return).unwrap();
                locals.set_item("Annualized Return", performance_stats.performance_stats.annualized_return).unwrap();
                locals.set_item("Annualized Volatility", performance_stats.performance_stats.annualized_volatility).unwrap();
                locals.set_item("Alpha", performance_stats.performance_stats.alpha).unwrap();
                locals.set_item("Beta", performance_stats.performance_stats.beta).unwrap();
                locals.set_item("Sharpe Ratio", performance_stats.performance_stats.sharpe_ratio).unwrap();
                locals.set_item("Sortino Ratio", performance_stats.performance_stats.sortino_ratio).unwrap();
                locals.set_item("Active Return", performance_stats.performance_stats.active_return).unwrap();
                locals.set_item("Active Risk", performance_stats.performance_stats.active_risk).unwrap();
                locals.set_item("Information Ratio", performance_stats.performance_stats.information_ratio).unwrap();
                locals.set_item("Calmar Ratio", performance_stats.performance_stats.calmar_ratio).unwrap();
                locals.set_item("Maximum Drawdown", performance_stats.performance_stats.maximum_drawdown).unwrap();
                locals.set_item("Value at Risk", performance_stats.performance_stats.value_at_risk).unwrap();
                locals.set_item("Expected Shortfall", performance_stats.performance_stats.expected_shortfall).unwrap();
                locals.set_item("Security Prices", PySeries(performance_stats.security_prices)).unwrap();
                locals.set_item("Security Returns", PySeries(performance_stats.security_returns)).unwrap();
                locals.set_item("Benchmark Returns", PySeries(performance_stats.benchmark_returns)).unwrap();
                locals.into()
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
    pub fn performance_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        let plot = task::block_in_place(move || {
            let performance_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.performance_chart(height, width)).unwrap();
            performance_chart
        });

        rust_plot_to_py_plot(plot).unwrap()
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
    pub fn candlestick_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject  {
        let plot = task::block_in_place(move || {
            let candlestick_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.candlestick_chart(height, width)).unwrap();
            candlestick_chart
        });

        rust_plot_to_py_plot(plot).unwrap()
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
    pub fn options_chart(&self, chart_type: String, height: Option<usize>, width: Option<usize>) -> PyObject {
        let plot = task::block_in_place(move || {

            let options_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.options_charts(height, width)).unwrap();

            match chart_type.as_str() {
                "surface" => options_chart.volatility_surface,
                "smile" => options_chart.volatility_smile,
                "term_structure" => options_chart.volatility_term_structure,
                _ => panic!("Invalid chart type. Please choose either 'surface', 'smile' or 'term_structure'"),
            }

        });

        rust_plot_to_py_plot(plot).unwrap()
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
    pub fn news_sentiment_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        let plot = task::block_in_place(move || {
            let news_sentiment_chart = tokio::runtime::Runtime::new().unwrap().block_on(
                self.ticker.news_sentiment_chart(height, width)).unwrap();
            news_sentiment_chart
        });

        rust_plot_to_py_plot(plot).unwrap()
    }

    /// Displays the analytics report for the ticker
    ///
    /// # Arguments
    ///
    /// * `report_type` - `optional str` - The type of report to display (performance, financials, options, news)
    /// * `display` - Optional str - Display mode ("notebook" to display in Jupyter, else displays to default web browser)
    #[pyo3(signature = (report_type=None, display=None))]
    pub fn report(&self, report_type: Option<String>, display: Option<String>) {
        task::block_in_place(move || {
            let report_type = match report_type {
                Some(report_type) => ReportType::from_str(&report_type).unwrap(),
                None => ReportType::Performance,
            };
            let report = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(self.ticker.report(Some(report_type)))
                .unwrap();
            if display.as_deref() == Some("notebook") {
                let html_content = report.to_html();
                Python::with_gil(|py| {
                    let ipython_display = py.import("IPython.display").unwrap();
                    let html_class = ipython_display.getattr("HTML").unwrap();
                    let display_fn = ipython_display.getattr("display").unwrap();
                    let html_obj = html_class.call1((html_content,)).unwrap();
                    display_fn.call1((html_obj,)).unwrap();
                });
            } else {
                report.show().unwrap();
            }
        });
    }

}

