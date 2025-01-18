use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn calculate_pi(iterations) -> PyResult<f64> {
    let mut pi = 0.0;
    for k in 0..iterations {
        pi += ((-1.0f64).pow(k as i32) / (2 * k + 1) as f64) * 4.0;
    }
    Ok(pi)
}

/// A Python module implemented in Rust.
#[pymodule]
fn libdigitspi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_pi, m)?)?;
    Ok(())
}
