use std::{mem, vec};

use serde_json::Error;

use crate::{
    auth,
    builder::HttpBuilder,
    error::{self, Result},
    http::header,
};

#[derive(Debug)]
pub enum ContentType {
    JSON,
    HTML,
    Unknown,
}
impl ContentType {
    fn from(word: &str) -> Option<ContentType> {
        match word.trim() {
            "application/json" => Some(ContentType::JSON),
            "text/html" => Some(ContentType::HTML),
            "document/html" => Some(ContentType::HTML),
            _ => {
                return None;
            }
        }
    }
    fn into_str(self) -> &'static str {
        match self {
            ContentType::JSON => "application/json",
            ContentType::HTML => "text/html",
            _ => "Unknown",
        }
    }
}
#[derive(Debug)]
pub struct HttpHeader {
    content_type: Option<ContentType>,
    content_lenght: Option<u32>,
    host: Option<String>,
    authorization: Option<auth::Auth>,
}
impl HttpHeader {
    pub fn new() -> Self {
        Self {
            content_type: None,
            content_lenght: None,
            host: None,
            authorization: None,
        }
    }
    fn set_content_lenght(mut self, content_len: u32) -> Self {
        self.content_lenght = Some(content_len);
        self
    }
    fn set_content_type(mut self, cont_type: ContentType) -> Self {
        self.content_type = Some(cont_type);
        self
    }

    //TODO push_str usage is very expensive
    pub fn build(self) -> String {
        let mut res = String::new();
        if let Some(d) = self.content_type {
            res.push_str("Content-Type:");
            res.push_str(d.into_str());
            res.push_str("\r\n");
        }
        if let Some(d) = self.content_lenght {
            res.push_str("Content-lenght:");
            res.push_str(d.to_string().as_ref());
            res.push_str("\r\n");
        }
        res.push_str("\r\n");
        return res;
    }
    pub fn from(&mut self, headers: &str) -> Result<String> {
        let lines = headers
            .split("\r\n")
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();
        for line in lines {
            match line.split_once(":") {
                Some((key, value)) => {
                    if value.is_empty() || key.is_empty() {
                        return Err(error::Error::EmptyHeaderField);
                    }

                    match key {
                        "Content-Type" => self.content_type = ContentType::from(value),
                        "Content-lenght" => self.content_lenght = value.trim().parse::<u32>().ok(),
                        "Host" => self.host = Some(value.to_string()),
                        //to extend
                        _ => {}
                    }
                }
                None => {
                    //      return Err(error::Error::InvalidHeader);
                }
            }
        }
        Ok(String::new())
    }
}
//Content-Type: text/html \
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_header() {
        let header = format!(
            "Transfer-Encoding: chunked\r\nDate: Sat, 28 Nov 2009 04:36:25 GMT\r\nServer: LiteSpeed\r\nConnection: close\r\nX-Powered-By: W3 Total Cache/0.8\r\nPragma: public\r\nExpires: Sat, 28 Nov 2009 05:36:25 GMT\r\nEtag: \"pub1259380237;gz\"\r\nCache-Control: max-age=3600, public\r\nContent-Type: text/html \r\nLast-Modified: Sat, 28 Nov 2009 03:50:37 GMT\r\nX-Pingback: https://code.tutsplus.com/xmlrpc.php\r\nContent-Encoding: gzip\r\nContent-lenght: 3444\r\nVary: Accept-Encoding, Cookie, User-Agent"
      );
        let mut hb = HttpHeader::new();
        let x = hb.from(&header).unwrap();
        //pm().parse::<u32>().ok()rintln!("===={:#?}", hb.build());
        //        assert_eq!(hb.content_lenght, Some(3444));
    }
    #[test]
    fn valid_big_http_request() {
        let http_header = String::from("Host: 127.0.0.1:1111\r\nConnection: keep-alive\r\nsec-ch-ua: \"Not A(Brand\";v=\"8\", \"Chromium\";v=\"132\", \"Google Chrome\";v=\"132\"\r\nsec-ch-ua-mobile: ?0\r\nsec-ch-ua-platform: \"Linux\"\r\nUpgrade-Insecure-Requests: 1\r\nUser-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/132.0.0.0 Safari/537.36\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7\r\nSec-Fetch-Site: none\r\nSec-Fetch-Mode: navigate\r\nwSec-Fetch-User: ?1\r\nContent-lenght: 2323232\r\nSec-Fetch-Dest: document\r\nAccept-Encoding: gzip, deflate, br, zstd\r\nAccept-Language: fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7\r\n\r\n");
        let mut x = HttpHeader::new();
        let dd = x.from(&http_header);
        println!("\n\n{:#?}\n\n", x);
        //assert_eq!(http_h.data, None);
    }

    #[test]
    #[should_panic]
    fn test_header_corrupt() {
        let header = format!(
            "Transfer-Encoding: chunked\r\nDate: Sat, 28 Nov 2009 04:36:25 GMT\r\nServer Lite"
        );
        let mut hb = HttpHeader::new();
        let x = hb.from(&header).unwrap();
    }
}
