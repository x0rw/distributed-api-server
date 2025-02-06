use std::env::var;
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
mod controller;

fn handle_client(mut stream: TcpStream, router: &RoutesMap) -> Result<()> {
    println!("Client Connected");
    let mut buffer = [0; 1000];
    stream.read(&mut buffer)?;
    let buffer_utf8 = String::from_utf8_lossy(&buffer[..]);

    // println!("{}", buffer_utf8.to_string());
    let handler = match handle_http(buffer_utf8.to_string()) {
        Ok(e) => e,
        Err(_) => {
            stream.write(HttpBuilder::build_badrequest().as_bytes())?;
            return Ok(()); // errors in handle_http arent that serious
        }
    };
    let uri = handler.uri.as_ref();

    //check if the requested route exist
    let route = router.get(uri);
    let builder = HttpBuilder::build(route, handler, router);

    println!("{}", builder.data);
    let stream_send = stream.write(builder.data.as_bytes())?;
    println!("{stream_send} Bytes sent to the client");
    Ok(())
}
fn main() -> Result<()> {
    let mut pub_routes = crate::routes::RoutesMap::new();
    let routes_ref = &mut pub_routes;
    routes_ref.load("/", "res/index.html")?;
    routes_ref.load("/article", "res/article.html")?;
    routes_ref.error_page("404", "res/404.html")?;
    routes_ref.add_controller("/art", controller::Controller::ArticleController)?;
    let listener = TcpListener::bind("127.0.0.1:1111").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //        thread::sleep(Duration::from_millis(4000));

        handle_client(stream, &routes_ref).expect("fdfdf");
    }
    Ok(())
}
