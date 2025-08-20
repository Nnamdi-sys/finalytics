use crate::portfolio::PortfolioHandle;
use crate::ticker::TickerHandle;
use crate::utils::{dataframe_from_json, dataframe_to_json};
use finalytics::prelude::*;
use polars::prelude::*;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::{c_int, c_uint};
use std::str::FromStr;
use tokio::runtime::Runtime;

// Opaque handle for Tickers
pub type TickersHandle = *mut Tickers;

// Helper to convert Rust string to C string
fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

// Create a new Tickers
#[no_mangle]
pub extern "C" fn finalytics_tickers_new(
    symbols: *const c_char,
    start_date: *const c_char,
    end_date: *const c_char,
    interval: *const c_char,
    benchmark_symbol: *const c_char,
    confidence_level: f64,
    risk_free_rate: f64,
    tickers_data: *const c_char,
    benchmark_data: *const c_char,
) -> TickersHandle {
    let symbols = unsafe { CStr::from_ptr(symbols).to_str().unwrap_or("[]") };
    let start_date = unsafe { CStr::from_ptr(start_date).to_str().unwrap_or("") };
    let end_date = unsafe { CStr::from_ptr(end_date).to_str().unwrap_or("") };
    let interval = unsafe { CStr::from_ptr(interval).to_str().unwrap_or("1d") };
    let benchmark_symbol = unsafe { CStr::from_ptr(benchmark_symbol).to_str().unwrap_or("^GSPC") };
    let symbols: Vec<String> = serde_json::from_str(symbols).unwrap_or_default();
    let symbols_ref: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
    let interval = Interval::from_str(interval).unwrap_or(Interval::OneDay);
    let tickers_data = unsafe {
        if tickers_data.is_null() {
            None
        } else {
            let tickers_data = CStr::from_ptr(tickers_data).to_str().unwrap_or("[]");
            let tickers_data: Vec<String> = serde_json::from_str(tickers_data).unwrap();
            let dfs: Vec<DataFrame> = tickers_data.iter().map(|s| 
                dataframe_from_json(s).unwrap()).collect();
            Some(symbols_ref
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
    let tickers = Tickers::builder()
        .tickers(symbols_ref)
        .start_date(start_date)
        .end_date(end_date)
        .interval(interval)
        .benchmark_symbol(benchmark_symbol)
        .confidence_level(confidence_level)
        .risk_free_rate(risk_free_rate)
        .tickers_data(tickers_data)
        .benchmark_data(benchmark_data)
        .build();
    Box::into_raw(Box::new(tickers))
}

// Free Tickers
#[no_mangle]
pub extern "C" fn finalytics_tickers_free(handle: TickersHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        };
    }
}

// Get summary stats
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_summary_stats(
    handle: TickersHandle,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_ticker_stats()) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get price history
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_price_history(
    handle: TickersHandle,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_chart()) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get options chain
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_options_chain(
    handle: TickersHandle,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_options()) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get news
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_news(
    handle: TickersHandle,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_news()) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get income statement
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_income_statement(
    handle: TickersHandle,
    frequency: *const c_char,
    formatted: c_int,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let formatted = formatted != 0;
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_financials(
        StatementType::IncomeStatement,
        frequency,
        Some(formatted),
    )) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get balance sheet
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_balance_sheet(
    handle: TickersHandle,
    frequency: *const c_char,
    formatted: c_int,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let formatted = formatted != 0;
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_financials(
        StatementType::BalanceSheet,
        frequency,
        Some(formatted),
    )) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get cashflow statement
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_cashflow_statement(
    handle: TickersHandle,
    frequency: *const c_char,
    formatted: c_int,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let formatted = formatted != 0;
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_financials(
        StatementType::CashFlowStatement,
        frequency,
        Some(formatted),
    )) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get financial ratios
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_financial_ratios(
    handle: TickersHandle,
    frequency: *const c_char,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.get_financials(StatementType::FinancialRatios, frequency, None)) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Returns
#[no_mangle]
pub extern "C" fn finalytics_tickers_returns(
    handle: TickersHandle,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.returns()) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Performance stats
#[no_mangle]
pub extern "C" fn finalytics_tickers_performance_stats(
    handle: TickersHandle,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.performance_stats()) {
        Ok(df) => {
            let json = dataframe_to_json(&mut df.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Returns chart
#[no_mangle]
pub extern "C" fn finalytics_tickers_returns_chart(
    handle: TickersHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
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
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.returns_chart(height, width)) {
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
pub extern "C" fn finalytics_tickers_returns_matrix(
    handle: TickersHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
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
    let rt = Runtime::new().unwrap();
    match rt.block_on(tickers.returns_matrix(height, width)) {
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
pub extern "C" fn finalytics_tickers_report(
    handle: TickersHandle,
    report_type: *const c_char,
    output: *mut *mut c_char,
) -> c_int {
    let tickers = unsafe {
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
    match rt.block_on(tickers.report(Some(report_type))) {
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

// Get ticker
#[no_mangle]
pub extern "C" fn finalytics_tickers_get_ticker(
    handle: TickersHandle,
    symbol: *const c_char,
) -> TickerHandle {
    let tickers = unsafe {
        if handle.is_null() {
            return std::ptr::null_mut();
        }
        &*handle
    };
    let symbol = unsafe { CStr::from_ptr(symbol).to_str().unwrap_or("") };
    let ticker = Ticker::builder()
        .ticker(symbol)
        .start_date(&tickers.start_date)
        .end_date(&tickers.end_date)
        .interval(tickers.interval)
        .benchmark_symbol(&tickers.benchmark_symbol)
        .confidence_level(tickers.confidence_level)
        .risk_free_rate(tickers.risk_free_rate)
        .build();
    Box::into_raw(Box::new(ticker))
}

// Optimize
#[no_mangle]
pub extern "C" fn finalytics_tickers_optimize(
    handle: TickersHandle,
    objective_function: *const c_char,
    asset_constraints: *const c_char,
    categorical_constraints: *const c_char,
    weights: *const c_char,
) -> PortfolioHandle {
    let tickers = unsafe {
        if handle.is_null() {
            return std::ptr::null_mut();
        }
        &*handle
    };
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
                .ticker_symbols(
                    tickers
                        .tickers
                        .iter()
                        .map(|x| x.ticker.as_str())
                        .collect::<Vec<&str>>(),
                )
                .benchmark_symbol(&tickers.benchmark_symbol)
                .start_date(&tickers.start_date)
                .end_date(&tickers.end_date)
                .interval(tickers.interval)
                .confidence_level(tickers.confidence_level)
                .risk_free_rate(tickers.risk_free_rate)
                .objective_function(objective_function)
                .constraints(constraints)
                .weights(weights)
                .build(),
        )
        .unwrap_or_else(|_| panic!("Failed to create Portfolio"));
    Box::into_raw(Box::new(portfolio))
}
