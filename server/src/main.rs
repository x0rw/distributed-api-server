use std::io::Error;
mod error;
mod routes;
use error::Result;
use tcp_server::TcpServer;
mod controller;
mod http;
use crate::http::{builder, handler, header};
mod tcp_server;
mod utils;
fn main() -> Result<()> {
    let mut pub_routes = routes::RoutesMap::new();
    pub_routes
        .load("/", "res/index.html", handler::HttpMethod::POST)
        .load("/article", "res/article.html", handler::HttpMethod::GET)
        .error_page("404", "res/404.html", handler::HttpMethod::GET)
        .add_controller(
            "/echo",
            controller::Controller::EchoController,
            handler::HttpMethod::GET,
        );
    TcpServer::new("127.0.0.1:1111".to_string(), pub_routes).launch()?;
    Ok(())
}
