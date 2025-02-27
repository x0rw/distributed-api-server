use std::{collections::HashMap, fs::File, io::Read};

use crate::controller::Controller;
use crate::http::{
    builder::Response,
    handler::{self, HttpMethod},
};

#[derive(Debug)]
pub enum RouteType {
    Data(String),
    Redirect(String, bool),
    NotFound,
    Controller(fn(handler::Data) -> Response),
}

pub struct RoutesMap {
    hm: HashMap<String, (HttpMethod, RouteType)>,
}
impl RoutesMap {
    pub fn add_controller(
        mut self,
        route: &str,
        controller: fn(handler::Data) -> Response,
        method: HttpMethod,
    ) -> Self {
        self.hm.insert(
            route.to_string(),
            (method, RouteType::Controller(controller)),
        );
        self
    }
    pub fn new() -> Self {
        let mut hm = HashMap::new();
        hm.insert(
            "ping".to_string(),
            (
                HttpMethod::POST,
                RouteType::Controller(Controller::PingController),
            ),
        );
        Self { hm: hm }
    }
    pub fn load(mut self, route: &str, file: &str, method: HttpMethod) -> Self {
        println!("Loading {}", file);
        let mut fi = File::open(file).expect("route doesn't exist");
        let mut contents = String::new();
        fi.read_to_string(&mut contents).expect("failed to read");
        self.hm
            .insert(route.to_string(), (method, RouteType::Data(contents)));
        self
    }
    pub fn error_page(mut self, route_name: &str, file: &str, method: HttpMethod) -> Self {
        //       self.load(route_name, file, method);
        self
    }
    pub fn get(&self, k: &str) -> &RouteType {
        //println!("Client Requesting: {}", k);

        if let Some(e) = self.hm.get(k) {
            //println!("Found Route: {:?} of type {:?}", e.1, e.0);
            return &e.1;
        } else {
            return &RouteType::NotFound;
        }
    }
}
