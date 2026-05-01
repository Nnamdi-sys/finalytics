use crate::utils::{dataframe_from_json, dataframe_to_json, series_to_json};
use crate::{catch_panic, set_last_error, set_last_error_from_err};
use finalytics::prelude::*;
use serde_json::json;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::{c_int, c_uint};
use std::str::FromStr;
use tokio::runtime::Runtime;

// Opaque handle for Portfolio
pub type PortfolioHandle = *mut Portfolio;

// Helper to convert Rust string to C string
fn to_c_string(s: String) -> *mut c_char {
    CString::new(s)
        .unwrap_or_else(|_| CString::new("(string contained NUL byte)").unwrap())
        .into_raw()
}

/// Helper to create a tokio runtime, setting last error on failure.
fn make_runtime() -> Result<Runtime, ()> {
    Runtime::new().map_err(|e| {
        set_last_error(format!("Failed to create async runtime: {e}"));
    })
}

/// Parse a JSON string into a `RebalanceStrategy`.
///
/// Expected JSON formats:
///   - `{"type":"calendar","frequency":"monthly"}`
///   - `{"type":"threshold","threshold":0.05}`
///   - `{"type":"calendar_or_threshold","frequency":"quarterly","threshold":0.05}`
///
/// Frequency values: "monthly", "quarterly", "semi_annually", "annually"
fn parse_rebalance_strategy(json_str: &str) -> Option<RebalanceStrategy> {
    let v: serde_json::Value = serde_json::from_str(json_str).ok()?;
    let strategy_type = v.get("type")?.as_str()?;

    let parse_frequency = |v: &serde_json::Value| -> Option<ScheduleFrequency> {
        match v.get("frequency")?.as_str()? {
            "monthly" => Some(ScheduleFrequency::Monthly),
            "quarterly" => Some(ScheduleFrequency::Quarterly),
            "semi_annually" => Some(ScheduleFrequency::SemiAnnually),
            "annually" => Some(ScheduleFrequency::Annually),
            _ => None,
        }
    };

    match strategy_type {
        "calendar" => {
            let freq = parse_frequency(&v)?;
            Some(RebalanceStrategy::Calendar(freq))
        }
        "threshold" => {
            let threshold = v.get("threshold")?.as_f64()?;
            Some(RebalanceStrategy::Threshold(threshold))
        }
        "calendar_or_threshold" => {
            let freq = parse_frequency(&v)?;
            let threshold = v.get("threshold")?.as_f64()?;
            Some(RebalanceStrategy::CalendarOrThreshold(freq, threshold))
        }
        _ => None,
    }
}

/// Parse a JSON string into a `Vec<Transaction>`.
///
/// Expected JSON format:
/// ```json
/// [
///   {"date": "2024-01-15", "ticker": "AAPL", "amount": 5000.0},
///   {"date": "2024-06-01", "ticker": "MSFT", "amount": -2000.0}
/// ]
/// ```
fn parse_transactions(json_str: &str) -> Option<Vec<Transaction>> {
    let v: serde_json::Value = serde_json::from_str(json_str).ok()?;
    let arr = v.as_array()?;
    let mut transactions = Vec::new();
    for item in arr {
        let date = item.get("date")?.as_str()?.to_string();
        let ticker = item.get("ticker")?.as_str()?.to_string();
        let amount = item.get("amount")?.as_f64()?;
        transactions.push(Transaction {
            date,
            ticker,
            amount,
        });
    }
    Some(transactions)
}

/// Parse a JSON string into a `Vec<ScheduledCashFlow>`.
///
/// Expected JSON format:
/// ```json
/// [
///   {
///     "amount": 2000.0,
///     "frequency": "monthly",
///     "start_date": "2024-01-01",
///     "end_date": null,
///     "allocation": "pro_rata"
///   },
///   {
///     "amount": -5000.0,
///     "frequency": "quarterly",
///     "start_date": null,
///     "end_date": "2025-12-31",
///     "allocation": "rebalance"
///   },
///   {
///     "amount": 1000.0,
///     "frequency": "annually",
///     "start_date": null,
///     "end_date": null,
///     "allocation": {"custom": [0.4, 0.3, 0.2, 0.1]}
///   }
/// ]
/// ```
fn parse_scheduled_cash_flows(json_str: &str) -> Option<Vec<ScheduledCashFlow>> {
    let v: serde_json::Value = serde_json::from_str(json_str).ok()?;
    let arr = v.as_array()?;
    let mut flows = Vec::new();
    for item in arr {
        let amount = item.get("amount")?.as_f64()?;
        let frequency = match item.get("frequency")?.as_str()? {
            "monthly" => ScheduleFrequency::Monthly,
            "quarterly" => ScheduleFrequency::Quarterly,
            "semi_annually" => ScheduleFrequency::SemiAnnually,
            "annually" => ScheduleFrequency::Annually,
            _ => return None,
        };
        let start_date = item
            .get("start_date")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let end_date = item
            .get("end_date")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let allocation = match item.get("allocation") {
            Some(serde_json::Value::String(s)) => match s.as_str() {
                "pro_rata" => CashFlowAllocation::ProRata,
                "rebalance" => CashFlowAllocation::Rebalance,
                _ => CashFlowAllocation::ProRata,
            },
            Some(serde_json::Value::Object(obj)) => {
                if let Some(custom) = obj.get("custom") {
                    let weights: Vec<f64> = custom
                        .as_array()?
                        .iter()
                        .filter_map(|v| v.as_f64())
                        .collect();
                    CashFlowAllocation::Custom(weights)
                } else {
                    CashFlowAllocation::ProRata
                }
            }
            _ => CashFlowAllocation::ProRata,
        };
        flows.push(ScheduledCashFlow {
            amount,
            frequency,
            start_date,
            end_date,
            allocation,
        });
    }
    Some(flows)
}

// Create a new Portfolio
#[no_mangle]
pub extern "C" fn finalytics_portfolio_new(
    ticker_symbols: *const c_char,
    benchmark_symbol: *const c_char,
    start_date: *const c_char,
    end_date: *const c_char,
    interval: *const c_char,
    confidence_level: f64,
    risk_free_rate: f64,
    objective_function: *const c_char,
    asset_constraints: *const c_char,
    categorical_constraints: *const c_char,
    weights: *const c_char,
    tickers_data: *const c_char,
    benchmark_data: *const c_char,
    transactions: *const c_char,
    rebalance_strategy: *const c_char,
    scheduled_cash_flows: *const c_char,
) -> PortfolioHandle {
    let result = catch_panic(std::panic::AssertUnwindSafe(|| {
        let ticker_symbols_str = unsafe { CStr::from_ptr(ticker_symbols).to_str().unwrap_or("[]") };
        let benchmark_symbol = unsafe {
            if benchmark_symbol.is_null() {
                None
            } else {
                let s = CStr::from_ptr(benchmark_symbol).to_str().unwrap_or("");
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }
        };
        let start_date = unsafe { CStr::from_ptr(start_date).to_str().unwrap_or("") };
        let end_date = unsafe { CStr::from_ptr(end_date).to_str().unwrap_or("") };
        let interval_str = unsafe { CStr::from_ptr(interval).to_str().unwrap_or("1d") };
        let objective_function_str = unsafe {
            CStr::from_ptr(objective_function)
                .to_str()
                .unwrap_or("max_sharpe")
        };
        let asset_constraints_str =
            unsafe { CStr::from_ptr(asset_constraints).to_str().unwrap_or("[]") };
        let categorical_constraints_str = unsafe {
            CStr::from_ptr(categorical_constraints)
                .to_str()
                .unwrap_or("[]")
        };
        let weights_str = unsafe { CStr::from_ptr(weights).to_str().unwrap_or("[]") };
        let ticker_symbols: Vec<String> =
            serde_json::from_str(ticker_symbols_str).unwrap_or_default();
        let ticker_symbols_ref: Vec<&str> = ticker_symbols.iter().map(|s| s.as_str()).collect();
        let tickers_data = unsafe {
            if tickers_data.is_null() {
                None
            } else {
                let tickers_data_str = CStr::from_ptr(tickers_data).to_str().unwrap_or("[]");
                let tickers_data_vec: Vec<String> = match serde_json::from_str(tickers_data_str) {
                    Ok(v) => v,
                    Err(e) => {
                        set_last_error(format!("Failed to parse tickers data JSON array: {e}"));
                        return std::ptr::null_mut();
                    }
                };
                let mut klines = Vec::new();
                for (i, s) in tickers_data_vec.iter().enumerate() {
                    let df = match dataframe_from_json(s) {
                        Ok(df) => df,
                        Err(e) => {
                            let sym = ticker_symbols_ref.get(i).unwrap_or(&"unknown");
                            set_last_error(format!(
                                "Failed to parse ticker data JSON for '{sym}': {e}"
                            ));
                            return std::ptr::null_mut();
                        }
                    };
                    let sym = ticker_symbols_ref.get(i).unwrap_or(&"unknown");
                    match KLINE::from_dataframe(sym, &df) {
                        Ok(kline) => klines.push(kline),
                        Err(e) => {
                            set_last_error_from_err(
                                &format!("Failed to build KLINE for '{sym}'"),
                                &*e,
                            );
                            return std::ptr::null_mut();
                        }
                    }
                }
                Some(klines)
            }
        };
        let benchmark_data = unsafe {
            if benchmark_data.is_null() {
                None
            } else {
                let benchmark_data_str = CStr::from_ptr(benchmark_data).to_str().unwrap_or("");
                let df = match dataframe_from_json(benchmark_data_str) {
                    Ok(df) => df,
                    Err(e) => {
                        set_last_error(format!("Failed to parse benchmark data JSON: {e}"));
                        return std::ptr::null_mut();
                    }
                };
                let bench_name = benchmark_symbol.as_deref().unwrap_or("Benchmark");
                match KLINE::from_dataframe(bench_name, &df) {
                    Ok(kline) => Some(kline),
                    Err(e) => {
                        set_last_error_from_err(
                            &format!("Failed to build KLINE for benchmark '{bench_name}'"),
                            &*e,
                        );
                        return std::ptr::null_mut();
                    }
                }
            }
        };

        // Parse transactions
        let parsed_transactions = unsafe {
            if transactions.is_null() {
                None
            } else {
                let transactions_str = CStr::from_ptr(transactions).to_str().unwrap_or("[]");
                if transactions_str.is_empty() || transactions_str == "[]" {
                    None
                } else {
                    parse_transactions(transactions_str)
                }
            }
        };

        // Parse rebalance strategy
        let parsed_rebalance_strategy = unsafe {
            if rebalance_strategy.is_null() {
                None
            } else {
                let strategy_str = CStr::from_ptr(rebalance_strategy).to_str().unwrap_or("");
                if strategy_str.is_empty() || strategy_str == "{}" {
                    None
                } else {
                    parse_rebalance_strategy(strategy_str)
                }
            }
        };

        // Parse scheduled cash flows
        let parsed_scheduled_cash_flows = unsafe {
            if scheduled_cash_flows.is_null() {
                None
            } else {
                let flows_str = CStr::from_ptr(scheduled_cash_flows)
                    .to_str()
                    .unwrap_or("[]");
                if flows_str.is_empty() || flows_str == "[]" {
                    None
                } else {
                    parse_scheduled_cash_flows(flows_str)
                }
            }
        };

        let interval = Interval::from_str(interval_str).unwrap_or(Interval::OneDay);
        let objective_function = ObjectiveFunction::from_str(objective_function_str)
            .unwrap_or(ObjectiveFunction::MaxSharpe);
        let asset_constraints: Option<Vec<(f64, f64)>> =
            serde_json::from_str(asset_constraints_str).ok();
        let categorical_constraints: Option<Vec<CategoricalWeights>> =
            serde_json::from_str(categorical_constraints_str).ok();
        let weights: Option<Vec<f64>> = serde_json::from_str(weights_str).ok();

        let constraints = Constraints {
            asset_weights: asset_constraints,
            categorical_weights: categorical_constraints,
        };

        let rt = match make_runtime() {
            Ok(rt) => rt,
            Err(()) => return std::ptr::null_mut(),
        };

        let build_result = rt.block_on({
            let mut builder = Portfolio::builder()
                .ticker_symbols(ticker_symbols_ref)
                .start_date(start_date)
                .end_date(end_date)
                .interval(interval)
                .confidence_level(confidence_level)
                .risk_free_rate(risk_free_rate)
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
        });

        let mut portfolio = match build_result {
            Ok(p) => p,
            Err(e) => {
                set_last_error_from_err("Failed to build portfolio", &*e);
                return std::ptr::null_mut();
            }
        };

        // If weights are provided, evaluate directly (no optimization).
        // Otherwise, optimize (which also computes in-sample performance stats).
        if weights.is_some() {
            if let Err(e) = portfolio.performance_stats() {
                set_last_error_from_err("Failed to compute performance stats", &*e);
                return std::ptr::null_mut();
            }
        } else {
            if let Err(e) = portfolio.optimize() {
                set_last_error_from_err("Failed to optimize portfolio", &*e);
                return std::ptr::null_mut();
            }
        }

        Box::into_raw(Box::new(portfolio))
    }));
    match result {
        Ok(ptr) => ptr,
        Err(()) => std::ptr::null_mut(),
    }
}

// Free Portfolio
#[no_mangle]
pub extern "C" fn finalytics_portfolio_free(handle: PortfolioHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        };
    }
}

// Get optimization results
#[no_mangle]
pub extern "C" fn finalytics_portfolio_optimization_results(
    handle: PortfolioHandle,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };

    let data = &portfolio.data;

    // Build the optimization portion of the JSON
    let (
        obj_fn_str,
        opt_method,
        optimal_weights,
        category_weights,
        efficient_frontier,
        risk_contributions,
    ) = if let Some(ref opt) = portfolio.optimization_result {
        let obj_fn_str = match opt.objective_function {
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
        };
        (
            json!(obj_fn_str),
            json!(opt.optimization_method),
            json!(opt.optimal_weights),
            json!(opt.category_weights),
            json!(opt.efficient_frontier),
            json!(opt.risk_contributions),
        )
    } else {
        (
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
        )
    };

    // Build the performance portion of the JSON
    let (
        weights,
        starting_weights,
        ending_weights,
        starting_values,
        ending_values,
        portfolio_returns_json,
        daily_return,
        daily_volatility,
        cumulative_return,
        annualized_return,
        annualized_volatility,
        alpha,
        beta,
        sharpe_ratio,
        sortino_ratio,
        active_return,
        active_risk,
        information_ratio,
        calmar_ratio,
        maximum_drawdown,
        value_at_risk,
        expected_shortfall,
        money_weighted_return,
    ) = if let Some(ref perf) = portfolio.performance_stats {
        let s = &perf.performance_stats;
        (
            json!(perf.weights),
            json!(perf.starting_weights),
            json!(perf.ending_weights),
            json!(perf.starting_values),
            json!(perf.ending_values),
            json!(series_to_json(&perf.portfolio_returns).unwrap_or_else(|_| "null".to_string())),
            json!(s.daily_return),
            json!(s.daily_volatility),
            json!(s.cumulative_return),
            json!(s.annualized_return),
            json!(s.annualized_volatility),
            json!(s.alpha),
            json!(s.beta),
            json!(s.sharpe_ratio),
            json!(s.sortino_ratio),
            json!(s.active_return),
            json!(s.active_risk),
            json!(s.information_ratio),
            json!(s.calmar_ratio),
            json!(s.maximum_drawdown),
            json!(s.value_at_risk),
            json!(s.expected_shortfall),
            json!(perf.money_weighted_return),
        )
    } else {
        (
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
            json!(null),
        )
    };

    let json = json!({
        "ticker_symbols": data.ticker_symbols,
        "benchmark_symbol": data.benchmark_symbol,
        "start_date": data.start_date,
        "end_date": data.end_date,
        "interval": data.interval.mode,
        "confidence_level": data.confidence_level,
        "risk_free_rate": data.risk_free_rate,
        "portfolio_returns": dataframe_to_json(&mut data.portfolio_returns.clone()).unwrap_or_else(|_| "null".to_string()),
        "benchmark_returns": data.benchmark_returns.as_ref().map(|br| series_to_json(br).unwrap_or_else(|_| "null".to_string())).unwrap_or_else(|| "null".to_string()),
        "objective_function": obj_fn_str,
        "optimization_method": opt_method,
        "optimal_weights": optimal_weights,
        "category_weights": category_weights,
        "efficient_frontier": efficient_frontier,
        "risk_contributions": risk_contributions,
        "weights": weights,
        "starting_weights": starting_weights,
        "ending_weights": ending_weights,
        "starting_values": starting_values,
        "ending_values": ending_values,
        "optimal_portfolio_returns": portfolio_returns_json,
        "Daily Return": daily_return,
        "Daily Volatility": daily_volatility,
        "Cumulative Return": cumulative_return,
        "Annualized Return": annualized_return,
        "Annualized Volatility": annualized_volatility,
        "Alpha": alpha,
        "Beta": beta,
        "Sharpe Ratio": sharpe_ratio,
        "Sortino Ratio": sortino_ratio,
        "Active Return": active_return,
        "Active Risk": active_risk,
        "Information Ratio": information_ratio,
        "Calmar Ratio": calmar_ratio,
        "Maximum Drawdown": maximum_drawdown,
        "Value at Risk": value_at_risk,
        "Expected Shortfall": expected_shortfall,
        "Money Weighted Return": money_weighted_return,
    })
    .to_string();
    unsafe {
        *output = to_c_string(json);
    }
    0
}

// Optimization chart
#[no_mangle]
pub extern "C" fn finalytics_portfolio_optimization_chart(
    handle: PortfolioHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };
    let height = if height == 0 {
        None
    } else {
        Some(height as usize)
    };
    let width = if width == 0 {
        None
    } else {
        Some(width as usize)
    };
    match portfolio.optimization_chart(height, width) {
        Ok(plot) => {
            let html = plot.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(e) => {
            set_last_error_from_err("Failed to generate optimization chart", &*e);
            -1
        }
    }
}

// Performance chart
#[no_mangle]
pub extern "C" fn finalytics_portfolio_performance_chart(
    handle: PortfolioHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };
    let height = if height == 0 {
        None
    } else {
        Some(height as usize)
    };
    let width = if width == 0 {
        None
    } else {
        Some(width as usize)
    };
    match portfolio.performance_chart(height, width) {
        Ok(plot) => {
            let html = plot.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(e) => {
            set_last_error_from_err("Failed to generate performance chart", &*e);
            -1
        }
    }
}

// Performance stats — recomputes stats then returns the formatted table.
// This mirrors the Rust API where calling `performance_stats()` always
// recomputes, so callers can simply call this after `update_dates()`.
#[no_mangle]
pub extern "C" fn finalytics_portfolio_performance_stats(
    handle: PortfolioHandle,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &mut *handle
    };
    // Recompute stats (mirrors Rust's `portfolio.performance_stats()`)
    if let Err(e) = portfolio.performance_stats() {
        set_last_error_from_err("Failed to compute performance stats", &*e);
        return -1;
    }
    // Format the (now-fresh) stats into a table (sync call)
    match portfolio.performance_stats_table() {
        Ok(df) => match dataframe_to_json(&mut df.data.clone()) {
            Ok(json) => {
                unsafe {
                    *output = to_c_string(json);
                }
                0
            }
            Err(e) => {
                set_last_error(format!(
                    "Failed to serialize performance stats to JSON: {e}"
                ));
                -1
            }
        },
        Err(e) => {
            set_last_error_from_err("Failed to build performance stats table", &*e);
            -1
        }
    }
}

// Asset returns chart
#[no_mangle]
pub extern "C" fn finalytics_portfolio_asset_returns_chart(
    handle: PortfolioHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };
    let height = if height == 0 {
        None
    } else {
        Some(height as usize)
    };
    let width = if width == 0 {
        None
    } else {
        Some(width as usize)
    };
    match portfolio.returns_chart(height, width) {
        Ok(plot) => {
            let html = plot.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(e) => {
            set_last_error_from_err("Failed to generate asset returns chart", &*e);
            -1
        }
    }
}

// Portfolio value chart
#[no_mangle]
pub extern "C" fn finalytics_portfolio_value_chart(
    handle: PortfolioHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };
    let height = if height == 0 {
        None
    } else {
        Some(height as usize)
    };
    let width = if width == 0 {
        None
    } else {
        Some(width as usize)
    };
    match portfolio.portfolio_value_chart(height, width) {
        Ok(plot) => {
            let html = plot.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(e) => {
            set_last_error_from_err("Failed to generate portfolio value chart", &*e);
            -1
        }
    }
}

// Returns matrix
#[no_mangle]
pub extern "C" fn finalytics_portfolio_returns_matrix(
    handle: PortfolioHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };
    let height = if height == 0 {
        None
    } else {
        Some(height as usize)
    };
    let width = if width == 0 {
        None
    } else {
        Some(width as usize)
    };
    match portfolio.returns_matrix(height, width) {
        Ok(plot) => {
            let html = plot.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(e) => {
            set_last_error_from_err("Failed to generate returns matrix", &*e);
            -1
        }
    }
}

// Report
#[no_mangle]
pub extern "C" fn finalytics_portfolio_report(
    handle: PortfolioHandle,
    report_type: *const c_char,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };
    let report_type = unsafe { CStr::from_ptr(report_type).to_str().unwrap_or("") };
    let report_type = if report_type.is_empty() {
        ReportType::Performance
    } else {
        ReportType::from_str(report_type).unwrap_or(ReportType::Performance)
    };
    let rt = match make_runtime() {
        Ok(rt) => rt,
        Err(()) => return -1,
    };
    match rt.block_on(portfolio.report(Some(report_type))) {
        Ok(report) => {
            let html = report.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(e) => {
            set_last_error_from_err("Failed to generate report", &*e);
            -1
        }
    }
}

// Update portfolio dates for out-of-sample evaluation (Yahoo Finance data only).
// After calling this, call finalytics_portfolio_performance_stats to evaluate
// the optimized weights on the new data.
#[no_mangle]
pub extern "C" fn finalytics_portfolio_update_dates(
    handle: PortfolioHandle,
    start_date: *const c_char,
    end_date: *const c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &mut *handle
    };
    let start_date = unsafe { CStr::from_ptr(start_date).to_str().unwrap_or("") };
    let end_date = unsafe { CStr::from_ptr(end_date).to_str().unwrap_or("") };
    let rt = match make_runtime() {
        Ok(rt) => rt,
        Err(()) => return -1,
    };
    match rt.block_on(portfolio.update_dates(start_date, end_date)) {
        Ok(_) => 0,
        Err(e) => {
            set_last_error_from_err("Failed to update dates", &*e);
            -1
        }
    }
}

// Transaction history table
#[no_mangle]
pub extern "C" fn finalytics_portfolio_transaction_history(
    handle: PortfolioHandle,
    output: *mut *mut c_char,
) -> c_int {
    let portfolio = unsafe {
        if handle.is_null() {
            set_last_error("Portfolio handle is null".into());
            return -1;
        }
        &*handle
    };
    match portfolio.transaction_history_table() {
        Ok(Some(dt)) => match dataframe_to_json(&mut dt.data.clone()) {
            Ok(json) => {
                unsafe {
                    *output = to_c_string(json);
                }
                0
            }
            Err(e) => {
                set_last_error(format!(
                    "Failed to serialize transaction history to JSON: {e}"
                ));
                -1
            }
        },
        Ok(None) => {
            // No transaction history available — return an empty JSON array
            unsafe {
                *output = to_c_string("[]".to_string());
            }
            0
        }
        Err(e) => {
            set_last_error_from_err("Failed to get transaction history", &*e);
            -1
        }
    }
}
