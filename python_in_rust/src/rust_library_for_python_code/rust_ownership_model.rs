use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::pyclass;

// pyclass macro give us the option to use python class inside rust code
struct NumberList {
    numbers: Vec<i32>,
}

impl NumberList {
    fn new() -> Self {
        NumberList {
            numbers: Vec::new(),
        }
    }

    fn add_number(&mut self, num: i32) {
        self.nubmers.push(num);
    }

    fn len(&self) -> usize {
        self.numbers.len()
    }

    fn clear(&self) {
        self.numbers.clear();
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn rust_list(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<NumberList>()?;
    m.add_function(wrap_pyfunction!(add_number)?)?;
    m.add_function(wrap_pyfunction!(len)?)?;
    m.add_function(wrap_pyfunction!(clear)?)?;
    Ok(())
}

#[pyfunction]
fn add_number(list: &mut NumberList, num: i32) -> PyResult<()> {
    list.add_number(num);
    Ok()
}

#[pyfunction]
fn len(list: &mut NumberList) -> PyResult<usize> {
    Ok(list.len())
}

#[pyfunction]
fn clear(list: &mut NumberList) -> PyResult<()> {
    list.clear();
    Ok()
}

#[pymethods]
impl NumberList {
    #[new]
    fn new_obj() -> Self {
        NumberList::new()
    }

    fn add(&mut self, value: i32) {
        self.add_number(value);
    }

    fn length(&self) -> usize {
        self.len();
    }

    fn clear_list(&mut self) {
        self.clear();
    }
}

#[pymodule]
fn libownership_pyrust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NumberList>()?;
    Ok(())
}