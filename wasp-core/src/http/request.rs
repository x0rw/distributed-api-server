use std::collections::HashMap;

// Custom Request
#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub params: HashMap<String, String>,
    // ...
}

impl Request {
    pub fn new(path: String, method: String) -> Self {
        Request {
            path,
            method,
            params: HashMap::new(),
        }
    }
}
