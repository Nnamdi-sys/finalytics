use plotly::Plot;
use pyo3::prelude::*;
use pyo3::{PyObject, PyResult};


pub fn rust_plot_to_py_plot(plot: Plot) -> PyResult<PyObject> {
    // Convert the Plotly object to a JSON string
    let plot_json = plot.to_json();

    Python::with_gil(|py| {
        // Import the necessary Python libraries
        let py_plotly = Python::import(py,"plotly.graph_objects")?;
        let py_json = Python::import(py,"json")?;

        // Convert the JSON string to a Python dictionary
        let plot_dict: PyObject = py_json.call_method1("loads", (plot_json,))?.extract()?;

        // Manually construct the Plotly figure from the dictionary
        let figure_class = py_plotly.getattr("Figure")?;
        let fig = figure_class.call1((plot_dict,))?;

        Ok(fig.into())
    })
}





