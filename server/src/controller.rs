use std::collections::HashMap;

use crate::{error::Result, Error};
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn ArticleController(params: HashMap<&str, &str>) -> String {
        let res = format!(
            "HTTP/1.1 200 OK\r\n\r\nART PAGE\n{}",
            params
                .iter()
                .map(|(k, v)| format!("{k} {v}"))
                .collect::<String>()
        );
        return res;
    }
}
