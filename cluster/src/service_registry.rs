use base::error::{Error, Result};

use crate::service::Service;
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

pub struct Router {
    pub map: HashMap<String, Arc<RwLock<Service>>>,
}
impl Router {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn add_route(&mut self, route: String, service: Arc<RwLock<Service>>) {
        self.map.insert(route, service);
    }
    pub fn get_route(&self, uri: &str) -> Result<&Arc<RwLock<Service>>> {
        match self.map.get(uri) {
            Some(e) => Ok(e),
            None => Err(Error::EmptyHeaderField),
        }

        //println!("avaliable routes:{:#?}", self.routes);
    }
}
use std::sync::RwLock;

#[derive(Debug)]
pub struct ServiceRegistry {
    pub services: Vec<Arc<RwLock<Service>>>,
}

impl ServiceRegistry {
    pub fn find_service(&mut self, service_id: &str) -> Option<&Arc<RwLock<Service>>> {
        self.services
            .iter()
            .find(|&e| e.read().unwrap().service_name == service_id)
    }
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register(&mut self, service: Arc<RwLock<Service>>) -> &Arc<RwLock<Service>> {
        println!("Registering a new service {:#?}", service.clone());
        self.services.push(service);
        return self.services.last().unwrap();

        //        let s_routes = service.clone().supported_routes;
        //        s_routes
        //            .into_iter()
        //            .map(|x| self.routes.insert(x, service.clone()))
        //            .for_each(|_| ());
        //
        //        self.as_vec();
        //        println!("{:#?}", self.routes.keys());
    }
    fn _worker(self) {
        loop {
            thread::sleep(Duration::from_millis(5000));
            //    for service in self.as_vec().iter() {
            //        service.ping();
            //    }
        }
    }
    pub fn health_checker(&self) {
        loop {}
    }
    //periodically pings services
    //if the broadcast doesnt recieve any heartbeat in 5 seconds its removed from the registery
}
