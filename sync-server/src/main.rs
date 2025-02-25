use std::mem;
use std::pin::Pin;
use std::{io::Error, ops::DerefMut};

use base::auth;
use base::error::Result;
use base::http::handler;
use base::routes::RoutesMap;
use tcp_server::TcpServer;

mod tcp_server;
fn main() -> Result<()> {
    let mut pub_routes = RoutesMap::new();
    pub_routes
        .load("/", "base/res/index.html", handler::HttpMethod::POST)
        .load(
            "/article",
            "base/res/article.html",
            handler::HttpMethod::GET,
        )
        //          .error_page("404", "/404.html", handler::HttpMethod::GET)
        .add_controller(
            "/echo",
            base::controller::Controller::EchoController,
            handler::HttpMethod::GET,
        );
    TcpServer::new("127.0.0.1:1111".to_string(), pub_routes).launch();
    Ok(())
}
/*
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
*/
