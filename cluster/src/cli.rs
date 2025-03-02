use crate::gateway::Gateway;
use std::sync::Arc;

use std::io::{self, Write};
pub fn cli_gateway(gateway: Arc<Gateway>) {
    loop {
        io::stdout().flush().unwrap();
        print!("> ");
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
                gateway
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
                gateway
                    .service_registry
                    .lock()
                    .unwrap()
                    .routes
                    .iter()
                    .map(|(k, v)| format!("{} : {}", k, v.service_name))
                    .collect::<Vec<String>>()
            );
        }
    }
}
