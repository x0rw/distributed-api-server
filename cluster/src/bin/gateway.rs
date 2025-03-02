use std::io::{self, Write};
use std::iter::Inspect;
use std::sync::{Arc, Mutex, RwLock};
use std::vec;
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
        let service = Service::init("node_sevice_1", "127.0.0.1:8888", vec![]);
        ServiceRegistry::broadcast(nsr, &service);
    });
    let pub_routes = RoutesMap::new();
    let gateway = Gateway::new("127.0.0.1:1111".to_string(), pub_routes, sr);
    let arc_gateway = Arc::new(gateway);
    let gateway_clone = Arc::clone(&arc_gateway);
    thread::spawn(move || loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.starts_with("exit") {
            println!("Exiting..");
        } else if input.starts_with("show services") {
            println!("Services");
            println!(" Service Name  |  Network  | status");
            println!(
                "{}",
                gateway_clone
                    .service_registry
                    .lock()
                    .unwrap()
                    .services
                    .iter()
                    .map(|x| format!(
                        "{}   {}  {:#?}",
                        x.service_name.clone(),
                        x.inc_address.clone(),
                        x.health.status.clone(),
                    ))
                    .collect::<Vec<String>>()
                    .join("\r")
            );
        } else if input.starts_with("show routes") {
            println!("routes");
            println!(
                "{:#?}",
                gateway_clone
                    .service_registry
                    .lock()
                    .unwrap()
                    .routes
                    .iter()
                    .map(|(k, v)| format!("{} : {}", k, v.service_name))
                    .collect::<Vec<String>>()
            );
        }
    });
    let arc_c2 = Arc::clone(&arc_gateway);
    Gateway::launch(arc_c2).unwrap();

    Ok(())
}

//fn main() {
//  let clad = "127.0.0.1:1212".to_string();
//  let service = Service::init("test_service", &clad);
//  let sr = ServiceRegistry::new().broadcast(&service); //bind to add 1212 PROTO
//}
