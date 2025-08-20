mod portfolio;
mod screener;
mod ticker;
mod tickers;
mod utils;
pub use portfolio::*;
pub use screener::*;
pub use ticker::*;
pub use tickers::*;


// Dummy structs to satisfy cbindgen, with a field to avoid zero-sized type warnings
#[repr(C)]
pub struct Ticker {
    _dummy: u8,
}
#[repr(C)]
pub struct Portfolio {
    _dummy: u8,
}
#[repr(C)]
pub struct Tickers {
    _dummy: u8,
}
#[repr(C)]
pub struct Screener {
    _dummy: u8,
}

use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn finalytics_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        };
    }
}

// To generate finalytics header file
// cbindgen --config cbindgen.toml --crate finalytics-ffi --output include/finalytics.h
