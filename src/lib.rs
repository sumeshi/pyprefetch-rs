extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

mod prefetch;

#[pyclass]
struct Prefetch {
    p: prefetch::PyPrefetch,
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

        let name: String = p.instance.name().to_string();
        let exec_count: usize = p.instance.execution_counter();
        let last_exec_time: u64 = p.instance.last_execution_time();
        Prefetch {p, name, exec_count, last_exec_time}
    }

    fn get_metrics(&mut self, _py: Python) -> PyObject {
        let list = PyList::empty(_py);
        for metric in self.p.instance.metrics().unwrap() {
            let dict = PyDict::new(_py);
            dict.set_item("id", metric.id());
            dict.set_item("filename", metric.filename());
            dict.set_item("start_time", metric.start_time().unwrap());
            dict.set_item("duration", metric.duration().unwrap());
            list.append(dict);
        }
        return PyObject::from(list);
    }

    fn get_volumes(&mut self, _py: Python) -> PyObject {
        let list = PyList::empty(_py);
        for volume in self.p.instance.volumes().unwrap() {
            let dict = PyDict::new(_py);
            dict.set_item("id", volume.id());
            dict.set_item("path", volume.device_path());
            dict.set_item("creation_time", volume.creation_time());
            dict.set_item("serial_number", volume.serial_number());

            let directories = PyList::empty(_py);
            for directory in volume.directories().unwrap() {
                directories.append(directory);
            }
            dict.set_item("directories", directories);
            list.append(dict);
        }
        return PyObject::from(list);
    }
}

#[pymodule]
fn prefetch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Prefetch>()?;
    Ok(())
}