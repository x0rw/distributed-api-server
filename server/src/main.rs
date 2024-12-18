use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
mod http_handler;
use http_handler::*;
fn handle_client(mut stream: TcpStream){
    println!("Client Connected");
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    let handler = handle_http(String::from_utf8_lossy(&buffer[..]).split("\n\r").next().unwrap());
    println!("Req:{:?}",handler);
    
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:1111").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_client(stream);   
    }
}
