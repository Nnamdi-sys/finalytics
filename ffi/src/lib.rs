mod portfolio;
mod screener;
mod ticker;
mod tickers;
mod utils;
pub use portfolio::*;
pub use screener::*;
pub use ticker::*;
pub use tickers::*;

use std::cell::RefCell;
use std::ffi::{c_char, CString};

// ---------------------------------------------------------------------------
// Thread-local last-error buffer
// ---------------------------------------------------------------------------
//
// Every FFI function that fails writes a descriptive error message here before
// returning -1 (or a null pointer).  Consumers (Go, JS, C, …) can then call
// `finalytics_last_error()` to retrieve the message.
//
// This is the same pattern used by `GetLastError` / `errno` and avoids having
// to change the signature of every existing C function.

thread_local! {
    static LAST_ERROR: RefCell<String> = RefCell::new(String::new());
}

/// Store an error message in the thread-local buffer.
///
/// Called internally by every FFI function on the error path.
pub fn set_last_error(msg: String) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = msg;
    });
}

/// Format an error and its full `.source()` chain into a single string.
///
/// This is the FFI equivalent of the Python FFI's `error_chain_string`.
pub fn error_chain_string(err: &dyn std::error::Error) -> String {
    let mut chain = String::new();
    chain.push_str(&err.to_string());
    let mut current = err.source();
    while let Some(cause) = current {
        chain.push_str(": ");
        chain.push_str(&cause.to_string());
        current = cause.source();
    }
    chain
}

/// Convenience: set the last error from any `dyn Error`, walking the full
/// source chain so the caller gets the same detail level as the Python FFI.
pub fn set_last_error_from_err(prefix: &str, err: &dyn std::error::Error) {
    set_last_error(format!("{prefix}: {}", error_chain_string(err)));
}

// ---------------------------------------------------------------------------
// Public C API – error retrieval
// ---------------------------------------------------------------------------

/// Retrieve the last error message set by any `finalytics_*` function on the
/// current thread.
///
/// Returns a heap-allocated C string that the caller **must** free with
/// `finalytics_free_string()`.  If no error has been recorded the returned
/// string is empty (`""`).
///
/// # Usage from C
///
/// ```c
/// int rc = finalytics_ticker_get_price_history(handle, &output);
/// if (rc != 0) {
///     char *err = finalytics_last_error();
///     fprintf(stderr, "error: %s\n", err);
///     finalytics_free_string(err);
/// }
/// ```
#[no_mangle]
pub extern "C" fn finalytics_last_error() -> *mut c_char {
    LAST_ERROR.with(|e| {
        let msg = e.borrow().clone();
        CString::new(msg)
            .unwrap_or_else(|_| CString::new("(error message contained a NUL byte)").unwrap())
            .into_raw()
    })
}

/// Clear the thread-local error buffer.
///
/// Optional – callers that check `finalytics_last_error()` only after a
/// non-zero return code don't need this.  Useful in test harnesses.
#[no_mangle]
pub extern "C" fn finalytics_clear_last_error() {
    LAST_ERROR.with(|e| {
        e.borrow_mut().clear();
    });
}

// ---------------------------------------------------------------------------
// Public C API – memory management
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn finalytics_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        };
    }
}

// ---------------------------------------------------------------------------
// Panic guard
// ---------------------------------------------------------------------------
//
// A panic that unwinds across an `extern "C"` boundary is **undefined
// behaviour**.  `catch_panic` converts any panic into an error code and
// stores the panic payload in the last-error buffer.

/// Run `f` inside `std::panic::catch_unwind`.
///
/// *  If `f` returns normally its value is returned as `Ok(T)`.
/// *  If `f` panics the panic message is stored via `set_last_error` and
///    `Err(())` is returned so the caller can map it to `-1` / null.
pub fn catch_panic<F, T>(f: F) -> Result<T, ()>
where
    F: FnOnce() -> T + std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(val) => Ok(val),
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic".to_string()
            };
            set_last_error(format!("internal panic: {msg}"));
            Err(())
        }
    }
}

// ---------------------------------------------------------------------------
// Dummy structs for cbindgen
// ---------------------------------------------------------------------------
//
// These satisfy cbindgen so it can emit opaque pointer typedefs in the
// generated C header.  The `_dummy` field avoids zero-sized-type warnings.

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

// To generate finalytics header file
// cbindgen --config cbindgen.toml --crate finalytics-ffi --output include/finalytics.h
