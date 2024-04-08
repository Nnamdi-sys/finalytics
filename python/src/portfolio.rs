use finalytics::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use tokio::task;
use crate::ffi::{display_html_with_iframe, rust_df_to_py_df, rust_series_to_py_series};


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
    /// portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT", "ZN=F"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
    /// ```
    pub fn new(ticker_symbols: Vec<&str>, benchmark_symbol: String, start_date: String, end_date: String,
               interval: String, confidence_level: f64, risk_free_rate: f64, max_iterations: u64, objective_function: String) -> Self {
        task::block_in_place(move || {
            let interval = Interval::from_str(&interval);
            let objective_function = ObjectiveFunction::from_str(&objective_function);
            let portfolio = tokio::runtime::Runtime::new().unwrap().block_on(
                PortfolioBuilder::new().ticker_symbols(ticker_symbols).benchmark_symbol(&benchmark_symbol)
                    .start_date(&start_date).end_date(&end_date).interval(interval)
                    .confidence_level(confidence_level).risk_free_rate(risk_free_rate)
                    .max_iterations(max_iterations).objective_function(objective_function)
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
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "SPY", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
    /// optimization_results = portfolio.get_optimization_results()
    /// ```
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
    /// * `display_format` - `str` - The format to display the charts in (html, png, notebook)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
    /// portfolio.display_portfolio_charts("performance", "html")
    /// ```
    pub fn display_portfolio_charts(&self, chart_type: String, display_format: String) {
        task::block_in_place(move || {

            let chart = match chart_type.as_str() {
                "optimization" => self.portfolio.optimization_chart().unwrap(),
                "performance" => self.portfolio.performance_chart().unwrap(),
                "asset_returns" => self.portfolio.asset_returns_chart().unwrap(),
                _ => panic!("chart_type must be one of: optimization, performance, asset_returns")
            };

            match display_format.as_str() {
                "html" => {
                    chart.write_html(&format!("{}.html", chart_type));
                    println!("chart written to {}.html", chart_type);
                },
                "png" => {
                    chart.to_png(&format!("{}.html", chart_type),  1500, 1200, 1.0);
                    println!("chart written to {}.png", chart_type);
                },
                "notebook" => {
                    if let Err(err) = display_html_with_iframe(Some(chart), &chart_type) {
                        eprintln!("Error displaying HTML with iframe: {:?}", err);
                    }
                },
                _ => panic!("display_format must be one of: html, png, notebook or colab")
            }
        })
    }
}
