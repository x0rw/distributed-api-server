use std::sync::{Arc, RwLock};

use base::error::Result;
use cluster::cli::cli_gateway;
use cluster::gateway::Gateway;
use cluster::service_registry::ServiceRegistry;

fn main() -> Result<()> {
    let sr = ServiceRegistry::new();

    match Gateway::new(
        "127.0.0.1:1111".to_string(),
        sr,
        "127.0.0.1:1111",
        "127.0.0.1:8888",
    ) {
        Ok(e) => {
            let gateway = Arc::new(e);
            //Gateway::broadcast(Arc::clone(&gateway)).unwrap();
            //cli_gateway(Arc::clone(&gateway));
            Gateway::launch(Arc::clone(&gateway))?;
        }
        Err(err) => {
            println!("error gateway.rs:{}", err);
        }
    }

    Ok(())
}
