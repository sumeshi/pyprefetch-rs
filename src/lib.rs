extern crate pyo3;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod hoge;

#[pyfunction]
fn get_dict(_py: Python<'_>, filePath: &'static str) -> PyObject {
    let dict = PyObject::from(hoge::hoge(filePath).to_object(_py));
    return dict;
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn prefetch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_dict))?;

    Ok(())
}