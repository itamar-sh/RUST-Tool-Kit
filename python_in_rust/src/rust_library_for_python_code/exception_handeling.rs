use pyo3::prelude::*;
use pyo3::exceptions::PyZeroDivisionError;


// raise ZeroDivisionError
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(PyZeroDivisionError::new_err("Exception: Division by Zero"));
    }
}

// python module implemented in rust
#[pymodule]
fn librust_exceptions(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    Ok(())
}
