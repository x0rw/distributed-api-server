pub mod actix_web;

pub use std::net::SocketAddr;

use super::{request::Request, response::Response, HttpMethod};

pub type WaspHandler = fn(Request) -> Response;

pub trait ServerAdapter {
    type RequestType;
    type ResponseType;
    async fn convert_request(req: Self::RequestType) -> Request;
    async fn handler_wrapper(req: Self::RequestType, handler: WaspHandler) -> Self::ResponseType;
    async fn run(routes: Vec<(&'static str, HttpMethod, WaspHandler)>, address: SocketAddr);
}
