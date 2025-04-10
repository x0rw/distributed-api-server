use std::collections::HashMap;

pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Response {
            status,
            body,
            headers: HashMap::new(),
        }
    }

    pub fn set_header(&mut self, name: String, value: String) {
        self.headers.insert(name, value);
    }
}
