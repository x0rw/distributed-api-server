use crate::service::Service;
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

pub struct Router {
    pub map: HashMap<String, Arc<Service>>,
}
impl Router {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn add_route(&mut self, route: String, service: Arc<Service>) {
        self.map.insert(route, service);
    }
    pub fn get_route(&self, uri: &str) -> Option<Arc<Service>> {
        let gr = self.map.get(uri).unwrap();
        println!("Requesting route:{}", uri);

        //println!("avaliable routes:{:#?}", self.routes);
        return Some(Arc::clone(&gr));
    }
}

#[derive(Debug)]
pub struct ServiceRegistry {
    pub services: Vec<Arc<Service>>,
}

impl ServiceRegistry {
    pub fn as_ref(&self) -> Vec<&Service> {
        self.services
            .iter()
            .map(|x| &**x) //alternatives,.as_ref()
            //or map(Arc::as_ref)
            .collect::<Vec<&Service>>()
    }
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }
    pub fn register(&mut self, service: Arc<Service>) -> &Service {
        println!("Registering a new service {:#?}", service.clone());
        self.services.push(service.clone());
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
