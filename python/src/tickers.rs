use std::str::FromStr;
use tokio::task;
use pyo3::prelude::*;
use polars::export::chrono;
use pyo3_polars::PyDataFrame;
use finalytics::prelude::*;
use crate::ffi::{rust_df_to_py_df, rust_plot_to_py_plot};
use crate::ticker::PyTicker;
use crate::portfolio::PyPortfolio;


/// Creates a new Tickers object
///
/// # Arguments
///
/// * `symbols` - `List[str]` - A list of ticker symbols
/// * `start_date` - `str` - The start date of the time period in the format YYYY-MM-DD
/// * `end_date` - `str` - The end date of the time period in the format YYYY-MM-DD
/// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
/// * `benchmark_symbol` - `str` - The ticker symbol of the benchmark to compare against
/// * `confidence_level` - `float` - The confidence level for the VaR and ES calculations
/// * `risk_free_rate` - `float` - The risk-free rate to use in the calculations
///
/// # Optional Arguments (For Custom Data)
///
/// * `tickers_data` - `List[DataFrame]` - A list of Polars DataFrames containing the ticker data for each symbol
/// * `benchmark_data` - `DataFrame` - A Polars DataFrame containing the benchmark data
///
/// # Returns
///
/// `Tickers` - A Tickers object
///
/// # Example
///
/// ```python
/// import finalytics
///
/// ticker = finalytics.Tickers(symbols=["AAPL", "MSFT", "NVDA", "BTC-USD],
///                             start_date="2020-01-01",
///                             end_date="2021-01-01",
///                             interval="1d",
///                             benchmark_symbol="^GSPC",
///                             confidence_level=0.95,
///                             risk_free_rate=0.02)
/// ```
#[pyclass]
#[pyo3(name = "Tickers")]
pub struct PyTickers {
    tickers: Tickers
}

#[pymethods]
impl PyTickers {
    #[new]
    #[pyo3(signature = (symbols, start_date=None, end_date=None, interval=None, benchmark_symbol=None,
    confidence_level=None, risk_free_rate=None, tickers_data=None, benchmark_data=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(symbols: Vec<String>, start_date: Option<String>, end_date: Option<String>, interval: Option<String>, benchmark_symbol: Option<String>,
               confidence_level: Option<f64>, risk_free_rate: Option<f64>,
               tickers_data: Option<Vec<PyDataFrame>>, benchmark_data: Option<PyDataFrame>) -> Self {
        let symbols: Vec<&str> = symbols.iter().map(|x| x.as_str()).collect();
        let tickers_data = tickers_data.map(|data: Vec<PyDataFrame>| {
            symbols.clone().into_iter().zip(data).map(|(symbol, df)| {
                KLINE::from_dataframe(symbol, &df.0).unwrap()
            }).collect::<Vec<KLINE>>()
        });
        let benchmark_data = benchmark_data.map(|df| KLINE::from_dataframe(
            &benchmark_symbol.clone().unwrap_or("Benchmark".to_string()), &df.0).unwrap());
        let default_start = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(365))
            .unwrap().format("%Y-%m-%d").to_string();
        let default_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval = Interval::from_str(&interval.unwrap_or("1d".to_string())).unwrap();
        task::block_in_place(move || {
            let tickers = Tickers::builder()
                .tickers(symbols)
                .start_date(&start_date.unwrap_or(default_start))
                .end_date(&end_date.unwrap_or(default_end))
                .interval(interval)
                .benchmark_symbol(&benchmark_symbol.unwrap_or("^GSPC".to_string()))
                .confidence_level(confidence_level.unwrap_or(0.95))
                .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                .tickers_data(tickers_data)
                .benchmark_data(benchmark_data)
                .build();
            PyTickers {
                tickers
            }
        })
    }

    /// Fetch the Ticker Summary Stats Data for all tickers
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the summary statistics for each ticker
    pub fn get_summary_stats(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_ticker_stats()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the OHLCV Data for all tickers
    pub fn get_price_history(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_chart()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the Options Chain Data for all tickers
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the options chain data for all tickers
    pub fn get_options_chain(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_options()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the Historical News Headlines for all tickers
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the news headlines for all tickers
    pub fn get_news(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_news()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the income statement for all tickers
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the income statement (e.g., "quarterly", "annual")
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the income statement data for all tickers
    pub fn get_income_statement(&self, frequency: &str) -> PyObject {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_financials(StatementType::IncomeStatement, frequency)
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the balance sheet for all tickers
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the balance sheet (e.g., "quarterly", "annual")
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the balance sheet data for all tickers
    pub fn get_balance_sheet(&self, frequency: &str) -> PyObject {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_financials(StatementType::BalanceSheet, frequency)
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the cash flow statement for all tickers
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the cash flow statement (e.g., "quarterly", "annual")
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the cash flow statement data for all tickers
    pub fn get_cashflow_statement(&self, frequency: &str) -> PyObject {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_financials(StatementType::CashFlowStatement, frequency)
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the financial ratios for all tickers
    ///
    /// # Arguments
    ///
    /// * `frequency` - `str` - The frequency of the financial ratios (e.g., "quarterly", "annual")
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the financial ratios data for all tickers
    pub fn get_financial_ratios(&self, frequency: &str) -> PyObject {
        task::block_in_place(move || {
            let frequency = StatementFrequency::from_str(frequency).unwrap();
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_financials(StatementType::FinancialRatios, frequency)
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Compute the returns for all tickers
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the returns data for all tickers
    pub fn returns(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.returns()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Compute the performance stats for all tickers
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the performance statistics for all tickers
    pub fn performance_stats(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.performance_stats()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Display the cumulative returns chart for all tickers
    ///
    /// # Arguments
    ///
    /// * `height` - `int` - The height of the chart in pixels (optional)
    /// * `width` - `int` - The width of the chart in pixels (optional)
    ///
    /// # Returns
    ///
    /// `Plot` - A Plotly plot object containing the cumulative returns chart
    #[pyo3(signature = (height=None, width=None))]
    pub fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.returns_chart(height, width)
            ).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }

    /// Display the Returns correlation matrix for all tickers
    ///
    /// # Arguments
    ///
    /// * `height` - `int` - The height of the chart in pixels (optional)
    /// * `width` - `int` - The width of the chart in pixels (optional)
    ///
    /// # Returns
    ///
    /// `Plot` - A Plotly plot object containing the returns correlation matrix
    #[pyo3(signature = (height=None, width=None))]
    pub fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.returns_matrix(height, width)
            ).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }

    /// Displays the analytics report for the tickers
    ///
    /// # Arguments
    ///
    /// * `report_type` - `optional str` - The type of report to display (performance)
    #[pyo3(signature = (report_type=None))]
    pub fn report(&self, report_type: Option<String>) {
        task::block_in_place(move || {
            let report_type = match report_type {
                Some(report_type) => ReportType::from_str(&report_type).unwrap(),
                None => ReportType::Performance
            };
            let report = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.report(Some(report_type))).unwrap();
            report.show().unwrap();
        });
    }

    /// Fetch the Ticker object for a specific ticker symbol
    ///
    /// # Arguments
    ///
    /// * `symbol` - `str` - The ticker symbol to fetch (e.g., "AAPL", "MSFT", "BTC-USD")
    ///
    /// # Returns
    ///
    /// `Ticker` - A Ticker object containing the ticker data
    pub fn get_ticker(&self, symbol: &str) -> PyTicker {
        PyTicker::new(
            symbol,
            Some(self.tickers.start_date.clone()),
            Some(self.tickers.end_date.clone()),
            Some(self.tickers.interval.to_string()),
            Some(self.tickers.benchmark_symbol.clone()),
            Some(self.tickers.confidence_level),
            Some(self.tickers.risk_free_rate),
            None,
            None
        )
    }

    /// Optimizes the tickers given the objective function and constraints
    ///
    /// # Arguments
    ///
    /// * `objective_function` - `optional str` - The objective function to optimize the portfolio ("max_sharpe", "min_vol", "max_return", "min_drawdown", "min_var", "min_cvar")
    /// * `constraints` - `optional List[Tuple[float, float]]` - A list of tuples representing the constraints for the optimization (e.g., [(0.1, 0.5), (0.2, 0.8)])
    ///
    /// # Returns
    ///
    /// `Portfolio` - A Portfolio object containing the optimized portfolio
    #[pyo3(signature = (objective_function=None, constraints=None))]
    pub fn optimize(&self, objective_function: Option<String>, constraints: Option<Vec<(f64, f64)>>) -> PyPortfolio {
        PyPortfolio::new(
            self.tickers.tickers.clone().iter().map(|x| x.ticker.to_string()).collect(),
            Some(self.tickers.benchmark_symbol.clone()),
            Some(self.tickers.start_date.clone()),
            Some(self.tickers.end_date.clone()),
            Some(self.tickers.interval.to_string()),
            Some(self.tickers.confidence_level),
            Some(self.tickers.risk_free_rate),
            objective_function,
            constraints,
            None,
            None
        )
    }
}