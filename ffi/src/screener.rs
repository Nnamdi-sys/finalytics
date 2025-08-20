use crate::utils::dataframe_to_json;
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
    CString::new(s).unwrap().into_raw()
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
    let quote_type = unsafe { CStr::from_ptr(quote_type).to_str().unwrap_or("EQUITY") };
    let filters = unsafe { CStr::from_ptr(filters).to_str().unwrap_or("[]") };
    let sort_field = unsafe {
        if sort_field.is_null() {
            None
        } else {
            Some(CStr::from_ptr(sort_field).to_str().unwrap_or(""))
        }
    };
    let sort_descending = sort_descending != 0;
    let offset = offset as usize;
    let size = if size == 0 { 250 } else { size as usize };

    let quote_type = QuoteType::from_str(quote_type).unwrap_or(QuoteType::Equity);
    let filters: Vec<String> = serde_json::from_str(filters).unwrap();
    let filters = filters
        .into_iter()
        .map(ScreenerFilter::Custom)
        .collect::<Vec<_>>();
    let sort_field = sort_field
        .map(|f| {
            if f.is_empty() {
                None
            } else {
                Some(match quote_type {
                    QuoteType::Equity => ScreenerMetric::Equity(
                        EquityScreener::from_str(f).unwrap_or(EquityScreener::MarketCapIntraday),
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
                        CryptoScreener::from_str(f).unwrap_or(CryptoScreener::MarketCapIntraday),
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

    let rt = Runtime::new().unwrap();
    let screener = rt
        .block_on(screener_builder.build())
        .unwrap_or_else(|err| panic!("Failed to create Screener: {}", err));
    Box::into_raw(Box::new(screener))
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
            return -1;
        }
        &*handle
    };
    let json = dataframe_to_json(&mut screener.result.clone()).unwrap();
    unsafe {
        *output = to_c_string(json);
    }
    0
}

// Get metrics
#[no_mangle]
pub extern "C" fn finalytics_screener_metrics(
    handle: ScreenerHandle,
    output: *mut *mut c_char,
) -> c_int {
    let screener = unsafe {
        if handle.is_null() {
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    match rt.block_on(screener.metrics()) {
        Ok(metrics) => {
            let json = dataframe_to_json(&mut metrics.data.clone()).unwrap();
            unsafe {
                *output = to_c_string(json);
            }
            0
        }
        Err(_) => -1,
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
            return -1;
        }
        &*handle
    };
    let rt = Runtime::new().unwrap();
    let overview = screener.overview();
    let metrics = match rt.block_on(screener.metrics()) {
        Ok(metrics) => metrics,
        Err(_) => return -1,
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
