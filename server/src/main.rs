use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::ptr::write;
use std::{thread, time::Duration};
mod http_handler;
mod routes;
use http_handler::*;
use routes::RoutesMap;
fn handle_client(mut stream: TcpStream, rm: &RoutesMap) {
    println!("Client Connected");
    let mut buffer = [0; 1000];
    stream.read(&mut buffer).unwrap();
    let buffer_utf8 = String::from_utf8_lossy(&buffer[..]);

    let handler = handle_http(buffer_utf8.to_string());
    let mut response = String::from("HTTP/1.1 200 OK\r\n\r\n");
    match handler.method {
        HTTP_METHOD::GET => {
            let uri = handler.uri;
            if let Some(e) = rm.get(&uri) {
                response.push_str(e);
            }
        }
        HTTP_METHOD::POST => println!("GETGET"),
    }
    stream.write(response.as_bytes());
}
fn main() {
    let mut pub_routes = crate::routes::RoutesMap::new();
    let routes_ref = &mut pub_routes;
    routes_ref.load("/".to_string(), "res/index.html".to_string());

    let listener = TcpListener::bind("127.0.0.1:1111").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //        thread::sleep(Duration::from_millis(4000));

        handle_client(stream, &routes_ref);
    }
}
