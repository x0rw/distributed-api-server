use crate::{error::Result, Error};
use std::{collections::HashMap, fs::File, io::Read};
pub struct RoutesMap {
    hm: HashMap<String, String>,
    error_route: String,
}
impl RoutesMap {
    pub fn new() -> Self {
        Self {
            hm: HashMap::new(),
            error_route: "res/404.html".to_string(),
        }
    }
    pub fn load(&mut self, route: &str, file: &str) -> Result<()> {
        println!("Loading {}", file);
        let mut fi = File::open(file)?;
        let mut contents = String::new();
        fi.read_to_string(&mut contents)?;
        self.hm.insert(route.to_string(), contents);
        Ok(())
    }
    pub fn error_page(&mut self, route_name: &str, file: &str) -> Result<()> {
        self.load(route_name, file)?;
        self.error_route = route_name.to_string();
        Ok(())
    }
    pub fn get(&self, k: &str) -> Route {
        if let Some(e) = self.hm.get(k) {
            return Route::RouteFound(e);
        } else {
            let s = self.error_route.as_str();
            let t = self.hm.get(s).unwrap();
            return Route::RouteNotFound(t);
        }
    }
}
#[derive(Debug)]
pub enum Route<'a> {
    RouteFound(&'a str),
    RouteNotFound(&'a str),
}
