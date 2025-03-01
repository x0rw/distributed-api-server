use std::sync::{Arc, Mutex, RwLock};
use std::{
    mem::{transmute, transmute_copy},
    thread,
};

use cluster::{
    gateway::{self, Gateway},
    service::{Service, ServiceRegistry},
};

use base::error::Result;
use base::http::handler;
use base::routes::RoutesMap;
use cluster::node::Node;

fn main() -> Result<()> {
    let sr = Arc::new(Mutex::new(ServiceRegistry::new()));
    let nsr = Arc::clone(&sr);
    thread::spawn(move || {
        let service = Service::init("node_sevice_1", "127.0.0.1:1212", vec![]);
        ServiceRegistry::broadcast(nsr, &service);
    });
    let pub_routes = RoutesMap::new()
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

    Gateway::new("127.0.0.1:1111".to_string(), pub_routes, sr)
        .launch()
        .unwrap();
    Ok(())
}

//fn main() {
//  let clad = "127.0.0.1:1212".to_string();
//  let service = Service::init("test_service", &clad);
//  let sr = ServiceRegistry::new().broadcast(&service); //bind to add 1212 PROTO
//}
