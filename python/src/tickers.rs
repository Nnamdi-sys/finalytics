use tokio::task;
use pyo3::prelude::*;
use polars::export::chrono;
use finalytics::prelude::*;
use crate::ticker::PyTicker;
use crate::ffi::{rust_df_to_py_df, rust_plot_to_py_plot};
use crate::portfolio::PyPortfolio;


#[pyclass]
#[pyo3(name = "Tickers")]
pub struct PyTickers {
    tickers: Tickers
}


#[pymethods]
impl PyTickers {
    #[new]
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
    /// * `risk_free_rate` - `float` - The risk free rate to use in the calculations
    ///
    /// # Returns
    ///
    /// `Tickers` - A Tickers object
    ///
    /// # Example
    ///
    /// ```
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
    pub fn new(symbols: Vec<String>, start_date: Option<String>, end_date: Option<String>, interval: Option<String>, benchmark_symbol: Option<String>,
               confidence_level: Option<f64>, risk_free_rate: Option<f64>) -> Self {
        let symbols = symbols.iter().map(|x| x.as_str()).collect();
        let default_start = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(365))
            .unwrap().format("%Y-%m-%d").to_string();
        let default_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval = Interval::from_str(&interval.unwrap_or("1d".to_string()));
        task::block_in_place(move || {
            let tickers = TickersBuilder::new()
                .tickers(symbols)
                .start_date(&start_date.unwrap_or(default_start))
                .end_date(&end_date.unwrap_or(default_end))
                .interval(interval)
                .benchmark_symbol(&benchmark_symbol.unwrap_or("^GSPC".to_string()))
                .confidence_level(confidence_level.unwrap_or(0.95))
                .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                .build();
            PyTickers {
                tickers
            }
        })
    }

    /// Fetch the Ticker Summary Stats Data for all tickers
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
    pub fn get_options_chain(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_options()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the Historical News Headlines for all tickers
    pub fn get_news(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.get_news()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the income statement for all tickers
    pub fn get_income_statement(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.income_statement()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the balance sheet for all tickers
    pub fn get_balance_sheet(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.balance_sheet()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the cash flow statement for all tickers
    pub fn get_cashflow_statement(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.cashflow_statement()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Fetch the financial ratios for all tickers
    pub fn get_financial_ratios(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.financial_ratios()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Compute the returns for all tickers
    pub fn returns(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.returns()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Compute the performance stats for all tickers
    pub fn performance_stats(&self) -> PyObject {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.performance_stats()
            ).unwrap();
            rust_df_to_py_df(&df).unwrap()
        })
    }

    /// Display the cumulative returns chart for all tickers
    pub fn returns_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.returns_chart(height, width)
            ).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }

    /// Display the returns correlation matrix for all tickers
    pub fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = tokio::runtime::Runtime::new().unwrap().block_on(
                self.tickers.returns_matrix(height, width)
            ).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }

    /// Fetch the Ticker object for a specific ticker symbol
    pub fn get_ticker(&self, symbol: &str) -> PyTicker {
        PyTicker::new(
            symbol,
            Some(self.tickers.start_date.clone()),
            Some(self.tickers.end_date.clone()),
            Some(self.tickers.interval.to_string()),
            Some(self.tickers.benchmark_symbol.clone()),
            Some(self.tickers.confidence_level),
            Some(self.tickers.risk_free_rate)
        )
    }

    /// Optimizes the tickers given the objective function and constraints
    pub fn optimize(&self, objective_function: Option<String>, constraints: Option<Vec<(f64, f64)>>) -> PyPortfolio {
        PyPortfolio::new(
            self.tickers.tickers.clone().iter().map(|x| x.ticker.as_str()).collect(),
            Some(self.tickers.benchmark_symbol.clone()),
            Some(self.tickers.start_date.clone()),
            Some(self.tickers.end_date.clone()),
            Some(self.tickers.interval.to_string()),
            Some(self.tickers.confidence_level),
            Some(self.tickers.risk_free_rate),
            objective_function,
            constraints
        )
    }
}