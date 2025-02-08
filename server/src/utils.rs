use std::collections::HashMap;

use crate::http_handler::{self, Data};

pub fn parse_params(url: &str) -> (&str, Data) {
    let (inituri, url) = url.split_once("?").unwrap_or_default();
    let mut hm = HashMap::new();
    let u = url.split("&").map(|x| {
        let y = x
            .split("=")
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|x| hm.insert(x[0], x[1]));
    });
    return (inituri, http_handler::Data::Params(hm));
}
