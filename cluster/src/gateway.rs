use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
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
    pub listener: TcpListener,
    pub service_registry: ServiceRegistry,
    pub router: Router,
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
            listener: TcpListener::bind(hostaddr)?,
            service_registry,
            router: Router::new(),
        })
    }

    //1 read lock per launch
    pub fn launch(gw: Arc<RwLock<Self>>) -> Result<()> {
        for stream in gw.read().unwrap().listener.incoming() {
            Self::handle_client(Arc::clone(&gw), stream.unwrap())?;
        }
        Ok(())
    }

    // 1 read lock
    pub fn handle_client(gw: Arc<RwLock<Self>>, mut stream: TcpStream) -> Result<()> {
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
        let router = &gw.read().unwrap().router;
        if let Some(ser) = router.get_route(&uri) {
            let rec = ser.forward(&buffer_utf8);
            println!("forwarding {}", rec.clone());
            let stream_send = stream.write(rec.as_bytes())?;
            println!("{stream_send} Bytes sent to the client");
        } else {
            stream.write(HttpBuilder::build_badrequest().as_bytes())?;
        }
        //        println!("{:#?}", self.service_registry.services);

        Ok(())
    }

    pub fn broadcast(gw: Arc<RwLock<Self>>) -> Result<()> {
        let _d = thread::spawn(move || {
            println!(
                "Launching ServiceRegistry broadcast bind at:{}",
                &gw.read().unwrap().inc_address
            );
            let listener = TcpListener::bind(&gw.read().unwrap().inc_address).unwrap();

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
                        let service: Arc<Service> =
                            Arc::new(serde_json::from_str(&buffer).unwrap());

                        let resp = format!(
                            "Service registered successfully at host:{} for the routes: {:#}",
                            service.inc_address,
                            service.supported_routes.join(" ")
                        );

                        match ns.write(resp.as_bytes()) {
                            Ok(_size) => {
                                //add to ServiceRegistry
                                let mut gwo = gw.write().unwrap();
                                let ser = gwo.service_registry.register(service.clone());
                                let _ = ser
                                    .supported_routes
                                    .iter()
                                    .map(|x| {
                                        gw.write()
                                            .unwrap()
                                            .router
                                            .add_route(x.clone(), service.clone())
                                    })
                                    .collect::<Vec<_>>();
                            }
                            Err(_) => break 'reference,
                            //..println!("{:#?}", service);
                        };
                    } else if buffer.starts_with("HEARTBEAT") {
                        let _buffer = buffer.split_once(' ').unwrap().1;
                        println!("Recieved signal at the broadcast: {}", buffer);
                    }
                }
            }
        });
        return Ok(());
    }
}
