use crate::http::{
    self,
    builder::{Response, StatusCode},
    handler::Data,
};
use std::collections::HashMap;
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn EchoController(data: Data) -> Response {
        let header = data.header.unwrap().host;
        let serial = serde_json::to_string(&header).unwrap_or_default();
        return Response::JSON(serial, StatusCode::Ok200);
    }
}
