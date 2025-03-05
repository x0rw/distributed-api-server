use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::{thread, time::Duration};

// service registry and discovery
// nodes register themselves into it
// after conecting to /register the node will be added to the service registery
// so that the worker thread can ping it periodically for health
use crate::health::Health;
use base::error::{self, Result};
use base::routes;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    pub service_name: String,
    pub node_address: String,
    pub inc_address: String,
    pub supported_routes: Vec<String>,
    pub health: Health,
}

#[derive(Debug)]
pub struct ServiceRegistry {
    pub services: Vec<Service>,
    pub routes: HashMap<String, Service>,
}
impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            routes: HashMap::new(),
        }
    }

    pub fn get_route(&self, uri: &str) -> Option<&Service> {
        let gr = self.routes.get(uri);
        println!("Requesting route:{}", uri);
        //println!("avaliable routes:{:#?}", self.routes);
        return gr;
    }

    pub fn add_service(&mut self, service: Service) {
        println!("Registering a new service {:#?}", service.clone());
        let s_routes = service.clone().supported_routes;
        s_routes
            .into_iter()
            .map(|x| self.routes.insert(x, service.clone()))
            .for_each(|_| ());

        self.services.push(service);
        println!("{:#?}", self.routes.keys());
    }
    fn worker(self) {
        loop {
            thread::sleep(Duration::from_millis(5000));
            for service in self.services.iter() {
                service.ping();
            }
        }
    }
    //if the broadcast doesnt recieve any heartbeat in 5 seconds its removed from the registery
    pub fn health_checker(&self) {
        loop {}
    }
    pub fn broadcast(sr: Arc<Mutex<ServiceRegistry>>, service: &Service) -> error::Result<()> {
        println!(
            "Launching ServiceRegistry broadcast bind at:{}",
            &service.inc_address
        );
        let listener = TcpListener::bind(&service.inc_address).unwrap();
        for stream in listener.incoming() {
            let mut buffer = [0u8; 1000];
            let mut stream = stream?;
            let read = stream.read(&mut buffer)?;
            let buffer = String::from_utf8_lossy(&buffer[..read]);

            if buffer.starts_with("REGISTER") {
                let buffer = buffer.split_once(' ').unwrap().1;
                let service: Service = serde_json::from_str(&buffer).unwrap();
                let resp = format!(
                    "Service registered successfully at host:{} for the routes: {:#}",
                    service.inc_address,
                    service.supported_routes.join(" ")
                );
                stream.write(resp.as_bytes())?;
                sr.lock().unwrap().add_service(service);
                //..println!("{:#?}", service);
            } else if buffer.starts_with("HEARTBEAT") {
                let buffer = buffer.split_once(' ').unwrap().1;
                //println!("Recieved signal at the broadcast: {}", buffer);
            }
        }
        return Ok(());
    }
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
impl Service {
    // api gateway listener for ServiceRegistry from nodes
    // hooking to the api gateway
    pub fn discover_gateway(&self, host: String) -> Result<()> {
        let mut nb_tries: u8 = 0;
        loop {
            if let Ok(mut stream) = TcpStream::connect(&host) {
                let st = serde_json::to_string(self).unwrap();

                let mut http = "REGISTER ".to_string();
                http.push_str(&st);
                let data = stream.write(http.as_bytes()).unwrap();
                let mut buf = [0; 100];
                let red = stream.read(&mut buf).unwrap();
                println!("recv: {:#?}", String::from_utf8_lossy(&buf[..red]));
                self.heartbeat(host);
                return Ok(());
            }
            sleep(Duration::from_millis(1000));
            nb_tries += 1;
        }
    }

    pub fn heartbeat(&self, host: String) {
        loop {
            thread::sleep(Duration::from_millis(1000));
            if let Ok(mut stream) = TcpStream::connect(&host) {
                let mut hb = format!("HEARTBEAT {}", self.service_name);
                let mut buf = [0u8; 100];
                stream.write(hb.as_bytes()).unwrap();
                let size = stream.read(&mut buf).unwrap();
                if buf[..size].starts_with(b"OK") {
                    println!("heartbeat delivered successfully");
                }
            }
        }
    }

    pub fn forward(&self, data: &str) -> String {
        let mut buffer = [0u8; 1000];
        println!("Forwarding to {}", self.node_address.to_string());
        let mut stream = TcpStream::connect(self.node_address.to_string()).unwrap();
        let sent_size = stream.write(data.as_bytes()).unwrap();
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
        }
    }
    pub fn ping(&self) {
        //ping to {host}/ping
        todo!()
    }
}
