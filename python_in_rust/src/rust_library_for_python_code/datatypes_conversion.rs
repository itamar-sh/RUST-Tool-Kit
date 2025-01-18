use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;


// Demonstration conversion between Rust and Python data types.
fn data_types_example(py: Python<'_>) -> PyResult<PyObject> {
    // rust variables with rust data types
    let text: &str = "Hello, Python!";
    let integer: i32 = 42;
    let floating: f64 = 3.14;
    let boolean: bool = true;

    // python dict
    let python_dict = PyDict::new(py);

    // insert rust data types to python dict
    python_dict.set_item("text", text)?;
    python_dict.set_item("integert", integer)?;
    python_dict.set_item("floating", floating)?;
    python_dict.set_item("boolean", boolean)?;

    Ok(python_dict.to_object(py))
}


/// A Python module implemented in Rust.
#[pymodule]
fn libdataconversion(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(data_types_example, m)?)?;
    Ok(())
}