use std::{collections::HashMap, fs::File, io::Read};

pub struct RoutesMap {
    hm: HashMap<String, String>,
}
impl RoutesMap {
    pub fn new() -> Self {
        Self { hm: HashMap::new() }
    }
    pub fn load(&mut self, route: String, file: String) {
        println!("Loading {}", file);
        let mut fi = File::open(file).unwrap();
        let mut contents = String::new();
        fi.read_to_string(&mut contents);
        self.hm.insert(route, contents);
    }
    pub fn get(&self, k: &str) -> Option<&String> {
        self.hm.get(k)
    }
}
