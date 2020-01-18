extern crate libprefetch;

//use std::collections::HashMap;
use self::libprefetch::Prefetch;

//pub fn hoge(file_path: &'static str) -> HashMap<&str, String> {
//    let file = std::fs::File::open(file_path).unwrap();
//    let prefetch = Prefetch::new(file).unwrap();
//
//    let mut hmap = HashMap::new();
//
//    hmap.insert("name", prefetch.name().to_string());
//    hmap.insert("exec_count", prefetch.execution_counter().to_string());
//    hmap.insert("last_exec_time", prefetch.last_execution_time().to_string());
//
//    for volume in prefetch.volumes().unwrap() {
//        hmap.insert("volume", volume.id().to_string());
//        println!("{}", volume.id());
//    }
//
//    return hmap;
//}

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
