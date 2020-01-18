use std::collections::HashMap;

pub fn hoge(filePath: &'static str) -> HashMap<&'static str, &'static str> {
    let mut hmap = HashMap::new();
    hmap.insert("hoge", filePath);
    return hmap;
}