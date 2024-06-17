mod ffi;
mod ticker;
mod portfolio;

use pyo3::prelude::*;
use crate::ticker::PyTicker;
use crate::portfolio::PyPortfolio;

#[pymodule]
#[pyo3(name = "finalytics")]
fn finalytics_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyTicker>()?;
    m.add_class::<PyPortfolio>()?;
    Ok(())
}

