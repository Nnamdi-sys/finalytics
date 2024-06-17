use polars::export::chrono;
use finalytics::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::task;
use crate::ffi::{rust_df_to_py_df, rust_plot_to_py_plot, rust_series_to_py_series};


#[pyclass]
#[pyo3(name = "Portfolio")]
pub struct PyPortfolio {
    pub portfolio: Portfolio
}

#[pymethods]
impl PyPortfolio {
    #[new]
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
    /// * `risk_free_rate` - `float` - The risk free rate to use in the calculations
    /// * `max_iterations` - `int` - The maximum number of iterations to use in the optimization
    /// * `objective_function` - `str` - The objective function to use in the optimization (max_sharpe, min_vol, max_return, nin_var, min_cvar, min_drawdown)
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
    ///                                  max_iterations = 1000,
    ///                                  objective_function = "max_sharpe")
    /// ```
    pub fn new(ticker_symbols: Vec<&str>, benchmark_symbol: Option<String>, start_date: Option<String>, end_date: Option<String>,
               interval: Option<String>, confidence_level: Option<f64>, risk_free_rate: Option<f64>,
               max_iterations: Option<u64>, objective_function: Option<String>) -> Self {
        let default_start = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(365))
            .unwrap().format("%Y-%m-%d").to_string();
        let defualt_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval = Interval::from_str(&interval.unwrap_or("1d".to_string()));
        let objective_function = ObjectiveFunction::from_str(&objective_function.unwrap_or("max_sharpe".to_string()));
        task::block_in_place(move || {
            let portfolio = tokio::runtime::Runtime::new().unwrap().block_on(
                PortfolioBuilder::new()
                    .ticker_symbols(ticker_symbols)
                    .benchmark_symbol(&benchmark_symbol.unwrap_or("^GSPC".to_string()))
                    .start_date(&start_date.unwrap_or(default_start))
                    .end_date(&end_date.unwrap_or(defualt_end))
                    .interval(interval)
                    .confidence_level(confidence_level.unwrap_or(0.95))
                    .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                    .max_iterations(max_iterations.unwrap_or(1000))
                    .objective_function(objective_function)
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
    pub fn get_optimization_results(&self) -> PyObject {
        task::block_in_place(move || {
            Python::with_gil(|py| {
                let py_dict = PyDict::new(py);
                py_dict.set_item("ticker_symbols", self.portfolio.performance_stats.ticker_symbols.clone()).unwrap();
                py_dict.set_item("benchmark_symbol", self.portfolio.performance_stats.benchmark_symbol.clone()).unwrap();
                py_dict.set_item("start_date", self.portfolio.performance_stats.start_date.clone()).unwrap();
                py_dict.set_item("end_date", self.portfolio.performance_stats.end_date.clone()).unwrap();
                py_dict.set_item("interval", self.portfolio.performance_stats.interval.to_string()).unwrap();
                py_dict.set_item("confidence_level", self.portfolio.performance_stats.confidence_level).unwrap();
                py_dict.set_item("risk_free_rate", self.portfolio.performance_stats.risk_free_rate).unwrap();
                py_dict.set_item("portfolio_returns", rust_df_to_py_df(&self.portfolio.performance_stats.portfolio_returns).unwrap()).unwrap();
                py_dict.set_item("benchmark_returns", rust_series_to_py_series(&self.portfolio.performance_stats.benchmark_returns).unwrap()).unwrap();
                py_dict.set_item("objective_function", match self.portfolio.performance_stats.objective_function{
                    ObjectiveFunction::MaxSharpe => "Maximize Sharpe Ratio",
                    ObjectiveFunction::MinVol => "Minimize Volatility",
                    ObjectiveFunction::MaxReturn => "Maximize Return",
                    ObjectiveFunction::MinDrawdown => "Minimize Drawdown",
                    ObjectiveFunction::MinVar => "Minimize Value at Risk",
                    ObjectiveFunction::MinCVaR => "Minimize Expected Shortfall",
                }).unwrap();
                py_dict.set_item("optimization_method", self.portfolio.performance_stats.optimization_method.clone()).unwrap();
                py_dict.set_item("max_iterations", self.portfolio.performance_stats.max_iterations).unwrap();
                py_dict.set_item("optimal_weights", self.portfolio.performance_stats.optimal_weights.clone()).unwrap();
                py_dict.set_item("optimal_portfolio_returns", rust_series_to_py_series(&self.portfolio.performance_stats.optimal_portfolio_returns).unwrap()).unwrap();
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

    /// display the portfolio optimization charts
    ///
    /// # Arguments
    ///
    /// * `chart_type` - `str` - The type of chart to display (optimization, performance, asset_returns)
    /// * `height` - `int` - The height of the chart in pixels
    /// * `width` - `int` - The width of the chart in pixels
    ///
    /// # Returns
    ///
    /// `Plot` object
    pub fn portfolio_chart(&self, chart_type: String, height: Option<usize>, width: Option<usize>) -> PyObject {
        let height = height.unwrap_or(800);
        let width = width.unwrap_or(1200);
        task::block_in_place(move || {

            let plot = match chart_type.as_str() {
                "optimization" => self.portfolio.optimization_chart(height, width).unwrap(),
                "performance" => self.portfolio.performance_chart(height, width).unwrap(),
                "asset_returns" => self.portfolio.asset_returns_chart(height, width).unwrap(),
                _ => panic!("chart_type must be one of: optimization, performance, asset_returns")
            };

           rust_plot_to_py_plot(plot).unwrap()
        })
    }
}
