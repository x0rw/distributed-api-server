use std::sync::{Arc, Mutex};
use std::thread;

use base::error::Result;
use base::http::handler;
use base::routes::RoutesMap;
use cluster::node::Node;
use cluster::service::{Service, ServiceRegistry};
use cluster::sync_node::SyncNode;
pub struct SomeBuilder<'a> {
    pub arg1: &'a str,
}

impl<'a> SomeBuilder<'a> {
    pub fn new(arg1: &'a str) -> SomeBuilder<'a> {
        SomeBuilder { arg1 }
    }

    pub fn set_arg(mut self, arg1: &'a str) -> Self {
        self.arg1 = arg1;
        self
    }

    pub fn build(self) -> String {
        self.arg1.to_string()
    }
}

fn setter(mut builder: SomeBuilder) -> String {
    let str1 = String::from("asdad");
    builder.set_arg(str1.as_str()).build()
}
fn main() -> Result<()> {
    let gateway_inc_host = "127.0.0.1:8888";

    //spawn a thread to connect to register the service in the api gateway
    let supported_routes = vec!["/post".to_string(), "/hello".to_string()];
    let service = Service::init("node_sevice_1", "127.0.0.1:2222", supported_routes);
    thread::spawn(move || {
        service.discover(gateway_inc_host.to_string());
    });
    let mut pub_routes = RoutesMap::new().add_controller(
        "/echo",
        base::controller::Controller::EchoController,
        handler::HttpMethod::GET,
    );
    let sr = Arc::new(Mutex::new(ServiceRegistry::new()));
    SyncNode::new("127.0.0.1:2222".to_string(), pub_routes, sr).launch();
    Ok(())
}
