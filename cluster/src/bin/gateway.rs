use std::sync::{Arc, Mutex};
use std::thread;
use std::vec;

use cluster::cli::cli_gateway;
use cluster::gateway;
use cluster::{
    gateway::Gateway,
    service::{Service, ServiceRegistry},
};

use base::error::Result;
use base::routes::RoutesMap;

fn main() -> Result<()> {
    let sr = Arc::new(Mutex::new(ServiceRegistry::new()));
    let nsr = Arc::clone(&sr);
    thread::spawn(move || {
        let service = Service::init("gateway_node", "127.0.0.1:8888", "127.0.0.1:8888", vec![]);
        ServiceRegistry::broadcast(nsr, &service);
    });
    let pub_routes = RoutesMap::new();
    let gateway = Arc::new(Gateway::new("127.0.0.1:1111".to_string(), pub_routes, sr));
    let gateway_clone = Arc::clone(&gateway);
    thread::spawn(move || cli_gateway(gateway_clone.clone()));
    gateway.clone().launch().unwrap();
    Ok(())
}
