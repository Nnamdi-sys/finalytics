use plotly::Plot;
use polars::export::arrow::ffi;
use polars::prelude::*;
use pyo3::ffi::Py_uintptr_t;
use pyo3::prelude::*;
use pyo3::{PyObject, PyResult};
use pyo3::types:: PyDict;


pub fn rust_df_to_py_df(df: &DataFrame) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let py_polars = py.import("polars")?;
        let py_dict = PyDict::new(py);

        // Convert each series into a Python Polars Series
        for series in df.get_columns() {
            let column_name = series.name();
            let py_series = rust_series_to_py_series(series)?;
            py_dict.set_item(column_name, py_series)?;
        }

        // Create a Python Polars DataFrame
        let py_polars_df = py_polars
            .getattr("DataFrame")?
            .call1((py_dict,))
            .unwrap();

        Ok(py_polars_df.to_object(py))
    })
}


/// Arrow array to Python.
pub fn to_py_array(py: Python, pyarrow: &PyModule, array: ArrayRef) -> PyResult<PyObject> {
    let schema = Box::new(ffi::export_field_to_c(&ArrowField::new(
        "",
        array.data_type().clone(),
        true,
    )));
    let array = Box::new(ffi::export_array_to_c(array));

    let schema_ptr: *const ffi::ArrowSchema = &*schema;
    let array_ptr: *const ffi::ArrowArray = &*array;

    let array = pyarrow.getattr("Array")?.call_method1(
        "_import_from_c",
        (array_ptr as Py_uintptr_t, schema_ptr as Py_uintptr_t),
    )?;

    Ok(array.to_object(py))
}

pub fn rust_series_to_py_series(series: &Series) -> PyResult<PyObject> {
    // ensure we have a single chunk
    let series = series.rechunk();
    let array = series.to_arrow(0, false);

    Python::with_gil(|py| {
        // import pyarrow
        let pyarrow = py.import("pyarrow")?;

        // pyarrow array
        let pyarrow_array = to_py_array(py, pyarrow, array)?;

        // import polars
        let polars = py.import("polars")?;
        let out = polars.call_method1("from_arrow", (pyarrow_array,))?;
        Ok(out.to_object(py))
    })
}

pub fn rust_plot_to_py_plot(plot: Plot) -> PyResult<PyObject> {
    // Convert the Plotly object to a JSON string
    let plot_json = plot.to_json();

    Python::with_gil(|py| {
        // Import the necessary Python libraries
        let py_plotly = py.import("plotly.graph_objects")?;
        let py_json = py.import("json")?;

        // Convert the JSON string to a Python dictionary
        let plot_dict: PyObject = py_json.call_method1("loads", (plot_json,))?.extract()?;

        // Manually construct the Plotly figure from the dictionary
        let figure_class = py_plotly.getattr("Figure")?;
        let fig = figure_class.call1((plot_dict,))?;

        Ok(fig.to_object(py))
    })
}





