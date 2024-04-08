mod ffi;
mod symbols;
mod ticker;
mod portfolio;

use pyo3::prelude::*;
use crate::symbols::get_symbols_py;
use crate::ticker::PyTicker;
use crate::portfolio::PyPortfolio;

#[pymodule]
#[pyo3(name = "finalytics")]
fn finalytics_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_symbols_py, m)?).unwrap();
    m.add_class::<PyTicker>()?;
    m.add_class::<PyPortfolio>()?;
    Ok(())
}

