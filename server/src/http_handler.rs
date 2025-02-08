use crate::error::{Error, Result};
use crate::routes::{RouteType, RoutesMap};
use crate::utils;
use core::fmt;
use std::collections::HashMap;
use std::fmt::{format, Display};
use std::fs::write;
#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    POST,
    GET,
}

#[derive(Debug)]
pub enum Data<'a> {
    Params(Option<HashMap<&'a str, &'a str>>),
    Body(Option<&'a str>),
}
#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub uri: String,
    pub method: HttpMethod,
    pub params: Data<'a>,
    pub header: HeaderOptions,
    pub data: Option<String>,
}
#[derive(Debug)]
pub struct HeaderOptions {
    pub header: HashMap<String, String>, //we only own useful Header Features
}
impl std::fmt::Display for HeaderOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self
            .header
            .iter()
            .map(|(k, v)| format!("{k} => {v}\n"))
            .collect::<String>();
        println!("{b}");
        Ok(())
    }
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

impl<'a> HttpRequest<'a> {
    fn new(
        method: HttpMethod,
        uri: &'a str,
        header_opt: HeaderOptions,
        data: Option<&str>,
    ) -> Result<Self> {
        let r: Option<String> = match data {
            Some(e) => Some(e.to_string()),
            None => None,
        };
        let e = match uri.split("?").nth(1) {
            Some(e) => Some(
                e.split("&")
                    .map(|x| format!("{x}"))
                    .collect::<Vec<String>>(),
            ),
            None => None,
        };
        let (iniuri, uu) = utils::parse_params(uri);
        Ok(Self {
            uri: String::from(iniuri),
            method: method,
            params: uu,
            header: header_opt,
            data: r,
        })
    }
    pub fn get_data(&self) -> &str {
        match &self.data {
            Some(e) => e.as_ref(),
            None => "",
        }
    }
}

pub fn handle_http<'a>(proc: &'a str) -> Result<HttpRequest<'a>> {
    let ref_s: &str = proc;
    let mut sp = ref_s.split("\r\n");
    let req = sp.next().ok_or(Error::NullHeaderReq)?;

    //println!("{}", proc.clone());
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
        )?),
        Some("POST") => Ok(HttpRequest::new(
            HttpMethod::POST,
            words.next().unwrap(),
            header_opt,
            req_data,
        )?),
        _ => Err(Error::UnknowenHttpMethod),
    }
}

//currentlu not in use
pub enum HttpResponseCode {
    Ok200,
    NotFound404,
    MovedPerm301(String),
}

//builds http request
pub struct HttpBuilder {
    pub data: String,
}

impl HttpBuilder {
    pub fn build_badrequest() -> String {
        format!("HTTP/1.1 400 Bad Request\r\n").to_string()
    }
    pub fn build(route: &RouteType, handler: HttpRequest, router: &RoutesMap) -> HttpBuilder {
        let mut httpbuilt = HttpBuilder {
            data: match route {
                RouteType::NotFound => {
                    format!("HTTP/1.0 404 Not Found\r\n{}", router.getErrorRoute()).to_string()
                }
                RouteType::Data(html) => format!("HTTP/1.1 200 OK\r\n\r\n{}", html).to_string(),
                RouteType::Redirect(e, _) => {
                    format!("HTTP/1.1 301 Moved Permanently\r\nLocation:{}", e).to_string()
                }
                RouteType::Controller(func) => {
                    let params = handler.params;
                    func(params)
                }
            },
        };
        match handler.method {
            HttpMethod::GET => return httpbuilt,
            HttpMethod::POST => {
                //httpbuilt.data.push_str(handler.data.unwrap_or_default());
                return httpbuilt;
            }
        }
    }
}
#[cfg(test)]
#[warn(clippy::used_underscore_binding)]
mod tests {
    use super::*;
    #[test]
    fn valid_http_get_request() {
        let http_header = String::from("GET / HTTP/1.1\r\nHOST:hello.com");
        let http_h = handle_http(&http_header).unwrap();

        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HttpMethod::GET);
    }
    #[test]
    fn http_req_test() {
        let testvecs = vec![
            "/?dsd=fefd&df=fffff",
            "/Article?id=34&username=fdfdf",
            "/Article?dsd=fefd&df=fffff",
        ];
        for test_uri in testvecs {
            let _a = HttpRequest::new(
                HttpMethod::POST,
                "?dsd=fefd&df=fffff",
                HeaderOptions::new(),
                None,
            );
        }
    }
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
        let http_h = handle_http(&http_header).unwrap();
        assert_eq!(http_h.uri, "/f");
        assert_eq!(http_h.data, None);
        assert_eq!(http_h.method, HttpMethod::GET);
    }

    #[test]
    fn valid_http_post_request() {
        let http_header = String::from("POST / HTTP/1.1\r\n");
        let http_h = handle_http(&http_header).unwrap();

        assert_eq!(http_h.uri, "/");
        assert_eq!(http_h.method, HttpMethod::POST);
    }

    #[test]
    #[should_panic]
    fn unvalid_http_method_request() {
        let http_header = String::from("HACK / HTTP/1.1");
        let _http_h = handle_http(&http_header).unwrap();
    }
    #[test]
    #[should_panic]
    fn unvalid_header_size() {
        let http_header = String::from("POST / HTTP/1.1 HELLO");
        let _http_h = handle_http(&http_header).unwrap();
    }
}
// testing the http parsing using NetCat
// echo -ne "GET / HTTP/1.1\r\nHost: 127.0.0.1\r\nX-Custom-Header: \x80\x81\x82\r\n\r\n" | nc
// 127.0.0.1 11112
