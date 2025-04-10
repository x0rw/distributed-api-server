pub mod adapter;
pub mod request;
pub mod response;
pub use actix_web;
use request::Request;
use response::Response;
pub use std::net::SocketAddr;

pub type WaspHandler = fn(Request) -> Response;

trait ServerAdapter {
    type RequestType;
    type ResponseType;
    async fn convert_request(req: Self::RequestType) -> Request;
    async fn handler_wrapper(req: Self::RequestType, handler: WaspHandler) -> Self::ResponseType;
    async fn run(routes: Vec<(&'static str, &'static str, WaspHandler)>, address: SocketAddr);
}

#[derive(Debug)]
enum WaspRunner {
    ActixWeb,
    Custom,
}

pub struct WaspServer {
    address: SocketAddr,
    routes: Vec<(&'static str, &'static str, WaspHandler)>,
}

impl WaspServer {
    pub fn new(address: &str) -> Result<Self, std::net::AddrParseError> {
        let addr: SocketAddr = address.parse()?;
        Ok(WaspServer {
            address: addr,
            routes: Vec::new(),
        })
    }
    pub fn add_route(&mut self, path: &'static str, method: &'static str, handler: WaspHandler) {
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
