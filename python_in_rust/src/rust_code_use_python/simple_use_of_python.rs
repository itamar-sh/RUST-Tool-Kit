use pyo3::prelude::*;


fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();  // Release Python GLI
    let value = vec![1, 2, 3];
    print!("Passing values to Python sum: {:?}\n", value);
    Python::with_gil(|py|{
        let builtins = PyModule::import(py, "builtins")?;
        let total: i32 = builtins.getattr("sum")?.call1((value,))?.extract()?;
        print!("total sum from Python: {}\n", total);
        let os = PyModule::import(py, "os")?;
        let user: String = os.getattr("getenv")?.call1(("USER",))?.extract()?;
        print!("User from Python: {}\n", user);
        Ok(())
    })
}