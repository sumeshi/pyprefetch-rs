extern crate libprefetch;

use self::libprefetch::Prefetch;

pub struct PyPrefetch {
    pub instance: Prefetch,
}

impl PyPrefetch {
    pub fn new(path: String) -> PyPrefetch {
        let file = std::fs::File::open(path).unwrap();
        let instance = Prefetch::new(file).unwrap();
        return PyPrefetch{instance};
    }
}
