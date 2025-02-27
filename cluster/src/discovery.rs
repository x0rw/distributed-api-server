use std::{thread, time::Duration};

// service registry and discovery
// nodes register themselves into it
// after conecting to /register the node will be added to the service registery
// so that the worker thread can ping it periodically for health
use crate::health::Health;
struct Service {
    service_name: String,
    address: String,
    health: Health,
}

struct ServiceRegistry<'a> {
    services: Vec<&'a ServiceRegistry>,
}
impl<'a> ServiceRegistry<'a> {
    fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }
    fn add_service(&mut self, service: &'a Service) {
        self.services.push(service);
    }
    fn worker() {
        loop {
            thread::sleep(Duration::from_millis(5000));
            for service in ServiceRegistry {
                service.ping();
            }
        }
    }
}

impl Service {
    fn register(service_name: &str, address: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            address: address.to_string(),
            health: Health::default(),
        }
    }
    fn ping(&self) {
        //ping to {host}/ping

        todo!()
    }
}
