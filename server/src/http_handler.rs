use crate::routes;
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq)]
pub enum HTTP_METHOD {
    POST,
    GET,
}
#[derive(Debug)]
pub struct http_req {
    pub uri: String,
    pub method: HTTP_METHOD,
    pub header: header_options,
    pub data: Option<String>,
}
#[derive(Debug)]
struct header_options {
    header: HashMap<String, String>, //we only own useful Header Features
}
impl header_options {
    fn new() -> Self {
        Self {
            header: HashMap::new(),
        }
    }
    fn add(&mut self, option: &str, value: &str) {
        self.header
            .insert(String::from(option), String::from(value));
    }
    fn getLenght(self) -> Option<u32> {
        if let Some(e) = self.header.get("data_len") {
            return Some(e.parse::<u32>().unwrap());
        }
        None
    }
}
impl http_req {
    fn new(method: HTTP_METHOD, uri: &str, header_opt: header_options, data: Option<&str>) -> Self {
        let r: Option<String> = match data {
            Some(e) => Some(e.to_string()),
            None => None,
        };
        Self {
            uri: String::from(uri),
            method: method,
            header: header_opt,
            data: r,
        }
    }
}
pub fn handle_http(proc: String) -> http_req {
    let mut sp = proc.split("\r\n");
    let req = sp.next().unwrap();
    let header = sp.next().unwrap();
    let mut header_opt = header_options::new();
    let mut req_data: Option<&str> = None;

    print!("rrrrrr{}rrrrrrr", proc);
    // header options iterating
    //
    while let Some(e) = sp.next() {
        if e.contains(":") {
            let mut s = e.split(":");
            header_opt.add(s.next().unwrap(), s.next().unwrap());
        }
        if e.is_empty() {
            if let Some(ed) = sp.next() {
                let ed = ed.trim_matches(char::from(0));
                if ed.is_empty() {
                    req_data = None;
                } else {
                    req_data = Some(ed);
                }
            }
            break;
        }
    }

    let mut words = req.split_whitespace();
    if words.clone().count() != 3 {
        panic!("unvalid http header size{:?}", words.collect::<Vec<_>>());
    }
    match words.next() {
        Some("GET") => http_req::new(
            HTTP_METHOD::GET,
            words.next().unwrap(),
            header_opt,
            req_data,
        ),
        Some("POST") => http_req::new(
            HTTP_METHOD::POST,
            words.next().unwrap(),
            header_opt,
            req_data,
        ),
        _ => panic!("Unvalid Http Methode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_http_get_request() {
        let http_header = String::from("GET / HTTP/1.1\r\nHOST:hello.com");
        let http_h = handle_http(http_header);

        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HTTP_METHOD::GET);
    }

    #[test]
    fn valid_http_post_request() {
        let http_header = String::from("POST / HTTP/1.1\r\n");
        let http_h = handle_http(http_header);

        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HTTP_METHOD::POST);
    }

    #[test]
    #[should_panic]
    fn unvalid_http_method_request() {
        let http_header = String::from("HACK / HTTP/1.1");
        let http_h = handle_http(http_header);
    }

    #[test]
    #[should_panic]
    fn unvalid_header_size() {
        let http_header = String::from("POST / HTTP/1.1 HELLO");
        let http_h = handle_http(http_header);
    }
}
