use std::{thread, time::Duration};

// service registry and discovery
// nodes register themselves into it
// after conecting to /register the node will be added to the service registery
// so that the worker thread can ping it periodically for health
use crate::health::Health;
use base::http::builder::{HttpBuilder, Response, StatusCode};
use base::http::handler::{HttpMethod, ReqLine};
use base::http::header::{ContentType, HttpHeader};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Service {
    service_name: String,
    address: String,
    health: Health,
}

pub struct ServiceRegistry<'a> {
    pub services: Vec<&'a Service>,
}
impl<'a> ServiceRegistry<'a> {
    fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }
    fn add_service(&mut self, service: &'a Service) {
        self.services.push(service);
    }
    fn worker(self) {
        loop {
            thread::sleep(Duration::from_millis(5000));
            for service in self.services.iter() {
                service.ping();
            }
        }
    }
}

use std::io::{Read, Write};
use std::net::TcpStream;
impl Service {
    pub fn discover(&self, host: String) {
        let mut stream = TcpStream::connect(host).unwrap();
        let st = serde_json::to_string(self).unwrap_or_default();

        //build a ping
        let reql = ReqLine::new(HttpMethod::POST, "/ping".to_string(), 1);
        let ret = reql.build();
        let http = HttpHeader::new()
            .set_content_lenght(st.len() as u32)
            .set_content_type(ContentType::JSON)
            .build(st);
        let http = format!("{ret}\r\n{http}");

        println!("sent: {:#?}", http.clone());
        let data = stream.write(http.as_bytes());
        let mut buf = [0; 100];
        let red = stream.read(&mut buf).unwrap();
        println!(
            "recv: {:#?}",
            String::from_utf8_lossy(&buf).trim().to_string()
        );
    }
    pub fn init(service_name: &str, address: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            address: address.to_string(),
            health: Health::default(),
        }
    }
    pub fn ping(&self) {
        //ping to {host}/ping
        todo!()
    }
}
