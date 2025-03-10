use crate::gateway::Gateway;
use std::sync::{Arc, RwLock};
use std::thread;

use std::io::{self, Write};
pub fn cli_gateway(gateway: Arc<Gateway>) {
    thread::spawn(move || loop {
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
                    .map(|x| {
                        let x = x.read().unwrap();
                        format!(
                            "{}   {}  {:#?}",
                            x.service_name.clone(),
                            x.inc_address.clone(),
                            x.health.status.clone(),
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\r")
            );
        } else if input.starts_with("show routes") {
            println!("routes");
            println!(
                "{:#?}",
                gateway
                    .router
                    .lock()
                    .unwrap()
                    .map
                    .iter()
                    .map(|(k, v)| format!("{} : {:#?}", k, v.read().unwrap()))
                    .collect::<Vec<String>>()
            );
        }
    });
}
