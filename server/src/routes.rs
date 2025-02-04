use crate::{error::Result, Error};
use std::{collections::HashMap, fs::File, io::Read};
#[derive(Debug)]
pub enum RouteType {
    Data(String),
    Redirect(String, bool),
    NotFound,
}
pub struct RoutesMap {
    hm: HashMap<String, RouteType>,
    error_route: String,
}
impl RoutesMap {
    pub fn getErrorRoute(&self) -> &str {
        &self.error_route
    }
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
        self.hm.insert(route.to_string(), RouteType::Data(contents));
        Ok(())
    }
    pub fn error_page(&mut self, route_name: &str, file: &str) -> Result<()> {
        self.load(route_name, file)?;
        self.error_route = route_name.to_string();
        Ok(())
    }
    pub fn get(&self, k: &str) -> &RouteType {
        if let Some(e) = self.hm.get(k) {
            return e;
        } else {
            return &RouteType::NotFound;
        }
    }
}
