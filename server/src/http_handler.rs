use crate::error::{Error, Result};
use crate::routes;
use std::collections::HashMap;
use std::fmt::format;
#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    POST,
    GET,
}
#[derive(Debug)]
pub struct HttpRequest {
    pub uri: String,
    pub method: HttpMethod,
    pub header: HeaderOptions,
    pub data: Option<String>,
}
#[derive(Debug)]
struct HeaderOptions {
    header: HashMap<String, String>, //we only own useful Header Features
}

impl HeaderOptions {
    fn new() -> Self {
        Self {
            header: HashMap::new(),
        }
    }
    fn add(&mut self, option: &str, value: &str) {
        self.header
            .insert(String::from(option), String::from(value));
    }
    fn get_lenght(self) -> Option<u32> {
        if let Some(e) = self.header.get("data_len") {
            return Some(e.parse::<u32>().unwrap());
        }
        None
    }
}
impl HttpRequest {
    fn new(method: HttpMethod, uri: &str, header_opt: HeaderOptions, data: Option<&str>) -> Self {
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
    pub fn get_data(&self) -> &str {
        match &self.data {
            Some(e) => e.as_ref(),
            None => "",
        }
    }
}
pub fn handle_http(proc: String) -> Result<HttpRequest> {
    let ref_s = &proc;
    let mut sp = ref_s.split("\r\n");
    let req = sp.next().ok_or(Error::NullHeaderReq)?;

    let mut words = req.split_whitespace();
    if words.clone().count() != 3 {
        //cloning is cheap because we clone the internal state of an
        //iterator type &str
        return Err(Error::InvalidHttpReqSize);
    }
    // init options (for performance i should move it outside later and reuse the same structure)
    let mut header_opt = HeaderOptions::new();
    let mut req_data: Option<&str> = None;
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

    match words.next() {
        Some("GET") => Ok(HttpRequest::new(
            HttpMethod::GET,
            words.next().unwrap(),
            header_opt,
            req_data,
        )),
        Some("POST") => Ok(HttpRequest::new(
            HttpMethod::POST,
            words.next().unwrap(),
            header_opt,
            req_data,
        )),
        _ => Err(Error::UnknowenHttpMethod),
    }
}
pub enum HttpResponseCode {
    Ok200,
    NotFound404,
    MovedPerm301(String),
}
pub struct HttpBuilder {
    data: String,
}

impl HttpBuilder {
    pub fn response(res_code: HttpResponseCode, data: &str) -> String {
        let header_req = match res_code {
            HttpResponseCode::Ok200 => format!("HTTP/1.1 200 OK\r\n\r\n{}", data).to_string(),
            HttpResponseCode::NotFound404 => {
                format!("HTTP/1.0 404 Not Found\r\n{}", data).to_string()
            }
            HttpResponseCode::MovedPerm301(e) => {
                format!("HTTP/1.1 301 Moved Permanently\r\nLocation:{}", e).to_string()
            }
        };
        return header_req;
    }
}
#[cfg(test)]
#[warn(clippy::used_underscore_binding)]
mod tests {
    use super::*;
    #[test]
    fn valid_http_get_request() {
        let http_header = String::from("GET / HTTP/1.1\r\nHOST:hello.com");
        let http_h = handle_http(http_header).unwrap();

        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HttpMethod::GET);
    }
    #[test]
    fn valid_big_http_request() {
        let http_header = String::from("GET /f HTTP/1.1\r\n
            Host: 127.0.0.1:1111\r\n
            Connection: keep-alive\r\n
            sec-ch-ua: \"Not A(Brand\";v=\"8\", \"Chromium\";v=\"132\", \"Google Chrome\";v=\"132\"\r\n
            sec-ch-ua-mobile: ?0\r\n
            sec-ch-ua-platform: \"Linux\"\r\n
            Upgrade-Insecure-Requests: 1\r\n
            User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36\r\n
            Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7\r\n
            Sec-Fetch-Site: none\r\n
            Sec-Fetch-Mode: navigate\r\n
            Sec-Fetch-User: ?1\r\n
            Sec-Fetch-Dest: document\r\n
            Accept-Encoding: gzip, deflate, br, zstd\r\n
            Accept-Language: fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7\r\n
            \r\n");
        let http_h = handle_http(http_header).unwrap();
        assert_eq!(http_h.uri, "/f");
        assert_eq!(http_h.data, None);
        assert_eq!(http_h.method, HttpMethod::GET);
    }

    #[test]
    fn valid_http_post_request() {
        let http_header = String::from("POST / HTTP/1.1\r\n");
        let http_h = handle_http(http_header).unwrap();

        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HttpMethod::POST);
    }

    #[test]
    #[should_panic]
    fn unvalid_http_method_request() {
        let http_header = String::from("HACK / HTTP/1.1");
        let _http_h = handle_http(http_header).unwrap();
    }
    #[test]
    #[should_panic]
    fn unvalid_header_size() {
        let http_header = String::from("POST / HTTP/1.1 HELLO");
        let _http_h = handle_http(http_header).unwrap();
    }
}
