use crate::http::{
    self,
    builder::{Response, StatusCode},
    handler::Data,
};
use std::{collections::HashMap, thread::sleep, time::Duration};
pub struct Controller {
    count: u32,
}
impl Controller {
    pub fn EchoController(data: Data) -> Response {
        sleep(Duration::from_millis(100));
        let header = data.header.unwrap().host;
        let serial = serde_json::to_string(&data.params).unwrap_or_default();
        return Response::JSON(serial, StatusCode::Ok200);
    }
}
