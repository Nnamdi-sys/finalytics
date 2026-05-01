//! Centralised error type for the Finalytics library.
//!
//! [`FinalyticsError`] replaces the ad-hoc mix of `Box<dyn Error>`, `.unwrap()`,
//! and `panic!()` that previously existed throughout the codebase.  Every public
//! function that can fail should return `Result<T, FinalyticsError>`.
//!
//! # Design goals
//!
//! * **Defensive** – callers get a descriptive, structured error instead of a
//!   panic or an opaque trait object.
//! * **Ergonomic** – blanket `From` impls for Polars, reqwest, serde_json, and
//!   `Box<dyn Error>` let existing `?` call-sites compile without changes.
//! * **Matchable** – consumers can pattern-match on the variant to decide how to
//!   handle each failure mode (retry, skip, abort, etc.).

use chrono::{DateTime, NaiveDateTime};
use polars::prelude::{PolarsError, Series};
use std::fmt;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// The unified error type returned by all fallible Finalytics operations.
#[derive(Debug)]
pub enum FinalyticsError {
    // ── Data layer ──────────────────────────────────────────────────────
    /// A network request or remote API call failed.
    DataFetch { source: String, message: String },

    /// An API response could not be parsed (missing field, unexpected shape,
    /// wrong JSON type, malformed date, etc.).
    DataParse { source: String, message: String },

    // ── DataFrame / Series layer ────────────────────────────────────────
    /// A [`Series`] had an unexpected dtype (e.g. expected `Float64`, got
    /// `Utf8`).
    DtypeMismatch {
        column: String,
        expected: String,
        actual: String,
    },

    /// A [`Series`] or DataFrame column contained null values where none
    /// were expected.
    NullValues { column: String, null_count: usize },

    /// A required column was missing from a [`DataFrame`].
    ColumnNotFound { name: String },

    /// A DataFrame operation (join, sort, filter, …) failed.
    DataFrameOperation { message: String },

    // ── Computation layer ───────────────────────────────────────────────
    /// Not enough data points to compute the requested statistic.
    InsufficientData {
        required: usize,
        actual: usize,
        context: String,
    },

    /// A computation produced NaN or ±infinity.
    NonFiniteResult { context: String },

    /// The portfolio optimiser failed to find a feasible solution.
    OptimizationFailed { objective: String, message: String },

    // ── Configuration layer ─────────────────────────────────────────────
    /// A user-supplied parameter was invalid.
    InvalidParameter { name: String, message: String },

    // ── Pass-through wrappers ───────────────────────────────────────────
    /// Wraps a [`PolarsError`].
    Polars(PolarsError),

    /// Wraps any other upstream error that doesn't have a dedicated variant.
    External(Box<dyn std::error::Error + Send + Sync>),
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for FinalyticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Data
            FinalyticsError::DataFetch { source, message } => {
                write!(f, "[DataFetch] {source}: {message}")
            }
            FinalyticsError::DataParse { source, message } => {
                write!(f, "[DataParse] {source}: {message}")
            }

            // DataFrame / Series
            FinalyticsError::DtypeMismatch {
                column,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "[DtypeMismatch] Column '{column}': expected {expected}, got {actual}"
                )
            }
            FinalyticsError::NullValues { column, null_count } => {
                write!(
                    f,
                    "[NullValues] Column '{column}' contains {null_count} null value(s)"
                )
            }
            FinalyticsError::ColumnNotFound { name } => {
                write!(f, "[ColumnNotFound] Column '{name}' not found in DataFrame")
            }
            FinalyticsError::DataFrameOperation { message } => {
                write!(f, "[DataFrameOperation] {message}")
            }

            // Computation
            FinalyticsError::InsufficientData {
                required,
                actual,
                context,
            } => {
                write!(
                    f,
                    "[InsufficientData] {context}: need at least {required} observations, got {actual}"
                )
            }
            FinalyticsError::NonFiniteResult { context } => {
                write!(f, "[NonFiniteResult] {context}: result is NaN or infinite")
            }
            FinalyticsError::OptimizationFailed { objective, message } => {
                write!(f, "[OptimizationFailed] Objective '{objective}': {message}")
            }

            // Configuration
            FinalyticsError::InvalidParameter { name, message } => {
                write!(f, "[InvalidParameter] '{name}': {message}")
            }

            // Pass-through
            FinalyticsError::Polars(e) => write!(f, "[Polars] {e}"),
            FinalyticsError::External(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for FinalyticsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FinalyticsError::Polars(e) => Some(e),
            FinalyticsError::External(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<PolarsError> for FinalyticsError {
    fn from(e: PolarsError) -> Self {
        FinalyticsError::Polars(e)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for FinalyticsError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        FinalyticsError::External(e)
    }
}

impl From<Box<dyn std::error::Error>> for FinalyticsError {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        FinalyticsError::External(e.to_string().into())
    }
}

impl From<String> for FinalyticsError {
    fn from(s: String) -> Self {
        FinalyticsError::External(s.into())
    }
}

impl From<&str> for FinalyticsError {
    fn from(s: &str) -> Self {
        FinalyticsError::External(s.into())
    }
}

impl From<serde_json::Error> for FinalyticsError {
    fn from(e: serde_json::Error) -> Self {
        FinalyticsError::DataParse {
            source: "serde_json".into(),
            message: e.to_string(),
        }
    }
}

impl From<reqwest::Error> for FinalyticsError {
    fn from(e: reqwest::Error) -> Self {
        FinalyticsError::DataFetch {
            source: "reqwest".into(),
            message: e.to_string(),
        }
    }
}

impl From<std::io::Error> for FinalyticsError {
    fn from(e: std::io::Error) -> Self {
        FinalyticsError::External(Box::new(e))
    }
}

impl From<std::num::ParseFloatError> for FinalyticsError {
    fn from(e: std::num::ParseFloatError) -> Self {
        FinalyticsError::DataParse {
            source: "parse_float".into(),
            message: e.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for FinalyticsError {
    fn from(e: std::num::ParseIntError) -> Self {
        FinalyticsError::DataParse {
            source: "parse_int".into(),
            message: e.to_string(),
        }
    }
}

impl From<chrono::ParseError> for FinalyticsError {
    fn from(e: chrono::ParseError) -> Self {
        FinalyticsError::DataParse {
            source: "chrono".into(),
            message: e.to_string(),
        }
    }
}

// NOTE: No explicit `From<FinalyticsError> for Box<dyn Error>` is needed —
// since we implement `std::error::Error`, the blanket impl in `alloc`
// already provides this conversion automatically.

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

/// Extract a `Vec<f64>` from a Polars [`Series`], returning an error if the
/// series is not `Float64` or contains any null values.
///
/// This replaces the pervasive `.f64().unwrap().to_vec().iter().map(|x|
/// x.unwrap()).collect()` pattern with a single, safe call.
pub fn series_to_vec_f64(series: &Series, name: &str) -> Result<Vec<f64>, FinalyticsError> {
    let ca = series.f64().map_err(|_| FinalyticsError::DtypeMismatch {
        column: name.to_string(),
        expected: "Float64".to_string(),
        actual: format!("{:?}", series.dtype()),
    })?;
    let null_count = ca.null_count();
    if null_count > 0 {
        return Err(FinalyticsError::NullValues {
            column: name.to_string(),
            null_count,
        });
    }
    Ok(ca.into_no_null_iter().collect())
}

/// Extract a `Vec<Option<f64>>` from a Polars [`Series`], returning an error
/// only if the series dtype is wrong.  Nulls are preserved as `None`.
///
/// Useful for columns where nulls are expected and handled downstream (e.g.
/// prices before forward-fill).
pub fn series_to_optional_vec_f64(
    series: &Series,
    name: &str,
) -> Result<Vec<Option<f64>>, FinalyticsError> {
    let ca = series.f64().map_err(|_| FinalyticsError::DtypeMismatch {
        column: name.to_string(),
        expected: "Float64".to_string(),
        actual: format!("{:?}", series.dtype()),
    })?;
    Ok(ca.into_iter().collect())
}

/// Extract a `Vec<f64>` from a column of a [`polars::prelude::DataFrame`] by
/// name.  Returns [`FinalyticsError::ColumnNotFound`] if the column doesn't
/// exist, or [`FinalyticsError::DtypeMismatch`] / [`FinalyticsError::NullValues`]
/// if the data is invalid.
pub fn column_to_vec_f64(
    df: &polars::prelude::DataFrame,
    name: &str,
) -> Result<Vec<f64>, FinalyticsError> {
    let col = df
        .column(name)
        .map_err(|_| FinalyticsError::ColumnNotFound {
            name: name.to_string(),
        })?;
    series_to_vec_f64(
        col.as_series().unwrap_or_else(|| {
            // In current Polars versions, .column() returns a &Column which
            // always has an underlying Series.  This branch is a safety net.
            panic!("BUG: DataFrame column '{name}' could not be viewed as Series");
        }),
        name,
    )
}

/// Validate that a Series has at least `min_len` non-null observations.
/// Returns `Ok(())` or an [`FinalyticsError::InsufficientData`] error.
pub fn require_min_length(
    series: &Series,
    min_len: usize,
    context: &str,
) -> Result<(), FinalyticsError> {
    let actual = series.len() - series.null_count();
    if actual < min_len {
        return Err(FinalyticsError::InsufficientData {
            required: min_len,
            actual,
            context: context.to_string(),
        });
    }
    Ok(())
}

/// Safely get the underlying [`Series`] from a Polars [`polars::prelude::Column`].
///
/// In current Polars versions `.column()` returns a `&Column` which always has
/// an underlying `Series`, but this helper turns the theoretical failure into a
/// descriptive error instead of a panic.
pub fn col_as_series(col: &polars::prelude::Column) -> Result<&Series, FinalyticsError> {
    col.as_series()
        .ok_or_else(|| FinalyticsError::DataFrameOperation {
            message: format!("Column '{}' could not be viewed as a Series", col.name()),
        })
}

/// Extract a `Vec<NaiveDateTime>` from a Polars datetime [`Series`] (millisecond
/// precision), returning a descriptive error on null or invalid values instead
/// of panicking.
pub fn series_to_naive_datetimes(
    series: &Series,
    name: &str,
) -> Result<Vec<NaiveDateTime>, FinalyticsError> {
    let ca = series
        .datetime()
        .map_err(|_| FinalyticsError::DtypeMismatch {
            column: name.to_string(),
            expected: "Datetime".to_string(),
            actual: format!("{:?}", series.dtype()),
        })?;
    let mut out = Vec::with_capacity(ca.len());
    for opt_val in ca.into_iter() {
        let millis = opt_val.ok_or_else(|| FinalyticsError::NullValues {
            column: name.to_string(),
            null_count: 1,
        })?;
        let dt = DateTime::from_timestamp_millis(millis)
            .ok_or_else(|| FinalyticsError::DataParse {
                source: "timestamp".to_string(),
                message: format!(
                    "Column '{name}': millisecond value {millis} is not a valid timestamp"
                ),
            })?
            .naive_local();
        out.push(dt);
    }
    Ok(out)
}

/// Create a TA indicator, mapping the `Result::Err` (typically a `TaError`)
/// into [`FinalyticsError::InvalidParameter`].
///
/// Usage:
/// ```ignore
/// let mut sma = new_indicator("SMA", || SimpleMovingAverage::new(period))?;
/// ```
pub fn new_indicator<T, E: std::fmt::Display>(
    name: &str,
    f: impl FnOnce() -> Result<T, E>,
) -> Result<T, FinalyticsError> {
    f().map_err(|e| FinalyticsError::InvalidParameter {
        name: name.to_string(),
        message: e.to_string(),
    })
}
