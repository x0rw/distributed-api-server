use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::{thread, time::Duration};

// service registry and discovery
// nodes register themselves into it
// after conecting to /register the node will be added to the service registery
// so that the worker thread can ping it periodically for health
use crate::health::Health;
use base::error::Result;
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
        println!("get route:{}", uri);
        println!("avaliable routes:{:#?}", self.routes);
        return gr;
    }
    pub fn add_service(&mut self, service: Service) {
        println!("Registering a new service {}", service.clone().service_name);
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
    pub fn broadcast(sr: Arc<Mutex<ServiceRegistry>>, service: &Service) {
        println!(
            "Launching ServiceRegistry broadcast bind at:{}",
            &service.inc_address
        );
        let listener = TcpListener::bind(&service.inc_address).unwrap();
        for stream in listener.incoming() {
            let mut buffer = [0u8; 1000];
            let mut stream = stream.unwrap();
            let read = stream.read(&mut buffer).unwrap();
            let buffer = String::from_utf8_lossy(&buffer[..read]);
            if buffer.starts_with("PROTO") {
                let buffer = buffer.split_once(' ').unwrap().1;
                let service: Service = serde_json::from_str(&buffer).unwrap();
                let resp = format!(
                    "Service registered successfully at host:{} for the routes: {:#}",
                    service.inc_address.clone(),
                    service.supported_routes.clone().join(" ")
                );
                stream.write(resp.as_bytes()).unwrap();
                sr.lock().unwrap().add_service(service);
                //..println!("{:#?}", service);
            }
        }
    }
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
impl Service {
    // api gateway listener for ServiceRegistry from nodes
    // hooking to the api gateway
    pub fn discover(&self, host: String) -> Result<()> {
        let mut nb_tries: u8 = 0;
        loop {
            if let Ok(mut stream) = TcpStream::connect(&host) {
                let st = serde_json::to_string(self).unwrap_or_default();

                let mut http = "PROTO ".to_string();
                http.push_str(&st);
                let data = stream.write(http.as_bytes()).unwrap();
                let mut buf = [0; 100];
                let red = stream.read(&mut buf).unwrap();
                println!("recv: {:#?}", String::from_utf8_lossy(&buf[..red]));
                return Ok(());
            }
            sleep(Duration::from_millis(1000));
            nb_tries += 1;
        }
    }
    pub fn forward(&self, data: &str) -> String {
        let mut buffer = [0u8; 1000];
        println!("Requesting {}", self.node_address.to_string());
        let mut stream = TcpStream::connect(self.node_address.to_string()).unwrap();
        let sent_size = stream.write(data.as_bytes()).unwrap();
        let response = stream.read(&mut buffer).unwrap();
        return String::from_utf8_lossy(&buffer).into_owned();
    }

    //initilise the service
    pub fn init(service_name: &str, address: &str, supported_routes: Vec<String>) -> Self {
        println!("Started service {} at {} ", service_name, address);
        Self {
            service_name: service_name.to_string(),
            inc_address: address.to_string(),
            node_address: address.to_string(),
            supported_routes,
            health: Health::default(),
        }
    }
    pub fn ping(&self) {
        //ping to {host}/ping
        todo!()
    }
}
