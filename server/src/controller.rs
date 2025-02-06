use crate::{error::Result, Error};
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn ArticleController(params: Vec<String>) -> String {
        let res = format!("HTTP/1.1 200 OK\r\n\r\nART PAGE\n{}", params.join(" "));
        return res;
    }
}

