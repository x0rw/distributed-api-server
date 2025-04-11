pub mod adapter;
pub mod client;
pub mod request;
pub mod response;
use adapter::ServerAdapter;
use std::{fmt, net::SocketAddr};

pub use actix_web;
use adapter::WaspHandler;
#[derive(Debug, Clone)]
pub enum HttpMethod {
    POST,
    GET,
}
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            _ => "idk",
        };
        write!(f, "{}", method_str)
    }
}
#[derive(Debug, Clone)]
pub enum WaspRunner {
    ActixWeb,
    Custom,
}

pub struct WaspServer {
    address: SocketAddr,
    routes: Vec<(&'static str, HttpMethod, WaspHandler)>,
}

impl WaspServer {
    pub fn new(address: &str) -> Result<Self, std::net::AddrParseError> {
        let addr: SocketAddr = address.parse()?;
        Ok(WaspServer {
            address: addr,
            routes: Vec::new(),
        })
    }
    pub fn add_route(&mut self, path: &'static str, method: HttpMethod, handler: WaspHandler) {
        self.routes.push((path, method, handler));
    }
    // TODO(x0rw): add a parameter that can specify the underlaying framework
    pub async fn run(self, runner: WaspRunner) -> std::io::Result<()> {
        let routes = self.routes.clone();
        let address = self.address;
        match &runner {
            WaspRunner::ActixWeb => adapter::actix_web::ActixAdapter::run(routes, address).await,
            _ => println!("{:#?} is not implemented yet", runner),
        }
        println!("Wasp server listening on {}", address);

        Ok(())
    }
}
