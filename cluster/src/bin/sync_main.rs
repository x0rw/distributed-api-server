use std::mem;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::{io::Error, ops::DerefMut};

use base::auth;
use base::error::Result;
use base::http::handler;
use base::routes::RoutesMap;
use cluster::node::Node;
use cluster::service::{Service, ServiceRegistry};
use cluster::sync_node::SyncNode;

fn main() -> Result<()> {
    //    let service = Service::init("node_sevice_1", "127.0.0.1:1212").broadcast();
    let mut pub_routes = RoutesMap::new()
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
    let sr = Arc::new(Mutex::new(ServiceRegistry::new()));
    SyncNode::new("127.0.0.1:1111".to_string(), pub_routes, sr).launch();
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
