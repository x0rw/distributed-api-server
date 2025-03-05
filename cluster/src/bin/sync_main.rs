use std::env::args;
use std::{env, thread};

use base::error::Result;
use base::http::handler;
use base::routes::RoutesMap;
use cluster::node::Node;
use cluster::service::Service;
use cluster::sync_node::SyncNode;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 6 {
        println!(
            "Usage: {} node-name node_port node_inc_port gateway_port",
            args[0]
        );
        return Ok(());
    }

    let node_name = &args[1];
    let node_addr = format!("127.0.0.1:{}", &args[2]);
    let node_inc = format!("127.0.0.1:{}", &args[3]);
    let gateway_addr = format!("127.0.0.1:{}", &args[4]);
    let endpoint = format!("{}", &args[5]);

    println!("endpoint:{}", endpoint);
    //spawn a thread to connect to register the service in the api gateway
    let supported_routes = vec![endpoint.clone()];
    let service = Service::init(node_name, &node_inc, &node_addr, supported_routes);
    thread::spawn(move || {
        service.discover_gateway(gateway_addr.to_string()).unwrap();
    });
    let pub_routes = RoutesMap::new().add_controller(
        endpoint.as_ref(),
        base::controller::Controller::EchoController,
        handler::HttpMethod::GET,
    );
    //    let sr = Arc::new(Mutex::new(ServiceRegistry::new()));
    SyncNode::new(node_addr.to_string(), pub_routes).launch()?;
    Ok(())
}
