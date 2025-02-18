use std::{mem, vec};

use serde_json::Error;

use crate::{
    error::{self, Error, Result},
    http_builder::HttpBuilder,
};

pub enum AuthType {
    Bearer,
    Basic,
    Digest,
    OAuth,
}
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
struct Auth {
    auth_type: AuthType,
    value: String,
}
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
            .clone()
            .split("\r\n")
            .filter(|&x| !x.is_empty())
            .collect::<String>();
        match lines.split_once(":") {
            Some((key, value)) => {
                if value.is_empty() || key.is_empty() {
                    return Err(error::Error::EmptyHeaderField);
                }
                match key {
                    "content_type" => self.content_type = ContentType::from(value),
                    "content_lenght" => self.content_lenght = value.parse::<u32>().ok(),
                    _ => {}
                }
            }
            None => return Err(error::Error::InvalidHeader),
        }

        Ok(String::new())
    }
}
