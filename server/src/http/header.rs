use std::{mem, vec};

use serde_json::Error;

use crate::{
    builder::HttpBuilder,
    error::{self, Result},
};

#[derive(Debug)]
pub enum AuthType {
    Bearer,
    Basic,
    Digest,
    OAuth,
}
#[derive(Debug)]
pub enum ContentType {
    JSON,
    Unknown,
}
impl ContentType {
    fn from(word: &str) -> Option<ContentType> {
        match word {
            "application/json" => Some(ContentType::JSON),
            _ => None,
        }
    }
}
#[derive(Debug)]
struct Auth {
    auth_type: AuthType,
    value: String,
}
#[derive(Debug)]
pub struct HttpHeader {
    content_type: Option<ContentType>,
    content_lenght: Option<u32>,
    host: Option<String>,
    authorization: Option<Auth>,
}
impl HttpHeader {
    fn new() -> Self {
        Self {
            content_type: None,
            content_lenght: None,
            host: None,
            authorization: None,
        }
    }
    fn from(&mut self, headers: &str) -> Result<String> {
        let lines = headers
            .split("\r\n")
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();
        for line in lines {
            match line.split_once(":") {
                Some((key, value)) => {
                    println!("{:#?}", key);
                    if value.is_empty() || key.is_empty() {
                        return Err(error::Error::EmptyHeaderField);
                    }

                    match key {
                        "Content-Type" => self.content_type = ContentType::from(value),
                        "Content-lenght" => self.content_lenght = value.trim().parse::<u32>().ok(),
                        //to extend
                        _ => {}
                    }
                }
                None => return Err(error::Error::InvalidHeader),
            }
        }
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;

    use super::*;
    #[test]
    fn test_header() {
        let header = format!(
            "Transfer-Encoding: chunked\r\nDate: Sat, 28 Nov 2009 04:36:25 GMT\r\nServer: LiteSpeed\r\nConnection: close\r\nX-Powered-By: W3 Total Cache/0.8\r\nPragma: public\r\nExpires: Sat, 28 Nov 2009 05:36:25 GMT\r\nEtag: \"pub1259380237;gz\"\r\nCache-Control: max-age=3600, public\r\nContent-Type: text/html; charset=UTF-8\r\nLast-Modified: Sat, 28 Nov 2009 03:50:37 GMT\r\nX-Pingback: https://code.tutsplus.com/xmlrpc.php\r\nContent-Encoding: gzip\r\nContent-lenght: 3444\r\nVary: Accept-Encoding, Cookie, User-Agent"
      );
        let mut hb = HttpHeader::new();
        let x = hb.from(&header);
        println!("===={:#?}", hb);
        assert_eq!(hb.content_lenght, Some(3444));
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
