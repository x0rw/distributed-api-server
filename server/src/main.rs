use std::io::prelude::*;
use std::io::Error;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ptr::write;
use std::{thread, time::Duration};
mod error;
mod http_handler;
mod routes;
use error::Result;
use http_handler::*;
use routes::RoutesMap;
fn handle_client(mut stream: TcpStream, rm: &RoutesMap) {
    println!("Client Connected");
    let mut buffer = [0; 1000];
    stream.read(&mut buffer).unwrap();
    let buffer_utf8 = String::from_utf8_lossy(&buffer[..]);

    let handler = handle_http(buffer_utf8.to_string()).unwrap();
    let mut response = String::from("HTTP/1.1 200 OK\r\n\r\n");
    match handler.method {
        HTTP_METHOD::GET => {
            let uri = handler.uri;
            response.push_str(rm.get(&uri));
        }
        HTTP_METHOD::POST => println!("GETGET"),
    }
    stream.write(response.as_bytes());
}
fn main() -> Result<()> {
    let mut pub_routes = crate::routes::RoutesMap::new();
    let routes_ref = &mut pub_routes;
    routes_ref.load("/", "res/index.html")?;
    routes_ref.error_page("404", "res/404.html")?;
    let listener = TcpListener::bind("127.0.0.1:1111").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //        thread::sleep(Duration::from_millis(4000));

        handle_client(stream, &routes_ref);
    }
    Ok(())
}
