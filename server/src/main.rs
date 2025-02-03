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
use routes::{Route, RoutesMap};

fn handle_client(mut stream: TcpStream, rm: &RoutesMap) -> Result<()> {
    println!("Client Connected");
    let mut buffer = [0; 1000];
    stream.read(&mut buffer)?;
    let buffer_utf8 = String::from_utf8_lossy(&buffer[..]);

    // println!("{}", buffer_utf8.to_string());
    let handler = handle_http(buffer_utf8.to_string())?;
    let uri = handler.uri.as_ref();

    //check if the requested route exist
    let mut build_resp = match rm.get(uri) {
        Route::RouteFound(e) => HtmlBuilder::response(HttpResponseCode::Ok200, e),
        Route::RouteNotFound(e) => {
            HtmlBuilder::response(HttpResponseCode::MovedPerm301("/".to_string()), e)
        }
    };

    if let Some(e) = &handler.data {
        build_resp.push_str(e);
    }
    match handler.method {
        HttpMethod::GET => {}
        HttpMethod::POST => {
            println!("Recieved data: {}", handler.get_data());
        }
    }
    // println!("{}", build_resp);
    let stream_send = stream.write(build_resp.as_bytes())?;
    println!("{stream_send} Bytes sent to the client");
    Ok(())
}
fn main() -> Result<()> {
    let mut pub_routes = crate::routes::RoutesMap::new();
    let routes_ref = &mut pub_routes;
    routes_ref.load("/", "res/index.html")?;
    routes_ref.load("/article", "res/article.html")?;
    routes_ref.error_page("404", "res/404.html")?;
    let listener = TcpListener::bind("127.0.0.1:1111").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //        thread::sleep(Duration::from_millis(4000));

        handle_client(stream, &routes_ref).unwrap();
    }
    Ok(())
}
