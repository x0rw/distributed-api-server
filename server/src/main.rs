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
use tcp_server::TcpServer;
mod controller;
mod tcp_server;
mod utils;
fn main() -> Result<()> {
    let mut pub_routes = crate::routes::RoutesMap::new();
    pub_routes
        .load("/", "res/index.html")
        .load("/article", "res/article.html")
        .error_page("404", "res/404.html")
        .add_controller("/art", controller::Controller::ArticleController);
    let server = TcpServer::new("127.0.0.1:1111".to_string(), pub_routes).launch();
    Ok(())
}
