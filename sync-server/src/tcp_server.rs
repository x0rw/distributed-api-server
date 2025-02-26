use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use base::{
    error::Result,
    http::{builder::HttpBuilder, handler::handle_http},
    routes::{RouteType, RoutesMap},
};

pub struct TcpServer {
    hostaddr: String,
    listener: TcpListener,
    routes: RoutesMap,
}
impl TcpServer {
    pub fn new(hostaddr: String, routes: RoutesMap) -> Self {
        Self {
            hostaddr: "moved".to_string(),
            listener: TcpListener::bind(hostaddr).unwrap(),
            routes,
        }
    }
    pub fn launch(&self) -> Result<()> {
        for stream in self.listener.incoming() {
            self.handle_client(stream.unwrap())?;
        }
        Ok(())
    }

    // all read and write sys calls should be done here
    fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
        //println!("Client Connected");
        let mut buffer = [0; 1000];
        stream.read(&mut buffer)?;
        let buffer_utf8 = String::from_utf8_lossy(&buffer[..]).to_string();

        // println!("{}", buffer_utf8.to_string());

        let handler = match handle_http(&buffer_utf8) {
            Ok(e) => e,
            Err(e) => {
                stream.write(HttpBuilder::build_badrequest().as_bytes())?;
                println!("client handler error {}", e);
                return Ok(()); // errors in handle_http arent that serious
            }
        };
        let uri = &handler.req_line.uri;
        let route = self.routes.get(uri);
        let http_response = HttpBuilder::new(handler, route).build();

        let stream_send = stream.write(http_response.as_bytes())?;
        println!("{stream_send} Bytes sent to the client");
        Ok(())
    }
}
