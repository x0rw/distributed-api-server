use std::net::SocketAddr;

use wasp_core::{http::HttpMethod, request::Request, response::Response};

#[tokio::test]
async fn test_actix_adapter() {
    use wasp_core::http::WaspRunner;
    use wasp_core::WaspServer;
    let address = "127.0.0.1:8080";
    let mut server = WaspServer::new(address).unwrap();
    server.add_route("/hello/test", HttpMethod::GET, |req| {
        let p = req.path;
        Response::new(200, p.as_bytes().to_vec())
    });
    //server.run(WaspRunner::ActixWeb).await.unwrap();
}
