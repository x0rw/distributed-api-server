use std::marker::PhantomData;

use base::http::builder;
use cluster::service::Service;
fn main() {
    let clad = "127.0.0.1:1212".to_string();
    let supported_routes = vec!["/post".to_string(), "/hello".to_string()];
    let service = Service::init("test_service", "127.0.0.1:3333", supported_routes);
    let _discover = service.discover(clad);
}
