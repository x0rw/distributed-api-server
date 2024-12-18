use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
#[derive(Debug)]
enum HTTP_METHOD{
    POST,
    GET,
}
#[derive(Debug)]
struct http_req{
    uri: String,
    method: HTTP_METHOD,
}
impl http_req{
    fn new(method: HTTP_METHOD, uri: &str)->Self{
        Self{
            uri: String::from(uri),
            method: method,
        }
    }
}
fn handle_http(line:&str)-> http_req{
   let mut words = line.split_whitespace();
   let nw = words.next().unwrap();
   if(nw.len()!= 3){
        panic!("unvalid http header size");
   }

   match nw{
        "GET" => http_req::new(HTTP_METHOD::GET, words.next().unwrap()),
        "POST" => http_req::new(HTTP_METHOD::POST, words.next().unwrap()),
        _ => panic!("unvalid http"),
   }
}
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
