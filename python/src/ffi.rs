use plotly::Plot;
use polars::export::arrow::ffi;
use polars::prelude::*;
use pyo3::ffi::Py_uintptr_t;
use pyo3::prelude::*;
use pyo3::{PyObject, PyResult};
use pyo3::types:: PyDict;


pub fn rust_df_to_py_df(df: &DataFrame) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let py_polars = Python::import_bound(py, "polars")?;
        let py_dict = PyDict::new_bound(py);

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

        Ok(py_polars_df.into())
    })
}


/// Arrow array to Python.
pub fn to_py_array(pyarrow: Bound<PyModule>, array: ArrayRef) -> PyResult<PyObject> {
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

    Ok(array.into())
}

pub fn rust_series_to_py_series(series: &Series) -> PyResult<PyObject> {
    // ensure we have a single chunk
    let series = series.rechunk();
    let array = series.to_arrow(0, false);

    Python::with_gil(|py| {
        // import pyarrow
        let pyarrow = Python::import_bound(py, "pyarrow")?;

        // pyarrow array
        let pyarrow_array = to_py_array(pyarrow, array)?;

        // import polars
        let polars = Python::import_bound(py, "polars")?;
        let out = polars.call_method1("from_arrow", (pyarrow_array,))?;
        Ok(out.into())
    })
}

pub fn rust_plot_to_py_plot(plot: Plot) -> PyResult<PyObject> {
    // Convert the Plotly object to a JSON string
    let plot_json = plot.to_json();

    Python::with_gil(|py| {
        // Import the necessary Python libraries
        let py_plotly = Python::import_bound(py,"plotly.graph_objects")?;
        let py_json = Python::import_bound(py,"json")?;

        // Convert the JSON string to a Python dictionary
        let plot_dict: PyObject = py_json.call_method1("loads", (plot_json,))?.extract()?;

        // Manually construct the Plotly figure from the dictionary
        let figure_class = py_plotly.getattr("Figure")?;
        let fig = figure_class.call1((plot_dict,))?;

        Ok(fig.into())
    })
}





