use crate::ffi::{error_chain_string, rust_plot_to_py_plot};
use finalytics::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_polars::{PyDataFrame, PySeries};
use std::str::FromStr;
use tokio::task;

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
///     - `risk_parity`: Risk Parity (equal risk contribution)
///     - `max_diversification`: Maximize Diversification ratio
///     - `hierarchical_risk_parity`: Hierarchical Risk Parity
/// * `asset_constraints` - `list` - list of tuples with the lower and upper bounds for the ticker weights
/// * `categorical_constraints` - `list` - list of tuples defining category-based constraints.
///     Each tuple has the form `(category_name: str, category_per_symbol: list[str], weight_per_category: list[tuple[str, float, float]])`
///     where:
///       - `category_name` is the name of the constraint group (e.g., "AssetClass")
///       - `category_per_symbol` assigns each ticker to a category (in the same order as `ticker_symbols`)
///       - `weight_per_category` contains tuples of `(category_label, min_weight, max_weight)`
/// * `weights` - `list` - Dollar amounts for each asset in the portfolio. If provided, it will
///     be used as the initial allocation and skip optimization. The fractional weights are derived
///     as `allocation[i] / sum(allocation)`.
/// * `transactions` - `list` - Optional list of ad-hoc per-asset transactions.
///     Each transaction is a dict with keys: `date` (str), `ticker` (str), `amount` (float).
///     Positive amounts are additions, negative are withdrawals.
/// * `rebalance_strategy` - `dict` - Optional rebalancing strategy. Possible formats:
///     - `{"type": "calendar", "frequency": "monthly"}` — rebalance on a fixed calendar schedule
///     - `{"type": "threshold", "threshold": 0.05}` — rebalance when any weight drifts > 5%
///     - `{"type": "calendar_or_threshold", "frequency": "quarterly", "threshold": 0.05}`
///     Frequency values: "monthly", "quarterly", "semi_annually", "annually"
/// * `scheduled_cash_flows` - `list` - Optional list of recurring cash flow schedules.
///     Each schedule is a dict with keys:
///     - `amount` (float): Dollar amount per occurrence (positive = addition, negative = withdrawal)
///     - `frequency` (str): "monthly", "quarterly", "semi_annually", "annually"
///     - `start_date` (Optional[str]): Start date or None
///     - `end_date` (Optional[str]): End date or None
///     - `allocation` (str or dict): "pro_rata", "rebalance", or {"custom": [0.4, 0.3, 0.2, 0.1]}
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
/// # Optimization example
/// portfolio = finalytics.Portfolio(
///     ticker_symbols = ["AAPL", "GOOG", "MSFT", "ZN=F"],
///     benchmark_symbol = "^GSPC",
///     start_date = "2020-01-01",
///     end_date = "2021-01-01",
///     interval = "1d",
///     confidence_level = 0.95,
///     risk_free_rate = 0.02,
///     objective_function = "max_sharpe",
///     asset_constraints = [(0.0, 0.5), (0.0, 0.5), (0.0, 0.5), (0.0, 0.5)],
///     categorical_constraints=[(
///         "AssetClass",
///         ["EQUITY", "EQUITY", "EQUITY", "FIXED INCOME"],
///         [("EQUITY", 0.0, 0.8), ("FIXED INCOME", 0.0, 0.2)]
///     )]
/// )
///
/// # Explicit allocation with rebalancing and DCA
/// portfolio = finalytics.Portfolio(
///     ticker_symbols = ["AAPL", "MSFT", "NVDA", "BTC-USD"],
///     benchmark_symbol = "^GSPC",
///     start_date = "2023-01-01",
///     end_date = "2024-12-31",
///     interval = "1d",
///     confidence_level = 0.95,
///     risk_free_rate = 0.02,
///     weights = [25000.0, 25000.0, 25000.0, 25000.0],
///     rebalance_strategy = {"type": "calendar", "frequency": "quarterly"},
///     scheduled_cash_flows = [
///         {
///             "amount": 2000.0,
///             "frequency": "monthly",
///             "start_date": None,
///             "end_date": None,
///             "allocation": "pro_rata"
///         }
///     ]
/// )
/// ```
#[pyclass]
#[pyo3(name = "Portfolio")]
pub struct PyPortfolio {
    pub portfolio: Portfolio,
}

fn parse_rebalance_strategy_py(dict: &Bound<'_, PyDict>) -> Option<RebalanceStrategy> {
    let strategy_type: String = dict.get_item("type").ok()??.extract().ok()?;

    let parse_frequency = |d: &Bound<'_, PyDict>| -> Option<ScheduleFrequency> {
        let freq: String = d.get_item("frequency").ok()??.extract().ok()?;
        match freq.as_str() {
            "monthly" => Some(ScheduleFrequency::Monthly),
            "quarterly" => Some(ScheduleFrequency::Quarterly),
            "semi_annually" => Some(ScheduleFrequency::SemiAnnually),
            "annually" => Some(ScheduleFrequency::Annually),
            _ => None,
        }
    };

    match strategy_type.as_str() {
        "calendar" => {
            let freq = parse_frequency(dict)?;
            Some(RebalanceStrategy::Calendar(freq))
        }
        "threshold" => {
            let threshold: f64 = dict.get_item("threshold").ok()??.extract().ok()?;
            Some(RebalanceStrategy::Threshold(threshold))
        }
        "calendar_or_threshold" => {
            let freq = parse_frequency(dict)?;
            let threshold: f64 = dict.get_item("threshold").ok()??.extract().ok()?;
            Some(RebalanceStrategy::CalendarOrThreshold(freq, threshold))
        }
        _ => None,
    }
}

fn parse_transactions_py(list: Vec<Bound<'_, PyDict>>) -> Vec<Transaction> {
    list.iter()
        .filter_map(|dict| {
            let date: String = dict.get_item("date").ok()??.extract().ok()?;
            let ticker: String = dict.get_item("ticker").ok()??.extract().ok()?;
            let amount: f64 = dict.get_item("amount").ok()??.extract().ok()?;
            Some(Transaction {
                date,
                ticker,
                amount,
            })
        })
        .collect()
}

fn parse_allocation_py(value: &Bound<'_, pyo3::PyAny>) -> CashFlowAllocation {
    if let Ok(s) = value.extract::<String>() {
        match s.as_str() {
            "pro_rata" => CashFlowAllocation::ProRata,
            "rebalance" => CashFlowAllocation::Rebalance,
            _ => CashFlowAllocation::ProRata,
        }
    } else if let Ok(dict) = value.downcast::<PyDict>() {
        if let Ok(Some(custom)) = dict.get_item("custom") {
            if let Ok(weights) = custom.extract::<Vec<f64>>() {
                return CashFlowAllocation::Custom(weights);
            }
        }
        CashFlowAllocation::ProRata
    } else {
        CashFlowAllocation::ProRata
    }
}

fn parse_scheduled_cash_flows_py(list: Vec<Bound<'_, PyDict>>) -> Vec<ScheduledCashFlow> {
    list.iter()
        .filter_map(|dict| {
            let amount: f64 = dict.get_item("amount").ok()??.extract().ok()?;
            let freq_str: String = dict.get_item("frequency").ok()??.extract().ok()?;
            let frequency = match freq_str.as_str() {
                "monthly" => ScheduleFrequency::Monthly,
                "quarterly" => ScheduleFrequency::Quarterly,
                "semi_annually" => ScheduleFrequency::SemiAnnually,
                "annually" => ScheduleFrequency::Annually,
                _ => return None,
            };
            let start_date: Option<String> = dict
                .get_item("start_date")
                .ok()
                .flatten()
                .and_then(|v| v.extract().ok());
            let end_date: Option<String> = dict
                .get_item("end_date")
                .ok()
                .flatten()
                .and_then(|v| v.extract().ok());
            let allocation = dict
                .get_item("allocation")
                .ok()
                .flatten()
                .map(|v| parse_allocation_py(&v))
                .unwrap_or(CashFlowAllocation::ProRata);

            Some(ScheduledCashFlow {
                amount,
                frequency,
                start_date,
                end_date,
                allocation,
            })
        })
        .collect()
}

#[pymethods]
impl PyPortfolio {
    #[new]
    #[pyo3(signature = (
    ticker_symbols, benchmark_symbol=None, start_date=None, end_date=None, interval=None,
    confidence_level=None, risk_free_rate=None, objective_function=None, asset_constraints=None,
    categorical_constraints=None, weights=None, tickers_data=None, benchmark_data=None,
    transactions=None, rebalance_strategy=None, scheduled_cash_flows=None))]
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    pub fn new(
        ticker_symbols: Vec<String>,
        benchmark_symbol: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        interval: Option<String>,
        confidence_level: Option<f64>,
        risk_free_rate: Option<f64>,
        objective_function: Option<String>,
        asset_constraints: Option<Vec<(f64, f64)>>,
        categorical_constraints: Option<Vec<(String, Vec<String>, Vec<(String, f64, f64)>)>>,
        weights: Option<Vec<f64>>,
        tickers_data: Option<Vec<PyDataFrame>>,
        benchmark_data: Option<PyDataFrame>,
        transactions: Option<Vec<Bound<'_, PyDict>>>,
        rebalance_strategy: Option<Bound<'_, PyDict>>,
        scheduled_cash_flows: Option<Vec<Bound<'_, PyDict>>>,
    ) -> PyResult<Self> {
        let ticker_symbols: Vec<&str> = ticker_symbols.iter().map(|x| x.as_str()).collect();
        let tickers_data = match tickers_data {
            Some(data) => {
                let mut klines = Vec::new();
                for (symbol, df) in ticker_symbols.clone().into_iter().zip(data) {
                    let kline = KLINE::from_dataframe(symbol, &df.0).map_err(|e| {
                        PyRuntimeError::new_err(format!(
                            "Failed to parse ticker data: {}",
                            error_chain_string(&*e)
                        ))
                    })?;
                    klines.push(kline);
                }
                Some(klines)
            }
            None => None,
        };
        let benchmark_data = match benchmark_data {
            Some(df) => {
                let name = benchmark_symbol.clone().unwrap_or("Benchmark".to_string());
                let kline = KLINE::from_dataframe(&name, &df.0).map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to parse benchmark data: {}",
                        error_chain_string(&*e)
                    ))
                })?;
                Some(kline)
            }
            None => None,
        };
        let default_start = chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::days(365))
            .unwrap() // infallible: 365 days is always valid
            .format("%Y-%m-%d")
            .to_string();
        let default_end = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let interval_str = interval.unwrap_or("1d".to_string());
        let interval = Interval::from_str(&interval_str)
            .map_err(|_| PyRuntimeError::new_err(format!("Invalid interval: '{interval_str}'")))?;
        let obj_str = objective_function.unwrap_or("max_sharpe".to_string());
        let objective_function = ObjectiveFunction::from_str(&obj_str).map_err(|_| {
            PyRuntimeError::new_err(format!("Invalid objective function: '{obj_str}'"))
        })?;
        let constraints = Constraints {
            asset_weights: asset_constraints,
            categorical_weights: categorical_constraints.map(|cats| {
                cats.into_iter()
                    .map(
                        |(name, category_per_symbol, weight_per_category)| CategoricalWeights {
                            name,
                            category_per_symbol,
                            weight_per_category,
                        },
                    )
                    .collect()
            }),
        };

        // Parse new portfolio features
        let parsed_transactions: Option<Vec<Transaction>> = transactions.map(parse_transactions_py);
        let parsed_rebalance_strategy: Option<RebalanceStrategy> =
            rebalance_strategy.and_then(|d| parse_rebalance_strategy_py(&d));
        let parsed_scheduled_cash_flows: Option<Vec<ScheduledCashFlow>> =
            scheduled_cash_flows.map(parse_scheduled_cash_flows_py);

        let start_date = start_date.unwrap_or(default_start);
        let end_date = end_date.unwrap_or(default_end);

        task::block_in_place(move || {
            let rt = tokio::runtime::Runtime::new().map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to create async runtime: {e}"))
            })?;

            let mut portfolio = rt
                .block_on({
                    let mut builder = Portfolio::builder()
                        .ticker_symbols(ticker_symbols)
                        .start_date(&start_date)
                        .end_date(&end_date)
                        .interval(interval)
                        .confidence_level(confidence_level.unwrap_or(0.95))
                        .risk_free_rate(risk_free_rate.unwrap_or(0.02))
                        .objective_function(objective_function)
                        .constraints(Some(constraints))
                        .tickers_data(tickers_data)
                        .benchmark_data(benchmark_data);
                    if let Some(ref sym) = benchmark_symbol {
                        builder = builder.benchmark_symbol(sym);
                    }
                    if let Some(w) = weights.clone() {
                        builder = builder.weights(w);
                    }
                    if let Some(txns) = parsed_transactions {
                        builder = builder.transactions(txns);
                    }
                    if parsed_rebalance_strategy.is_some() {
                        builder = builder.rebalance_strategy(parsed_rebalance_strategy);
                    }
                    if parsed_scheduled_cash_flows.is_some() {
                        builder = builder.scheduled_cash_flows(parsed_scheduled_cash_flows);
                    }
                    builder.build()
                })
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to build portfolio: {}",
                        error_chain_string(&*e)
                    ))
                })?;

            // If weights are provided, evaluate directly (no optimization).
            // Otherwise, optimize (which also computes in-sample performance stats).
            if weights.is_some() {
                portfolio.performance_stats().map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to compute performance stats: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            } else {
                portfolio.optimize().map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to optimize portfolio: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            }

            Ok(PyPortfolio { portfolio })
        })
    }

    /// Get the portfolio optimization results
    ///
    /// # Returns
    ///
    /// `dict` - dictionary of optimization results including:
    ///   - ticker_symbols, benchmark_symbol, start_date, end_date, interval
    ///   - confidence_level, risk_free_rate
    ///   - portfolio_returns (DataFrame), benchmark_returns (Series)
    ///   - objective_function, optimization_method, optimal_weights
    ///   - category_weights, risk_contributions
    ///   - weights, starting_weights, ending_weights
    ///   - starting_values, ending_values
    ///   - optimal_portfolio_returns (Series)
    ///   - Performance metrics: Daily Return, Daily Volatility, Cumulative Return, etc.
    ///   - Money Weighted Return (annualized %, if applicable)
    pub fn optimization_results(&self) -> PyResult<PyObject> {
        task::block_in_place(move || {
            Python::with_gil(|py| {
                let py_dict = PyDict::new(py);
                let data = &self.portfolio.data;

                py_dict.set_item("ticker_symbols", data.ticker_symbols.clone())?;
                py_dict.set_item("benchmark_symbol", data.benchmark_symbol.clone())?;
                py_dict.set_item("start_date", data.start_date.clone())?;
                py_dict.set_item("end_date", data.end_date.clone())?;
                py_dict.set_item("interval", data.interval.mode)?;
                py_dict.set_item("confidence_level", data.confidence_level)?;
                py_dict.set_item("risk_free_rate", data.risk_free_rate)?;
                py_dict.set_item(
                    "portfolio_returns",
                    PyDataFrame(data.portfolio_returns.clone()),
                )?;
                match &data.benchmark_returns {
                    Some(br) => py_dict.set_item("benchmark_returns", PySeries(br.clone()))?,
                    None => py_dict.set_item("benchmark_returns", py.None())?,
                };

                // Optimization result fields
                if let Some(ref opt) = self.portfolio.optimization_result {
                    py_dict.set_item(
                        "objective_function",
                        match opt.objective_function {
                            ObjectiveFunction::MaxSharpe => "Maximize Sharpe Ratio",
                            ObjectiveFunction::MaxSortino => "Maximize Sortino Ratio",
                            ObjectiveFunction::MinVol => "Minimize Volatility",
                            ObjectiveFunction::MaxReturn => "Maximize Return",
                            ObjectiveFunction::MinDrawdown => "Minimize Drawdown",
                            ObjectiveFunction::MinVar => "Minimize Value at Risk",
                            ObjectiveFunction::MinCVaR => "Minimize Expected Shortfall",
                            ObjectiveFunction::RiskParity => "Risk Parity",
                            ObjectiveFunction::MaxDiversification => "Maximize Diversification",
                            ObjectiveFunction::HierarchicalRiskParity => "Hierarchical Risk Parity",
                        },
                    )?;
                    py_dict.set_item("optimization_method", opt.optimization_method.clone())?;
                    py_dict.set_item("optimal_weights", opt.optimal_weights.clone())?;
                    py_dict.set_item("category_weights", opt.category_weights.clone())?;
                    py_dict.set_item("efficient_frontier", opt.efficient_frontier.clone())?;
                    py_dict.set_item("risk_contributions", opt.risk_contributions.clone())?;
                }

                // Performance stats fields
                if let Some(ref perf) = self.portfolio.performance_stats {
                    let s = &perf.performance_stats;
                    py_dict.set_item("weights", perf.weights.clone())?;
                    py_dict.set_item("starting_weights", perf.starting_weights.clone())?;
                    py_dict.set_item("ending_weights", perf.ending_weights.clone())?;
                    py_dict.set_item("starting_values", perf.starting_values.clone())?;
                    py_dict.set_item("ending_values", perf.ending_values.clone())?;
                    py_dict.set_item(
                        "optimal_portfolio_returns",
                        PySeries(perf.portfolio_returns.clone()),
                    )?;
                    py_dict.set_item("Daily Return", s.daily_return)?;
                    py_dict.set_item("Daily Volatility", s.daily_volatility)?;
                    py_dict.set_item("Cumulative Return", s.cumulative_return)?;
                    py_dict.set_item("Annualized Return", s.annualized_return)?;
                    py_dict.set_item("Annualized Volatility", s.annualized_volatility)?;
                    py_dict.set_item("Alpha", s.alpha)?;
                    py_dict.set_item("Beta", s.beta)?;
                    py_dict.set_item("Sharpe Ratio", s.sharpe_ratio)?;
                    py_dict.set_item("Sortino Ratio", s.sortino_ratio)?;
                    py_dict.set_item("Active Return", s.active_return)?;
                    py_dict.set_item("Active Risk", s.active_risk)?;
                    py_dict.set_item("Information Ratio", s.information_ratio)?;
                    py_dict.set_item("Calmar Ratio", s.calmar_ratio)?;
                    py_dict.set_item("Maximum Drawdown", s.maximum_drawdown)?;
                    py_dict.set_item("Value at Risk", s.value_at_risk)?;
                    py_dict.set_item("Expected Shortfall", s.expected_shortfall)?;
                    py_dict.set_item("Money Weighted Return", perf.money_weighted_return)?;
                }

                Ok(py_dict.into())
            })
        })
    }

    /// Display the portfolio optimization chart
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
    pub fn optimization_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let plot = self
                .portfolio
                .optimization_chart(height, width)
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to generate optimization chart: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert optimization chart plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Display the portfolio performance chart
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
    pub fn performance_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let plot = self
                .portfolio
                .performance_chart(height, width)
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

    /// Compute the performance stats for the portfolio.
    ///
    /// This always recomputes stats before returning the table, so it can be
    /// called directly after `update_dates()` without any intermediate step.
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the performance statistics for the portfolio and its components
    pub fn performance_stats(&mut self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            // Recompute stats (mirrors Rust's `portfolio.performance_stats()`)
            self.portfolio.performance_stats().map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to compute performance stats: {}",
                    error_chain_string(&*e)
                ))
            })?;
            // Format the (now-fresh) stats into a table (sync call)
            let df = self.portfolio.performance_stats_table().map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to build performance stats table: {}",
                    error_chain_string(&*e)
                ))
            })?;
            Ok(PyDataFrame(df.data))
        })
    }

    /// Display the portfolio asset returns chart (percentage returns)
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
    pub fn asset_returns_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let plot = self.portfolio.returns_chart(height, width).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to generate asset returns chart: {}",
                    error_chain_string(&*e)
                ))
            })?;
            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert asset returns chart plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Display the portfolio value over time chart
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
    pub fn portfolio_value_chart(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let plot = self
                .portfolio
                .portfolio_value_chart(height, width)
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to generate portfolio value chart: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert portfolio value chart plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Display the portfolio assets returns correlation matrix
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
    pub fn returns_matrix(
        &self,
        height: Option<usize>,
        width: Option<usize>,
    ) -> PyResult<PyObject> {
        task::block_in_place(move || {
            let plot = self.portfolio.returns_matrix(height, width).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to generate returns matrix: {}",
                    error_chain_string(&*e)
                ))
            })?;
            rust_plot_to_py_plot(plot).map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to convert returns matrix plot: {}",
                    e.to_string()
                ))
            })
        })
    }

    /// Get the transaction history table
    ///
    /// Returns a table of all transaction events during the simulation, including
    /// rebalances, cash flows, and combined events. Each row includes:
    /// portfolio value before/after, per-asset values, trade amounts, turnover,
    /// cumulative TWR and MWR.
    ///
    /// # Returns
    ///
    /// `DataFrame` - A Polars DataFrame containing the transaction history
    pub fn transaction_history_table(&self) -> PyResult<PyDataFrame> {
        task::block_in_place(move || {
            let table = self
                .portfolio
                .transaction_history_table()
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to get transaction history: {}",
                        error_chain_string(&*e)
                    ))
                })?
                .ok_or_else(|| {
                    PyRuntimeError::new_err("No transaction history available for this portfolio")
                })?;
            Ok(PyDataFrame(table.data))
        })
    }

    /// Update the portfolio's date range and re-fetch data for out-of-sample evaluation.
    ///
    /// This method is for portfolios built from Yahoo Finance data (not custom data).
    /// It rebuilds all underlying ticker and benchmark data for the new date range.
    /// The optimization result (weights) is preserved so they can be evaluated
    /// out-of-sample on the new period.
    ///
    /// After calling this method, call `performance_stats()` to evaluate the
    /// optimized weights on the new data (it recomputes automatically).
    ///
    /// # Arguments
    ///
    /// * `start_date` - `str` - New start date (e.g. "2024-01-01")
    /// * `end_date` - `str` - New end date (e.g. "2024-12-31")
    ///
    /// # Example
    ///
    /// ```python
    /// portfolio = Portfolio(
    ///     ticker_symbols=["AAPL", "MSFT"],
    ///     start_date="2022-01-01",
    ///     end_date="2023-01-01",
    ///     objective_function="max_sharpe"
    /// )
    /// # Optimize on 2022 data
    /// print(portfolio.optimization_results())
    ///
    /// # Update to 2023 data for out-of-sample evaluation
    /// portfolio.update_dates("2023-01-01", "2024-01-01")
    /// print(portfolio.performance_stats())
    /// ```
    pub fn update_dates(&mut self, start_date: &str, end_date: &str) -> PyResult<()> {
        task::block_in_place(move || {
            let rt = tokio::runtime::Runtime::new().map_err(|e| {
                PyRuntimeError::new_err(format!(
                    "Failed to create async runtime: {}",
                    error_chain_string(&e)
                ))
            })?;
            rt.block_on(self.portfolio.update_dates(start_date, end_date))
                .map_err(|e| {
                    PyRuntimeError::new_err(format!(
                        "Failed to update dates: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            Ok(())
        })
    }

    /// Displays the analytics report for the portfolio
    ///
    /// # Arguments
    ///
    /// * `report_type` - `optional str` - The type of report to display:
    ///     - "performance" - Performance analysis report
    ///     - "optimization" - Optimization results report
    /// * `display` - Optional str - Display mode ("notebook" to display in Jupyter, else displays to default web browser)
    #[pyo3(signature = (report_type=None, display=None))]
    pub fn report(&self, report_type: Option<String>, display: Option<String>) -> PyResult<()> {
        task::block_in_place(move || {
            let report_type = match report_type {
                Some(report_type) => ReportType::from_str(&report_type)
                    .map_err(|e| PyRuntimeError::new_err(format!("Invalid report type: {e}")))?,
                None => ReportType::Performance,
            };
            let rt = tokio::runtime::Runtime::new().map_err(|e| {
                PyRuntimeError::new_err(format!("Failed to create async runtime: {e}"))
            })?;
            let report = rt
                .block_on(self.portfolio.report(Some(report_type)))
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
                        "Failed to show report: {}",
                        error_chain_string(&*e)
                    ))
                })?;
            }
            Ok(())
        })
    }
}
