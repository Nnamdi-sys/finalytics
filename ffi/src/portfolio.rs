use finalytics::prelude::*;
use polars::prelude::*;
use serde_json::json;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::{c_int, c_uint};
use std::str::FromStr;
use tokio::runtime::Runtime;

use crate::utils::{dataframe_from_json, dataframe_to_json, series_to_json};

// Opaque handle for Portfolio
pub type PortfolioHandle = *mut Portfolio;

// Helper to convert Rust string to C string
fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
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
) -> PortfolioHandle {
    let ticker_symbols = unsafe { CStr::from_ptr(ticker_symbols).to_str().unwrap_or("[]") };
    let benchmark_symbol = unsafe { CStr::from_ptr(benchmark_symbol).to_str().unwrap_or("^GSPC") };
    let start_date = unsafe { CStr::from_ptr(start_date).to_str().unwrap_or("") };
    let end_date = unsafe { CStr::from_ptr(end_date).to_str().unwrap_or("") };
    let interval = unsafe { CStr::from_ptr(interval).to_str().unwrap_or("1d") };
    let objective_function = unsafe {
        CStr::from_ptr(objective_function)
            .to_str()
            .unwrap_or("max_sharpe")
    };
    let asset_constraints = unsafe { CStr::from_ptr(asset_constraints).to_str().unwrap_or("[]") };
    let categorical_constraints = unsafe {
        CStr::from_ptr(categorical_constraints)
            .to_str()
            .unwrap_or("[]")
    };
    let weights = unsafe { CStr::from_ptr(weights).to_str().unwrap_or("[]") };
    let ticker_symbols: Vec<String> = serde_json::from_str(ticker_symbols).unwrap_or_default();
    let ticker_symbols_ref: Vec<&str> = ticker_symbols.iter().map(|s| s.as_str()).collect();
    let tickers_data = unsafe {
        if tickers_data.is_null() {
            None
        } else {
            let tickers_data = CStr::from_ptr(tickers_data).to_str().unwrap_or("[]");
            let tickers_data: Vec<String> = serde_json::from_str(tickers_data).unwrap();
            let dfs: Vec<DataFrame> = tickers_data.iter().map(|s| 
                dataframe_from_json(s).unwrap()).collect();
            Some(ticker_symbols_ref
                .iter()
                .zip(dfs)
                .map(|(&symbol, df)| KLINE::from_dataframe(symbol, &df).unwrap())
                .collect::<Vec<KLINE>>())
        }
    };
    let benchmark_data = unsafe {
        if benchmark_data.is_null() {
            None
        } else {
            let benchmark_data = CStr::from_ptr(benchmark_data).to_str().unwrap_or("");
            let df = dataframe_from_json(benchmark_data).unwrap();
            Some(KLINE::from_dataframe(benchmark_symbol, &df).unwrap())
        }
    };
    let interval = Interval::from_str(interval).unwrap_or(Interval::OneDay);
    let objective_function =
        ObjectiveFunction::from_str(objective_function).unwrap_or(ObjectiveFunction::MaxSharpe);
    let asset_constraints: Option<Vec<(f64, f64)>> = serde_json::from_str(asset_constraints).ok();
    let categorical_constraints: Option<Vec<(String, Vec<String>, Vec<(String, f64, f64)>)>> =
        serde_json::from_str(categorical_constraints).ok();
    let weights: Option<Vec<f64>> = serde_json::from_str(weights).ok();

    let constraints = categorical_constraints.map(|cats| Constraints {
        asset_weights: asset_constraints,
        categorical_weights: Some(
            cats.into_iter()
                .map(
                    |(name, category_per_symbol, weight_per_category)| CategoricalWeights {
                        name,
                        category_per_symbol,
                        weight_per_category,
                    },
                )
                .collect(),
        ),
    });

    let rt = Runtime::new().unwrap();
    let portfolio = rt
        .block_on(
            Portfolio::builder()
                .ticker_symbols(ticker_symbols_ref)
                .benchmark_symbol(benchmark_symbol)
                .start_date(start_date)
                .end_date(end_date)
                .interval(interval)
                .confidence_level(confidence_level)
                .risk_free_rate(risk_free_rate)
                .objective_function(objective_function)
                .constraints(constraints)
                .weights(weights)
                .tickers_data(tickers_data)
                .benchmark_data(benchmark_data)
                .build(),
        )
        .unwrap_or_else(|_| panic!("Failed to create Portfolio"));
    Box::into_raw(Box::new(portfolio))
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
            return -1;
        }
        &*handle
    };
    let stats = &portfolio.performance_stats;
    let json = json!({
        "ticker_symbols": stats.ticker_symbols,
        "benchmark_symbol": stats.benchmark_symbol,
        "start_date": stats.start_date,
        "end_date": stats.end_date,
        "interval": stats.interval.mode,
        "confidence_level": stats.confidence_level,
        "risk_free_rate": stats.risk_free_rate,
        "portfolio_returns": dataframe_to_json(&mut stats.portfolio_returns.clone()).unwrap(),
        "benchmark_returns": series_to_json(&stats.benchmark_returns).unwrap(),
        "objective_function": match stats.objective_function {
            ObjectiveFunction::MaxSharpe => "Maximize Sharpe Ratio",
            ObjectiveFunction::MaxSortino => "Maximize Sortino Ratio",
            ObjectiveFunction::MinVol => "Minimize Volatility",
            ObjectiveFunction::MaxReturn => "Maximize Return",
            ObjectiveFunction::MinDrawdown => "Minimize Drawdown",
            ObjectiveFunction::MinVar => "Minimize Value at Risk",
            ObjectiveFunction::MinCVaR => "Minimize Expected Shortfall",
        },
        "optimization_method": stats.optimization_method,
        "optimal_weights": stats.optimal_weights,
        "category_weights": stats.category_weights,
        "optimal_portfolio_returns": series_to_json(&stats.optimal_portfolio_returns).unwrap(),
        "Daily Return": stats.performance_stats.daily_return,
        "Daily Volatility": stats.performance_stats.daily_volatility,
        "Cumulative Return": stats.performance_stats.cumulative_return,
        "Annualized Return": stats.performance_stats.annualized_return,
        "Annualized Volatility": stats.performance_stats.annualized_volatility,
        "Alpha": stats.performance_stats.alpha,
        "Beta": stats.performance_stats.beta,
        "Sharpe Ratio": stats.performance_stats.sharpe_ratio,
        "Sortino Ratio": stats.performance_stats.sortino_ratio,
        "Active Return": stats.performance_stats.active_return,
        "Active Risk": stats.performance_stats.active_risk,
        "Information Ratio": stats.performance_stats.information_ratio,
        "Calmar Ratio": stats.performance_stats.calmar_ratio,
        "Maximum Drawdown": stats.performance_stats.maximum_drawdown,
        "Value at Risk": stats.performance_stats.value_at_risk,
        "Expected Shortfall": stats.performance_stats.expected_shortfall,
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
        Err(_) => -1,
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
        Err(_) => -1,
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
        Err(_) => -1,
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
        Err(_) => -1,
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
    let rt = Runtime::new().unwrap();
    match rt.block_on(portfolio.report(Some(report_type))) {
        Ok(report) => {
            let html = report.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(_) => -1,
    }
}
