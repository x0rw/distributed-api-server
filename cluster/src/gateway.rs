use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, RwLock},
    thread,
};

use base::{
    error::{self, Result},
    http::{builder::HttpBuilder, handler::ReqLine},
};

use crate::{
    service::Service,
    service_registry::{Router, ServiceRegistry},
};

pub struct Gateway {
    pub interface_addr: String,
    pub inc_address: String,
    pub listener: Option<TcpListener>,
    pub service_registry: Arc<Mutex<ServiceRegistry>>,
    pub router: Arc<Mutex<Router>>,
}

// the gatewat should be able to read the request line and forward the request to the corresponding
// node in ServiceRegistry
// i think ServiceRegistry should have fn forward(&self, &str) -> Service;
impl Gateway {
    pub fn new(
        hostaddr: String,
        service_registry: ServiceRegistry,
        interface_addr: &str,
        inc_address: &str,
    ) -> Result<Self> {
        Ok(Self {
            interface_addr: interface_addr.to_string(),
            inc_address: inc_address.to_string(),
            listener: None,
            service_registry: Arc::new(Mutex::new(service_registry)),
            router: Arc::new(Mutex::new(Router::new())),
        })
    }

    //1 read lock per launch
    pub fn launch(self: Arc<Self>) -> Result<()> {
        let listener = TcpListener::bind(&self.interface_addr)?;
        for stream in listener.incoming() {
            self.clone().handle_client(stream.unwrap())?;
        }
        Ok(())
    }

    // 1 read lock
    pub fn handle_client(self: Arc<Self>, mut stream: TcpStream) -> Result<()> {
        //println!("Client Connected");
        let mut buffer = [0; 1000];
        let size = stream.read(&mut buffer)?;
        let buffer_utf8 = String::from_utf8_lossy(&buffer[..size]).to_string();

        //only the request line is being parsed at the moment
        let (line, _rest) = buffer_utf8
            .split_once("\r\n")
            .ok_or(error::Error::InvalidHeader)?;

        let rl = ReqLine::parse_req_line(line)?;
        let uri = match rl.uri.split_once('?') {
            Some((f, _)) => f,
            None => &rl.uri,
        };

        println!("recieved  {}", uri);
        let gw_read = &self.router.lock().unwrap();

        println!("lock aquired");
        if let Ok(ser) = gw_read.get_route(&uri) {
            println!("forwarding {:#?}", ser);
            let rec = Service::forward(ser, &buffer_utf8);
            let stream_send = stream.write(rec.as_bytes())?;
            println!("{stream_send} Bytes sent to the client");
        } else {
            println!("failed");
            stream.write(HttpBuilder::build_badrequest().as_bytes())?;
        }
        //        println!("{:#?}", self.service_registry.services);

        Ok(())
    }

    // Listening for Registration requests and heartbeats
    pub fn broadcast(self: Arc<Self>) -> Result<()> {
        let _d = thread::spawn(move || {
            println!(
                "Launching ServiceRegistry broadcast bind at:{}",
                &self.inc_address
            );
            let listener = TcpListener::bind(&self.inc_address).unwrap();

            //Router
            //ServiceRegistry
            'reference: loop {
                for stream in listener.incoming() {
                    let mut buffer = [0u8; 1000];
                    let mut ns = match stream {
                        Ok(e) => e,
                        Err(_) => break 'reference,
                    };
                    let read = match ns.read(&mut buffer) {
                        Ok(e) => e,
                        Err(_) => break 'reference,
                    };
                    let buffer = String::from_utf8_lossy(&buffer[..read]);

                    if buffer.starts_with("REGISTER") {
                        let buffer = buffer.split_once(' ').unwrap().1;

                        let service: Arc<RwLock<Service>> =
                            Arc::new(RwLock::new(serde_json::from_str(&buffer).unwrap()));

                        //                        let resp = format!(
                        //                            "Service registered successfully at host:{} for the routes: {:#}",
                        //                            service.inc_address,
                        //                            service.supported_routes.join(" ")
                        //                        );

                        match ns.write("[Gateway] : Registered successfully".as_bytes()) {
                            Ok(_size) => {
                                //add to ServiceRegistry
                                let mut router = self.service_registry.lock().unwrap();
                                let ser = router.register(service.clone());
                                let _ = ser
                                    .read()
                                    .unwrap()
                                    .supported_routes
                                    .iter()
                                    .map(|x| {
                                        self.router
                                            .lock()
                                            .unwrap()
                                            .add_route(x.clone(), service.clone())
                                    })
                                    .collect::<Vec<_>>();
                            }
                            Err(_) => break 'reference,
                            //..println!("{:#?}", service);
                        };
                    } else if buffer.starts_with("HEARTBEAT") {
                        let _buffer = buffer.split_once(' ').unwrap().1;
                        let mut sr_lock = self.service_registry.lock().unwrap();
                        println!("found service: {:#?}", _buffer);
                        let service = sr_lock.find_service(_buffer).unwrap();
                        // update status
                        //service.update_time();

                        println!("found service: {:#?}", service);
                    }
                }
            }
        });
        return Ok(());
    }
}
