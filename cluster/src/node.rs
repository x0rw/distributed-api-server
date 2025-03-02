use base::{error::Result, routes::RoutesMap};
use std::sync::{Arc, Mutex};
use std::{net::TcpStream, sync::RwLock};

use crate::service::ServiceRegistry;

pub trait Node {
    fn new(
        hostaddr: String,
        routes: RoutesMap,
        //service_registry: Arc<Mutex<ServiceRegistry>>,
    ) -> Self;
    fn launch(self) -> Result<()>;
    fn handle_client(&self, stream: TcpStream) -> Result<()>;
}
