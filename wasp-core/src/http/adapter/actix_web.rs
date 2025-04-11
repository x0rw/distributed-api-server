use std::net::SocketAddr;

use super::{ServerAdapter, WaspHandler};
use crate::http::response::Response;
use crate::http::HttpMethod;
use crate::{http::request::Request, response};
use actix_web::{self, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

pub struct ActixAdapter();
impl ServerAdapter for ActixAdapter {
    type RequestType = actix_web::HttpRequest;
    type ResponseType = actix_web::HttpResponse;

    async fn convert_request(req: Self::RequestType) -> Request {
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

    async fn run(routes: Vec<(&'static str, HttpMethod, WaspHandler)>, address: SocketAddr) {
        let server = HttpServer::new(move || {
            let mut app = App::new();

            for (path, method, handler) in &routes {
                let handler = *handler;
                app = match *method {
                    HttpMethod::POST => app.route(
                        *path,
                        web::post().to(move |req| ActixAdapter::handler_wrapper(req, handler)),
                    ),
                    HttpMethod::GET => app.route(
                        *path,
                        web::get().to(move |req| ActixAdapter::handler_wrapper(req, handler)),
                    ),
                    _ => panic!("un"),
                }
            }
            app
        });
        println!("Server running at {}, powered by Actix-web", address);
        server.bind(address).unwrap().run().await.unwrap();
    }
}
