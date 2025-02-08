use std::collections::HashMap;

use crate::{error::Result, http_handler::Data, Error};
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn ArticleController(params: Data) -> String {
        let res = "HTTP/1.1 200 OK\r\n\r\nART PAGE\n{}".to_string();
        if let Data::Params(e) = params {
            let res = format!(
                "HTTP/1.1 200 OK\r\n\r\nART PAGE\n{}",
                e.iter()
                    .map(|(k, v)| format!("{k} {v}"))
                    .collect::<String>()
            );
            return res;
        }

        return res;
    }
}
