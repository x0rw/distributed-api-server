use std::net::SocketAddr;

use crate::http::response::Response;
use crate::http::{ServerAdapter, WaspHandler};
use crate::{http::request::Request, response};
use actix_web::{self, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
pub struct ActixAdapter();
impl ServerAdapter for ActixAdapter {
    type RequestType = actix_web::HttpRequest;
    type ResponseType = actix_web::HttpResponse;

    async fn convert_request(req: actix_web::HttpRequest) -> Request {
        let path = req.path().to_string();
        let method = req.method().to_string();
        let mut abs_req = Request::new(path, method);

        for (key, value) in req.match_info().iter() {
            abs_req.params.insert(key.to_string(), value.to_string());
        }

        abs_req
    }
    // Wraps around the Wasp Handler
    async fn handler_wrapper(req: Self::RequestType, handler: WaspHandler) -> Self::ResponseType {
        // Convert Actix request to Wasp Request
        let converted_request = ActixAdapter::convert_request(req).await;
        let wasp_response = handler(converted_request);

        // Building an Actix-web familiar request
        return HttpResponse::build(
            actix_web::http::StatusCode::from_u16(wasp_response.status).unwrap(),
        )
        .body(wasp_response.body);
    }
    async fn run(routes: Vec<(&'static str, &'static str, WaspHandler)>, address: SocketAddr) {
        unimplemented!()
    }
}
