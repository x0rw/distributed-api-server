use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    http_builder::HttpBuilder,
    http_handler::{self, handle_http},
    routes::{self, RoutesMap},
    Result,
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
            //        thread::sleep(Duration::from_millis(4000));

            self.handle_client(stream.unwrap())?;
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
        println!("Client Connected");
        let mut buffer = [0; 1000];
        stream.read(&mut buffer)?;
        let buffer_utf8 = String::from_utf8_lossy(&buffer[..]).to_string();

        // println!("{}", buffer_utf8.to_string());

        let handler = match handle_http(&buffer_utf8) {
            Ok(e) => e,
            Err(_) => {
                stream.write(HttpBuilder::build_badrequest().as_bytes())?;
                return Ok(()); // errors in handle_http arent that serious
            }
        };
        let uri = handler.uri.as_ref();

        // println!("{uri}");
        let route = self.routes.get(uri);
        let built = HttpBuilder::new(handler, route).build();

        //  println!("{}", builder.data);
        let stream_send = stream.write(built.as_bytes())?;
        println!("{stream_send} Bytes sent to the client");
        Ok(())
    }
}
