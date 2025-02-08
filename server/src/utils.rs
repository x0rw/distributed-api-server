use std::collections::HashMap;

use crate::http_handler::{self, Data};

pub fn parse_params(url: &str) -> (&str, Data) {
    match url.split_once("?") {
        Some((uri, res)) => {
            let mut hm = HashMap::new();
            //rememver to use for_each instead of map
            let _d = res.split("&").for_each(|x| {
                let y = x.split("=").collect::<Vec<&str>>().chunks(2).for_each(|x| {
                    hm.insert(x[0], x[1]);
                });
            });
            return (uri, http_handler::Data::Params(Some(hm)));
        }
        None => return (url, Data::Params(None)),
    };
}
