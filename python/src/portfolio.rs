use std::str::FromStr;
use finalytics::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_polars::{PyDataFrame, PySeries};
use tokio::task;
use crate::ffi::rust_plot_to_py_plot;


/// Create a new Portfolio object
///
/// # Arguments
///
/// * `ticker_symbols` - `list` - list of ticker symbols
/// * `benchmark_symbol` - `str` - The ticker symbol of the benchmark to compare against
/// * `start_date` - `str` - The start date of the time period in the format YYYY-MM-DD
/// * `end_date` - `str` - The end date of the time period in the format YYYY-MM-DD
/// * `interval` - `str` - The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo)
/// * `confidence_level` - `float` - The confidence level for the VaR and ES calculations
/// * `risk_free_rate` - `float` - The risk-free rate to use in the calculations
/// * `objective_function` - `str` - The objective function to use in the optimization:
///     - `max_sharpe`: Maximize return per unit of volatility (Sharpe ratio)
///     - `max_sortino`: Maximize return per unit of downside risk (Sortino ratio)
///     - `min_vol`: Minimize total portfolio volatility
///     - `max_return`: Maximize expected portfolio return
///     - `min_var`: Minimize Value-at-Risk (VaR)
///     - `min_cvar`: Minimize Conditional Value-at-Risk (CVaR)
///     - `min_drawdown`: Minimize maximum portfolio drawdown
/// * `asset_constraints` - `list` - list of tuples with the lower and upper bounds for the ticker weights
/// * `categorical_constraints` - `list` - list of tuples defining category-based constraints.
///     Each tuple has the form `(category_name: str, category_per_symbol: list[str], weight_per_category: list[tuple[str, float, float]])`
///     where:
///       - `category_name` is the name of the constraint group (e.g., "AssetClass")
///       - `category_per_symbol` assigns each ticker to a category (in the same order as `ticker_symbols`)
///       - `weight_per_category` contains tuples of `(category_label, min_weight, max_weight)`
/// * `weights` - `list` - weights for the assets in the portfolio, if provided, it will override the optimization process
///
/// # Optional Arguments (For Custom Data)
///
/// * `tickers_data` - `List[DataFrame]` - A list of Polars DataFrames containing the ticker data for each symbol
/// * `benchmark_data` - `DataFrame` - A Polars DataFrame containing the benchmark data
///
///
/// # Returns
///
/// `Portfolio` object
///
/// # Example
///
/// ```
/// import finalytics
///
/// portfolio = finalytics.Portfolio(ticker_symbols = ["AAPL", "GOOG", "MSFT", "ZN=F"],
///                                  benchmark_symbol = "^GSPC",
///                                  start_date = "2020-01-01",
///                                  end_date = "2021-01-01",
///                                  interval = "1d",
///                                  confidence_level = 0.95,
///                                  risk_free_rate = 0.02,
///                                  objective_function = "max_sharpe",
///                                  asset_constraints = [(0.0, 0.5), (0.0, 0.5), (0.0, 0.5), (0.0, 0.5),
///                                  categorical_constraints=[(
    ///                                  "AssetClass",
    ///                                  ["EQUITY", "EQUITY", "EQUITY", "FIXED INCOME"],
    ///                                  [("EQUITY", 0.0, 0.8), ("FIXED INCOME", 0.0, 0.2)]
///                                  )],
///                                  weights = None)
/// ```
#[pyclass]
#[pyo3(name = "Portfolio")]
pub struct PyPortfolio {
    pub portfolio: Portfolio
}

#[pymethods]
impl PyPortfolio {
    #[new]
    #[pyo3(signature = (
    ticker_symbols, benchmark_symbol=None, start_date=None, end_date=None, interval=None,
    confidence_level=None, risk_free_rate=None, objective_function=None, asset_constraints=None,
    categorical_constraints=None, weights=None, tickers_data=None, benchmark_data=None))]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    pub fn new(ticker_symbols: Vec<String>, benchmark_symbol: Option<String>, start_date: Option<String>, end_date: Option<String>,
               interval: Option<String>, confidence_level: Option<f64>, risk_free_rate: Option<f64>,
               objective_function: Option<String>, asset_constraints: Option<Vec<(f64, f64)>>,
               categorical_constraints: Option<Vec<(String, Vec<String>, Vec<(String, f64, f64)>)>>,
               weights: Option<Vec<f64>>, tickers_data: Option<Vec<PyDataFrame>>,
               benchmark_data: Option<PyDataFrame>) -> Self {
        let ticker_symbols: Vec<&str> = ticker_symbols.iter().map(|x| x.as_str()).collect();
        let tickers_data = tickers_data.map(|data: Vec<PyDataFrame>| {
            ticker_symbols.clone().into_iter().zip(data).map(|(symbol, df)| {
                KLINE::from_dataframe(symbol, &df.0).unwrap()
            }).collect::<Vec<KLINE>>()
        });
        let benchmark_data = benchmark_data.map(|df| KLINE::from_dataframe(
            &benchmark_symbol.clone().unwrap_or("Benchmark".to_string()), &df.0).unwrap());
        let default_start = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(365))
            .unwrap().format("%Y-%m-%d").to_string();
        let default_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval = Interval::from_str(&interval.unwrap_or("1d".to_string())).unwrap();
        let objective_function = ObjectiveFunction::from_str(&objective_function.unwrap_or("max_sharpe".to_string())).unwrap();
        let constraints = Constraints {
            asset_weights: asset_constraints,
            categorical_weights: categorical_constraints.map(|cats| {
                cats.into_iter()
                    .map(|(name, category_per_symbol, weight_per_category)| CategoricalWeights {
                        name,
                        category_per_symbol,
                        weight_per_category,
                    })
                    .collect()
            }),
        };
        task::block_in_place(move || {
            let portfolio = tokio::runtime::Runtime::new().unwrap().block_on(
                Portfolio::builder()
                    .ticker_symbols(ticker_symbols)
                    .benchmark_symbol(&benchmark_symbol.unwrap_or("^GSPC".to_string()))
                    .start_date(&start_date.unwrap_or(default_start))
                    .end_date(&end_date.unwrap_or(default_end))
                    .interval(interval)
                    .confidence_level(confidence_level.unwrap_or(0.95))
                    .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                    .objective_function(objective_function)
                    .constraints(Some(constraints))
                    .weights(weights)
                    .tickers_data(tickers_data)
                    .benchmark_data(benchmark_data)
                    .build()).unwrap();
            PyPortfolio {
                portfolio
            }
        })
    }

    /// get the portfolio optimization results
    ///
    /// # Returns
    ///
    /// `dict` - dictionary of optimization results
    pub fn optimization_results(&self) -> PyObject {
        task::block_in_place(move || {
            Python::with_gil(|py| {
                let py_dict = PyDict::new(py);
                py_dict.set_item("ticker_symbols", self.portfolio.performance_stats.ticker_symbols.clone()).unwrap();
                py_dict.set_item("benchmark_symbol", self.portfolio.performance_stats.benchmark_symbol.clone()).unwrap();
                py_dict.set_item("start_date", self.portfolio.performance_stats.start_date.clone()).unwrap();
                py_dict.set_item("end_date", self.portfolio.performance_stats.end_date.clone()).unwrap();
                py_dict.set_item("interval", self.portfolio.performance_stats.interval.mode).unwrap();
                py_dict.set_item("confidence_level", self.portfolio.performance_stats.confidence_level).unwrap();
                py_dict.set_item("risk_free_rate", self.portfolio.performance_stats.risk_free_rate).unwrap();
                py_dict.set_item("portfolio_returns", PyDataFrame(self.portfolio.performance_stats.portfolio_returns.clone())).unwrap();
                py_dict.set_item("benchmark_returns", PySeries(self.portfolio.performance_stats.benchmark_returns.clone())).unwrap();
                py_dict.set_item("objective_function", match self.portfolio.performance_stats.objective_function{
                    ObjectiveFunction::MaxSharpe => "Maximize Sharpe Ratio",
                    ObjectiveFunction::MaxSortino => "Maximize Sortino Ratio",
                    ObjectiveFunction::MinVol => "Minimize Volatility",
                    ObjectiveFunction::MaxReturn => "Maximize Return",
                    ObjectiveFunction::MinDrawdown => "Minimize Drawdown",
                    ObjectiveFunction::MinVar => "Minimize Value at Risk",
                    ObjectiveFunction::MinCVaR => "Minimize Expected Shortfall",
                }).unwrap();
                py_dict.set_item("optimization_method", self.portfolio.performance_stats.optimization_method.clone()).unwrap();
                py_dict.set_item("optimal_weights", self.portfolio.performance_stats.optimal_weights.clone()).unwrap();
                py_dict.set_item("category_weights", self.portfolio.performance_stats.category_weights.clone()).unwrap();
                py_dict.set_item("optimal_portfolio_returns", PySeries(self.portfolio.performance_stats.optimal_portfolio_returns.clone())).unwrap();
                py_dict.set_item("Daily Return", self.portfolio.performance_stats.performance_stats.daily_return).unwrap();
                py_dict.set_item("Daily Volatility", self.portfolio.performance_stats.performance_stats.daily_volatility).unwrap();
                py_dict.set_item("Cumulative Return", self.portfolio.performance_stats.performance_stats.cumulative_return).unwrap();
                py_dict.set_item("Annualized Return", self.portfolio.performance_stats.performance_stats.annualized_return).unwrap();
                py_dict.set_item("Annualized Volatility", self.portfolio.performance_stats.performance_stats.annualized_volatility).unwrap();
                py_dict.set_item("Alpha", self.portfolio.performance_stats.performance_stats.alpha).unwrap();
                py_dict.set_item("Beta", self.portfolio.performance_stats.performance_stats.beta).unwrap();
                py_dict.set_item("Sharpe Ratio", self.portfolio.performance_stats.performance_stats.sharpe_ratio).unwrap();
                py_dict.set_item("Sortino Ratio", self.portfolio.performance_stats.performance_stats.sortino_ratio).unwrap();
                py_dict.set_item("Active Return", self.portfolio.performance_stats.performance_stats.active_return).unwrap();
                py_dict.set_item("Active Risk", self.portfolio.performance_stats.performance_stats.active_risk).unwrap();
                py_dict.set_item("Information Ratio", self.portfolio.performance_stats.performance_stats.information_ratio).unwrap();
                py_dict.set_item("Calmar Ratio", self.portfolio.performance_stats.performance_stats.calmar_ratio).unwrap();
                py_dict.set_item("Maximum Drawdown", self.portfolio.performance_stats.performance_stats.maximum_drawdown).unwrap();
                py_dict.set_item("Value at Risk", self.portfolio.performance_stats.performance_stats.value_at_risk).unwrap();
                py_dict.set_item("Expected Shortfall", self.portfolio.performance_stats.performance_stats.expected_shortfall).unwrap();
                py_dict.into()
            })
        })
    }

    /// display the portfolio optimization chart
    ///
    /// # Arguments
    ///
    /// * `height` - `optional int` - The height of the chart in pixels
    /// * `width` - `optional int` - The width of the chart in pixels
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (height=None, width=None))]
    pub fn optimization_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = self.portfolio.optimization_chart(height, width).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }

    /// display the portfolio performance chart
    ///
    /// # Arguments
    ///
    /// * `height` - `optional int` - The height of the chart in pixels
    /// * `width` - `optional int` - The width of the chart in pixels
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (height=None, width=None))]
    pub fn performance_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = self.portfolio.performance_chart(height, width).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }
    
    /// Compute the performance stats for the portfolio
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the performance statistics for the portfolio and its components
    pub fn performance_stats(&self) -> PyDataFrame {
        task::block_in_place(move || {
            let df = tokio::runtime::Runtime::new().unwrap().block_on(
                self.portfolio.performance_stats_table()
            ).unwrap();
            PyDataFrame(df.data)
        })
    }

    /// display the portfolio asset returns chart
    ///
    /// # Arguments
    ///
    /// * `height` - `int` - The height of the chart in pixels
    /// * `width` - `int` - The width of the chart in pixels
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (height=None, width=None))]
    pub fn asset_returns_chart(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = self.portfolio.returns_chart(height, width).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }

    /// Display the portfolio assets returns matrix
    ///
    /// # Arguments
    ///
    /// * `height` - `int` - The height of the chart in pixels
    /// * `width` - `int` - The width of the chart in pixels
    ///
    /// # Returns
    ///
    /// `Plot` object
    #[pyo3(signature = (height=None, width=None))]
    pub fn returns_matrix(&self, height: Option<usize>, width: Option<usize>) -> PyObject {
        task::block_in_place(move || {
            let plot = self.portfolio.returns_matrix(height, width).unwrap();
            rust_plot_to_py_plot(plot).unwrap()
        })
    }

    /// Displays the analytics report for the portfolio
    ///
    /// # Arguments
    ///
    /// * `report_type` - `optional str` - The type of report to display (performance)
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
                .block_on(self.portfolio.report(Some(report_type)))
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
