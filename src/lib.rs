extern crate pyo3;

use pyo3::prelude::*;
//use pyo3::wrap_pyfunction;

mod prefetch;

#[pyclass]
struct Prefetch {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    exec_count: usize,
    #[pyo3(get)]
    last_exec_time: u64,
}

#[pymethods]
impl Prefetch {
    #[new]
    fn new(paths: String) -> Self {
        let p: prefetch::PyPrefetch = prefetch::PyPrefetch::new(paths);

        let exec_count: usize = p.instance.execution_counter();
        //let last_exec_time: i32 = prefetch::get_last_exec_time();
        let last_exec_time: u64 = p.instance.last_execution_time();
        let name: String = "test.pf".to_string();
        //return Prefetch {paths, name, exec_count, last_exec_time};
        Prefetch {name, exec_count, last_exec_time}
    }
}

//#[pyfunction]
//fn parse_json(_py: Python<'_>, file_path: &'static str) -> PyObject {
//    let dict = PyObject::from(prefetch::hoge(file_path).to_object(_py));
//    return dict;
//}

/// This module is a python module implemented in Rust.
#[pymodule]
fn prefetch(_py: Python, m: &PyModule) -> PyResult<()> {
    //m.add_wrapped(wrap_pyfunction!(parse_json))?;
    m.add_class::<Prefetch>()?;

    Ok(())
}