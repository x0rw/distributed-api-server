use error::{self, Result};
use std::collections::HashMap;
use utils;

use super::header::HttpHeader;

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    POST,
    GET,
    Unknowen,
}
impl HttpMethod {
    fn from(method: &str) -> HttpMethod {
        match method {
            "POST" => HttpMethod::POST,
            "GET" => HttpMethod::GET,
            _ => HttpMethod::Unknowen,
        }
    }
    fn into_str(method: HttpMethod) -> String {
        match method {
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::Unknowen => "Unknowen".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Data {
    pub header: Option<HttpHeader>,
    pub params: Option<HashMap<String, String>>,
    pub body: Option<String>,
}
impl Data {
    fn new() -> Self {
        Self {
            params: None,
            body: None,
            header: None,
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub req_line: ReqLine,
    pub data: Data,
}

// req: GET /echo?ds=dd&house=fd HTTP/1.1
#[derive(Debug)]
pub struct ReqLine {
    pub method: HttpMethod,
    pub uri: String,
    pub http_version: u8,
}
impl ReqLine {
    pub fn parse_req_line(req: &str) -> Result<ReqLine> {
        let mut req_iter = req.split(' ');
        let method = req_iter.next().unwrap();
        let uri = req_iter.next().unwrap().to_string();
        let http_version = req_iter.next().unwrap();
        let d = HttpMethod::from(method);
        Ok(ReqLine {
            method: d,
            uri,
            http_version: 9,
        })
    }
}

// handle_http: disects raw http str into a struct
// HttpRequest
// todo later besides memory optimisation: process routes from the req_line to avoid further
// parsing in case of wrong routes and it will give us the ability to perform early anomality checks on
// the url
pub fn handle_http(raw_http: &str) -> Result<HttpRequest> {
    // split the first request line and the header and body from the raw http
    let header = raw_http.to_string();
    let (header, body) = header
        .split_once("\r\n\r\n")
        .ok_or(error::Error::InvalidHeader)
        .unwrap(); // double crlf
                   //    if it does have a header then parse it if not parse only the req_line
    let mut req_line: ReqLine;
    let mut http_header = HttpHeader::new();
    if let Some((nreq_line, rest)) = header.split_once("\r\n") {
        req_line = ReqLine::parse_req_line(nreq_line).unwrap();
        http_header = HttpHeader::new().from(rest).unwrap();
    } else {
        req_line = ReqLine::parse_req_line(header).unwrap();
    }

    println!("{:#?}", req_line);
    let (path, rest) = utils::parse_params(&req_line.uri);
    req_line.uri = path;
    let mut data = Data::new();
    data.params = rest;
    data.header = Some(http_header);
    data.body = Some(body.to_string());

    return Ok(HttpRequest {
        req_line: req_line,
        data: data,
    });
}

//currentlu not in use

#[cfg(test)]
#[warn(clippy::used_underscore_binding)]
mod tests {
    use crate::http::header::HttpHeader;

    use super::*;
    #[test]
    fn valid_http_get_request() {
        let http_header = String::from("GET / HTTP/1.1\r\nHOST:hello.com\r\n\r\n");
        let http_h = handle_http(&http_header).unwrap();

        assert_eq!(http_h.req_line.uri, "/");
        assert_eq!(http_h.req_line.method, HttpMethod::GET);
    }
    #[test]
    fn http_req_test() {
        let testvecs = vec![
            "/?dsd=fefd&df=fffff",
            "/Article?id=34&username=fdfdf",
            "/Article?dsd=fefd&df=fffff",
        ];
        for test_uri in testvecs {
            //let _a = HttpRequest::new(
            //HttpMethod::POST,
            //    "?dsd=fefd&df=fffff",
            //    Some(HeaderOptions::new()),
            //    None,
            //);
        }
    }
    #[test]
    fn valid_big_http_request() {
        let http_header = String::from("Host: 127.0.0.1:1111\r\nConnection: keep-alive\r\nsec-ch-ua: \"Not A(Brand\";v=\"8\", \"Chromium\";v=\"132\", \"Google Chrome\";v=\"132\"\r\nsec-ch-ua-mobile: ?0\r\nsec-ch-ua-platform: \"Linux\"\r\nUpgrade-Insecure-Requests: 1\r\nUser-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7\r\nSec-Fetch-Site: none\r\nSec-Fetch-Mode: navigate\r\nwSec-Fetch-User: ?1\r\nContent-lenght: 2323232\r\nSec-Fetch-Dest: document\r\nAccept-Encoding: gzip, deflate, br, zstd\r\nAccept-Language: fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7\r\n\r\n");
        let mut x = HttpHeader::new();
        let dd = x.from(&http_header);
        //        println!("\n\n{:#?}\n\n", x);
        //assert_eq!(http_h.data, None);
    }

    #[test]
    fn valid_http_post_request() {
        let http_header = String::from("POST / HTTP/1.1\r\n\r\n");
        let http_h = handle_http(&http_header).unwrap();

        println!("-=-=-=-=-={:#?}-=-==-=-=-=-", http_h);
        assert_eq!(http_h.req_line.method, HttpMethod::POST);
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
