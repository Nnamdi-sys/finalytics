use plotly::Plot;
use pyo3::prelude::*;
use pyo3::{PyObject, PyResult};

/// Format an error and its full chain of `.source()` causes into a single string.
///
/// Given an error like:
///   "Failed to generate report" caused by
///     "fetching chart data for ticker '#N/A'" caused by
///       "GET request failed with status 404 for URL: https://..."
///
/// This produces:
///   "Failed to generate report: fetching chart data for ticker '#N/A': GET request failed with status 404 for URL: https://..."
///
/// This is critical for the Python FFI because `format!("{e}")` on an
/// `anyhow::Error` (or `Box<dyn Error>`) only shows the outermost message,
/// losing all the context that was added via `.context()` / `.with_context()`.
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

pub fn rust_plot_to_py_plot(plot: Plot) -> PyResult<PyObject> {
    // Convert the Plotly object to a JSON string
    let plot_json = plot.to_json();

    Python::with_gil(|py| {
        // Import the necessary Python libraries
        let py_plotly = Python::import(py, "plotly.graph_objects")?;
        let py_json = Python::import(py, "json")?;

        // Convert the JSON string to a Python dictionary
        let plot_dict: PyObject = py_json.call_method1("loads", (plot_json,))?.extract()?;

        // Manually construct the Plotly figure from the dictionary
        let figure_class = py_plotly.getattr("Figure")?;
        let fig = figure_class.call1((plot_dict,))?;

        Ok(fig.into())
    })
}
