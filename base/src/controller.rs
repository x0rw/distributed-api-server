use serde::Deserialize;
use serde::Serialize;

use crate::http::{
    self,
    builder::{Response, StatusCode},
    handler::Data,
};
use std::{thread::sleep, time::Duration};
#[derive(Serialize, Deserialize, Debug)]
pub enum HealthStatus {
    Unhealthy,
    Healthy,
    Degraded,
    Starting,
    OffService,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Health {
    status: HealthStatus,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            status: HealthStatus::OffService,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    service_name: String,
    address: String,
    health: Health,
}

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
    pub fn PingController(data: Data) -> Response {
        let bd = &data.body.unwrap();
        let body: Service = serde_json::from_str(bd).unwrap();
        println!("{:#?}", body);
        let ping_ok = "ok";
        let serial = serde_json::to_string(&ping_ok);
        return Response::JSON(serial.unwrap_or_default(), StatusCode::Ok200);
    }
}
