mod ticker;
mod portfolio;
mod tickers;
mod ffi;

use pyo3::prelude::*;
use crate::ticker::PyTicker;
use crate::tickers::PyTickers;
use crate::portfolio::PyPortfolio;

#[pymodule]
#[pyo3(name = "finalytics")]
fn finalytics_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyTicker>()?;
    m.add_class::<PyTickers>()?;
    m.add_class::<PyPortfolio>()?;
    Ok(())
}

