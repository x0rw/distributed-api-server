use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use base::{
    error::Result,
    http::{builder::HttpBuilder, handler::ReqLine},
    routes::RoutesMap,
};

use crate::service::ServiceRegistry;
pub struct Gateway {
    pub hostaddr: String,
    pub listener: TcpListener,
    pub routes: RoutesMap,
    pub service_registry: Arc<Mutex<ServiceRegistry>>,
}
// the gatewat should be able to read the request line and forward the request to the corresponding
// node in ServiceRegistry
// i think ServiceRegistry should have fn forward(&self, &str) -> Service;
impl Gateway {
    pub fn new(
        hostaddr: String,
        routes: RoutesMap,
        service_registry: Arc<Mutex<ServiceRegistry>>,
    ) -> Self {
        Self {
            hostaddr: hostaddr.clone(),
            listener: TcpListener::bind(hostaddr).unwrap(),
            routes,
            service_registry,
        }
    }
    pub fn launch(self: Arc<Self>) -> Result<()> {
        for stream in self.listener.incoming() {
            self.handle_client(stream.unwrap())?;
        }
        Ok(())
    }

    pub fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
        //println!("Client Connected");
        let mut buffer = [0; 1000];
        let size = stream.read(&mut buffer)?;
        let buffer_utf8 = String::from_utf8_lossy(&buffer[..size]).to_string();

        let (line, rest) = buffer_utf8.split_once("\r\n").unwrap();

        let rl = ReqLine::parse_req_line(line).unwrap();
        let uri = match rl.uri.split_once('?') {
            Some((f, _)) => f,
            None => &rl.uri,
        };
        let srr = self.service_registry.lock().unwrap();
        if let Some(ser) = srr.get_route(&uri) {
            let rec = ser.forward(&buffer_utf8);
            println!("forwarding {}", rec.clone());
            let stream_send = stream.write(rec.as_bytes())?;
            println!("{stream_send} Bytes sent to the client");
        } else {
            stream.write(HttpBuilder::build_badrequest().as_bytes());
        }

        //        println!("{:#?}", self.service_registry.services);

        Ok(())
    }
}
