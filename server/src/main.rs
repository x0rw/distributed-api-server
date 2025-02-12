use std::io::Error;
mod error;
mod http_handler;
mod routes;
use error::Result;
use tcp_server::TcpServer;

mod controller;
mod http_builder;
mod tcp_server;
mod utils;
fn main() -> Result<()> {
    let mut pub_routes = routes::RoutesMap::new();
    pub_routes
        .load("/", "res/index.html")
        .load("/article", "res/article.html")
        .error_page("404", "res/404.html")
        .add_controller("/echo", controller::Controller::ArticleController);
    TcpServer::new("127.0.0.1:1111".to_string(), pub_routes).launch()?;
    Ok(())
}
