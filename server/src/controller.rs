use std::collections::HashMap;

use crate::{error::Result, http_handler::Data, Error};
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn ArticleController(params: Data) -> String {
        println!("hrhr {:#?}", params);
        let mut res = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_string();
        if let Data::Params(e) = params {
            if let Some(x) = e {
                let serial = serde_json::to_string(&x).unwrap_or_default();
                res.push_str(&serial);
            }
            return res;
        }

        return res;
    }
}
