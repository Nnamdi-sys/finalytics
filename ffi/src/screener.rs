use crate::utils::dataframe_to_json;
use crate::{catch_panic, set_last_error, set_last_error_from_err};
use finalytics::data::yahoo::screeners::builder::ScreenerBuilder;
use finalytics::prelude::{
    CryptoScreener, EquityScreener, EtfScreener, FutureScreener, IndexScreener, MutualFundScreener,
    QuoteType, Screener, ScreenerFilter, ScreenerMetric,
};

use serde_json::json;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::{c_int, c_uint};
use std::str::FromStr;
use tokio::runtime::Runtime;

// Opaque handle for Screener
pub type ScreenerHandle = *mut Screener;

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

// Create a new Screener
#[no_mangle]
pub extern "C" fn finalytics_screener_new(
    quote_type: *const c_char,
    filters: *const c_char,
    sort_field: *const c_char,
    sort_descending: c_int,
    offset: c_uint,
    size: c_uint,
) -> ScreenerHandle {
    let result = catch_panic(std::panic::AssertUnwindSafe(|| {
        let quote_type_str = unsafe { CStr::from_ptr(quote_type).to_str().unwrap_or("EQUITY") };
        let filters_str = unsafe { CStr::from_ptr(filters).to_str().unwrap_or("[]") };
        let sort_field_str = unsafe {
            if sort_field.is_null() {
                None
            } else {
                Some(CStr::from_ptr(sort_field).to_str().unwrap_or(""))
            }
        };
        let sort_descending = sort_descending != 0;
        let offset = offset as usize;
        let size = if size == 0 { 250 } else { size as usize };

        let quote_type = QuoteType::from_str(quote_type_str).unwrap_or(QuoteType::Equity);
        let filters: Vec<String> = serde_json::from_str(filters_str).unwrap_or_default();
        let filters = filters
            .into_iter()
            .map(ScreenerFilter::Custom)
            .collect::<Vec<_>>();
        let sort_field = sort_field_str
            .map(|f| {
                if f.is_empty() {
                    None
                } else {
                    Some(match quote_type {
                        QuoteType::Equity => ScreenerMetric::Equity(
                            EquityScreener::from_str(f)
                                .unwrap_or(EquityScreener::MarketCapIntraday),
                        ),
                        QuoteType::MutualFund => ScreenerMetric::MutualFund(
                            MutualFundScreener::from_str(f)
                                .unwrap_or(MutualFundScreener::FundNetAssets),
                        ),
                        QuoteType::Etf => ScreenerMetric::Etf(
                            EtfScreener::from_str(f).unwrap_or(EtfScreener::FundNetAssets),
                        ),
                        QuoteType::Index => ScreenerMetric::Index(
                            IndexScreener::from_str(f).unwrap_or(IndexScreener::PercentChange),
                        ),
                        QuoteType::Future => ScreenerMetric::Future(
                            FutureScreener::from_str(f).unwrap_or(FutureScreener::PercentChange),
                        ),
                        QuoteType::Crypto => ScreenerMetric::Crypto(
                            CryptoScreener::from_str(f)
                                .unwrap_or(CryptoScreener::MarketCapIntraday),
                        ),
                    })
                }
            })
            .unwrap_or(None);

        let screener_builder = ScreenerBuilder {
            quote_type: Some(quote_type),
            filters,
            sort_field,
            sort_descending,
            offset,
            size,
        };

        let rt = match make_runtime() {
            Ok(rt) => rt,
            Err(()) => return std::ptr::null_mut(),
        };
        match rt.block_on(screener_builder.build()) {
            Ok(screener) => Box::into_raw(Box::new(screener)),
            Err(e) => {
                set_last_error_from_err("Failed to build screener", &*e);
                std::ptr::null_mut()
            }
        }
    }));
    match result {
        Ok(ptr) => ptr,
        Err(()) => std::ptr::null_mut(),
    }
}

// Free Screener
#[no_mangle]
pub extern "C" fn finalytics_screener_free(handle: ScreenerHandle) {
    if !handle.is_null() {
        unsafe {
            let _ = Box::from_raw(handle);
        };
    }
}

// Get symbols
#[no_mangle]
pub extern "C" fn finalytics_screener_symbols(
    handle: ScreenerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let screener = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let json = json!(screener.symbols).to_string();
    unsafe {
        *output = to_c_string(json);
    }
    0
}

// Get overview
#[no_mangle]
pub extern "C" fn finalytics_screener_overview(
    handle: ScreenerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let screener = unsafe {
        if handle.is_null() {
            set_last_error("Screener handle is null".into());
            return -1;
        }
        &*handle
    };
    match dataframe_to_json(&mut screener.result.clone()) {
        Ok(json) => {
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(e) => {
            set_last_error(format!(
                "Failed to serialize screener overview to JSON: {e}"
            ));
            -1
        }
    }
}

// Get metrics
#[no_mangle]
pub extern "C" fn finalytics_screener_metrics(
    handle: ScreenerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let screener = unsafe {
        if handle.is_null() {
            set_last_error("Screener handle is null".into());
            return -1;
        }
        &*handle
    };
    let rt = match make_runtime() {
        Ok(rt) => rt,
        Err(()) => return -1,
    };
    match rt.block_on(screener.metrics()) {
        Ok(metrics) => match dataframe_to_json(&mut metrics.data.clone()) {
            Ok(json) => {
                unsafe {
                    *output = to_c_string(json);
                }
                0
            }
            Err(e) => {
                set_last_error(format!("Failed to serialize screener metrics to JSON: {e}"));
                -1
            }
        },
        Err(e) => {
            set_last_error_from_err("Failed to fetch screener metrics", &*e);
            -1
        }
    }
}

// Display
#[no_mangle]
pub extern "C" fn finalytics_screener_display(
    handle: ScreenerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let screener = unsafe {
        if handle.is_null() {
            set_last_error("Screener handle is null".into());
            return -1;
        }
        &*handle
    };
    let rt = match make_runtime() {
        Ok(rt) => rt,
        Err(()) => return -1,
    };
    let overview = screener.overview();
    let metrics = match rt.block_on(screener.metrics()) {
        Ok(metrics) => metrics,
        Err(e) => {
            set_last_error_from_err("Failed to fetch screener metrics for display", &*e);
            return -1;
        }
    };
    let overview_html = overview.to_html().unwrap_or_default();
    let metrics_html = metrics.to_html().unwrap_or_default();
    let json = json!({
        "overview_html": overview_html,
        "metrics_html": metrics_html
    })
    .to_string();
    unsafe {
        *output = to_c_string(json);
    }
    0
}
