use crate::utils::{dataframe_from_json, dataframe_to_json, series_to_json};
use finalytics::prelude::*;

use serde_json::json;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::{c_int, c_uint};
use std::str::FromStr;
use tokio::runtime::Runtime;

// Opaque handle for Ticker
pub type TickerHandle = *mut Ticker;

// Helper to convert Rust string to C string
fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

// Create a new Ticker
#[no_mangle]
pub extern "C" fn finalytics_ticker_new(
    symbol: *const c_char,
    start_date: *const c_char,
    end_date: *const c_char,
    interval: *const c_char,
    benchmark_symbol: *const c_char,
    confidence_level: f64,
    risk_free_rate: f64,
    ticker_data: *const c_char,
    benchmark_data: *const c_char,
) -> TickerHandle {
    let symbol = unsafe { CStr::from_ptr(symbol).to_str().unwrap_or("") };
    let start_date = unsafe { CStr::from_ptr(start_date).to_str().unwrap_or("") };
    let end_date = unsafe { CStr::from_ptr(end_date).to_str().unwrap_or("") };
    let interval = unsafe { CStr::from_ptr(interval).to_str().unwrap_or("1d") };
    let benchmark_symbol = unsafe { CStr::from_ptr(benchmark_symbol).to_str().unwrap_or("^GSPC") };
    let ticker_data = unsafe {
        if ticker_data.is_null() {
            None
        } else {
            let ticker_data = CStr::from_ptr(ticker_data).to_str().unwrap_or("");
            let df = dataframe_from_json(ticker_data).unwrap();
            Some(KLINE::from_dataframe(symbol, &df).unwrap())
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
    let ticker = Ticker::builder()
        .ticker(symbol)
        .start_date(start_date)
        .end_date(end_date)
        .interval(interval)
        .benchmark_symbol(benchmark_symbol)
        .confidence_level(confidence_level)
        .risk_free_rate(risk_free_rate)
        .ticker_data(ticker_data)
        .benchmark_data(benchmark_data)
        .build();
    Box::into_raw(Box::new(ticker))
}

// Free Ticker
#[no_mangle]
pub extern "C" fn finalytics_ticker_free(handle: TickerHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        };
    }
}

// Get quote
#[no_mangle]
pub extern "C" fn finalytics_ticker_get_quote(
    handle: TickerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_quote()) {
        Ok(quote) => {
            let json = json!({
                "Symbol": quote.symbol,
                "Name": quote.name,
                "Exchange": quote.exchange,
                "Currency": quote.currency,
                "Timestamp": quote.timestamp,
                "Current Price": quote.price,
                "24H Volume": quote.volume,
                "24H Open": quote.open,
                "24H High": quote.high,
                "24H Low": quote.low,
                "24H Close": quote.close,
            })
            .to_string();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Get summary stats
#[no_mangle]
pub extern "C" fn finalytics_ticker_get_summary_stats(
    handle: TickerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_ticker_stats()) {
        Ok(stats) => {
            let df = stats.to_dataframe().unwrap();
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
pub extern "C" fn finalytics_ticker_get_price_history(
    handle: TickerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_chart()) {
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
pub extern "C" fn finalytics_ticker_get_options_chain(
    handle: TickerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_options()) {
        Ok(options) => {
            let json = dataframe_to_json(&mut options.chain.clone()).unwrap();
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
pub extern "C" fn finalytics_ticker_get_news(
    handle: TickerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_news()) {
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
pub extern "C" fn finalytics_ticker_get_income_statement(
    handle: TickerHandle,
    frequency: *const c_char,
    formatted: c_int,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let formatted = formatted != 0;
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_financials(
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
pub extern "C" fn finalytics_ticker_get_balance_sheet(
    handle: TickerHandle,
    frequency: *const c_char,
    formatted: c_int,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let formatted = formatted != 0;
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_financials(
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
pub extern "C" fn finalytics_ticker_get_cashflow_statement(
    handle: TickerHandle,
    frequency: *const c_char,
    formatted: c_int,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let formatted = formatted != 0;
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_financials(
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
pub extern "C" fn finalytics_ticker_get_financial_ratios(
    handle: TickerHandle,
    frequency: *const c_char,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let frequency = unsafe { CStr::from_ptr(frequency).to_str().unwrap_or("annual") };
    let frequency = StatementFrequency::from_str(frequency).unwrap_or(StatementFrequency::Annual);
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.get_financials(StatementType::FinancialRatios, frequency, None)) {
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

// Volatility surface
#[no_mangle]
pub extern "C" fn finalytics_ticker_volatility_surface(
    handle: TickerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.volatility_surface()) {
        Ok(surface) => {
            let json = dataframe_to_json(&mut &mut surface.ivols_df.clone()).unwrap();
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
pub extern "C" fn finalytics_ticker_performance_stats(
    handle: TickerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(ticker.performance_stats()) {
        Ok(stats) => {
            let json = json!({
                "Symbol": stats.ticker_symbol,
                "Benchmark": stats.benchmark_symbol,
                "Start Date": stats.start_date,
                "End Date": stats.end_date,
                "Interval": stats.interval.average,
                "Confidence Level": stats.confidence_level,
                "Risk Free Rate": stats.risk_free_rate,
                "Daily Return": stats.performance_stats.daily_return,
                "Daily Volatility": stats.performance_stats.daily_volatility,
                "Total Return": stats.performance_stats.cumulative_return,
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
                "Security Prices": series_to_json(&stats.security_prices).unwrap(),
                "Security Returns": series_to_json(&stats.security_returns).unwrap(),
                "Benchmark Returns": series_to_json(&stats.benchmark_returns).unwrap(),
            })
            .to_string();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
    }
}

// Performance chart
#[no_mangle]
pub extern "C" fn finalytics_ticker_performance_chart(
    handle: TickerHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
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
    match rt.block_on(ticker.performance_chart(height, width)) {
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

// Candlestick chart
#[no_mangle]
pub extern "C" fn finalytics_ticker_candlestick_chart(
    handle: TickerHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
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
    match rt.block_on(ticker.candlestick_chart(height, width)) {
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

// Options chart
#[no_mangle]
pub extern "C" fn finalytics_ticker_options_chart(
    handle: TickerHandle,
    chart_type: *const c_char,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let chart_type = unsafe { CStr::from_ptr(chart_type).to_str().unwrap_or("") };
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
    match rt.block_on(ticker.options_charts(height, width)) {
        Ok(charts) => {
            let plot = match chart_type {
                "surface" => charts.volatility_surface,
                "smile" => charts.volatility_smile,
                "term_structure" => charts.volatility_term_structure,
                _ => return -1,
            };
            let html = plot.to_html();
            unsafe {
                *output = to_c_string(html);
            }
            0
        }
        Err(_) => -1,
    }
}

// News sentiment chart
#[no_mangle]
pub extern "C" fn finalytics_ticker_news_sentiment_chart(
    handle: TickerHandle,
    height: c_uint,
    width: c_uint,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
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
    match rt.block_on(ticker.news_sentiment_chart(height, width)) {
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
pub extern "C" fn finalytics_ticker_report(
    handle: TickerHandle,
    report_type: *const c_char,
    output: *mut *mut c_char,
) -> c_int {
    let ticker = unsafe {
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
    match rt.block_on(ticker.report(Some(report_type))) {
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
