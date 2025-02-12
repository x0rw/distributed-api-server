use std::collections::HashMap;

use crate::{
    error::Result,
    http_builder::{Response, StatusCode},
    http_handler::Data,
    Error,
};
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn EchoController(data: Data) -> Response {
        let mut res = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_string();
        let s = data.params.unwrap_or_default();
        let header = data.header;
        let serial = serde_json::to_string(&s).unwrap_or_default();
        res.push_str(&serial);
        return Response::JSON(res, StatusCode::Ok200);
    }
}
