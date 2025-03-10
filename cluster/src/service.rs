use std::thread::sleep;
use std::time::Instant;
use std::{thread, time::Duration};

// service registry and discovery
// nodes register themselves into it
// after conecting to /register the node will be added to the service registery
// so that the worker thread can ping it periodically for health
use crate::health::Health;
use base::error::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub service_name: String,
    pub node_address: String,
    pub inc_address: String,
    pub supported_routes: Vec<String>,
    pub health: Health,
    #[serde(skip)]
    pub timestamp: Option<Instant>,
}
use std::io::{Read, Write};
use std::net::TcpStream;
impl Service {
    pub fn update_time(&mut self) {
        todo!()
    }

    // api gateway listener for ServiceRegistry from nodes
    // hooking to the api gateway
    pub fn discover_gateway(&self, host: String) -> Result<()> {
        let mut _nb_tries: u8 = 0;
        loop {
            if let Ok(mut stream) = TcpStream::connect(&host) {
                let st = serde_json::to_string(self).unwrap_or_default();

                let mut http = "REGISTER ".to_string();
                http.push_str(&st);
                let _data = stream.write(http.as_bytes())?;
                let mut buf = [0; 100];
                let red = stream.read(&mut buf)?;
                println!("recv: {:#?}", String::from_utf8_lossy(&buf[..red]));
                self.heartbeat(host);
                return Ok(());
            }
            sleep(Duration::from_millis(1000));
            _nb_tries += 1;
        }
    }

    pub fn heartbeat(&self, host: String) {
        loop {
            thread::sleep(Duration::from_millis(1000));
            if let Ok(mut stream) = TcpStream::connect(&host) {
                let hb = format!("HEARTBEAT {}", self.service_name);
                let mut buf = [0u8; 100];
                stream.write(hb.as_bytes()).unwrap();
                let size = stream.read(&mut buf).unwrap();
                if buf[..size].starts_with(b"OK") {
                    println!("heartbeat delivered successfully");
                }
            }
        }
    }

    pub fn forward(slf: &Arc<RwLock<Service>>, data: &str) -> String {
        let slf = slf.read().unwrap();
        let mut buffer = [0u8; 1000];
        println!("Forwarding to {}", slf.node_address.to_string());
        let mut stream = TcpStream::connect(slf.node_address.to_string()).unwrap();
        let _sent_size = stream.write(data.as_bytes()).unwrap();
        let response_size = stream.read(&mut buffer).unwrap();
        println!("Recieved response of size {}", response_size);
        return String::from_utf8_lossy(&buffer).into_owned();
    }

    //initilise the service
    pub fn init(
        service_name: &str,
        inc_address: &str,
        node_add: &str,
        supported_routes: Vec<String>,
    ) -> Self {
        println!("Started service {} at {} ", service_name, inc_address);
        Self {
            service_name: service_name.to_string(),
            inc_address: inc_address.to_string(),
            node_address: node_add.to_string(),
            supported_routes,
            health: Health::default(),
            timestamp: Some(Instant::now()),
        }
    }
    pub fn ping(&self) {
        //ping to {host}/ping
        todo!()
    }
}
