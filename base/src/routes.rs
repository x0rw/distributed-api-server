use std::{collections::HashMap, fs::File, io::Read};

use crate::controller::Controller;
use crate::http::{
    builder::Response,
    handler::{self, HttpMethod},
};

#[derive(Debug, PartialEq)]
pub enum RouteType {
    //    Data(String),
    //    Redirect(String, bool),
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
            "/ping".to_string(),
            (
                HttpMethod::POST,
                RouteType::Controller(Controller::PingController),
            ),
        );
        Self { hm: hm }
    }
    //    pub fn load(mut self, route: &str, file: &str, method: HttpMethod) -> Self {
    //        println!("Loading {}", file);
    //        let mut fi = File::open(file).expect("route doesn't exist");
    //        let mut contents = String::new();
    //        fi.read_to_string(&mut contents).expect("failed to read");
    //        self.hm
    //            .insert(route.to_string(), (method, RouteType::Data(contents)));
    //        self
    //    }

    pub fn get(&self, k: &str) -> &RouteType {
        println!("Client Requesting: {}", k);
        match self.hm.get(k) {
            Some((method, rt)) => return rt,
            None => &RouteType::NotFound,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_route_type_not_found() {
        let rt = RoutesMap::new();
        let notd = rt.get("/post");
        assert_eq!(notd, &RouteType::NotFound);
    }
    #[test]
    fn test_route_type_ping() {
        let rt = RoutesMap::new();
        let notd = rt.get("/ping");
        assert_eq!(notd, &RouteType::Controller(Controller::PingController));
    }
    #[test]
    fn test_echo_controller() {
        let mut rt =
            RoutesMap::new().add_controller("/echo", Controller::EchoController, HttpMethod::GET);
        let notd = rt.get("/echo");
        assert_eq!(notd, &RouteType::Controller(Controller::EchoController));
    }
}
