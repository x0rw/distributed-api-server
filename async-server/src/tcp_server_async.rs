use base::error::Result;
use base::http::builder::HttpBuilder;
use base::http::handler::handle_http;
//use base::http::{builder, handler, header};
use base::routes::RoutesMap;
use std::sync::Arc;
use tokio::io::{AsyncBufRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub struct AsyncTcpServer {
    hostaddr: String,
    listener: TcpListener,
    routes: Arc<RoutesMap>,
}

impl AsyncTcpServer {
    pub async fn new(hostaddr: String, routes: RoutesMap) -> Result<Self> {
        Ok(Self {
            hostaddr: "moved".to_string(),
            listener: TcpListener::bind(hostaddr).await?,
            routes: Arc::new(routes),
        })
    }
    pub async fn launch(self) -> Result<()> {
        loop {
            let (mut stream, _sockaddr) = self.listener.accept().await.unwrap();
            let arc_route = Arc::clone(&self.routes);
            tokio::spawn(async_handle_client(arc_route, stream));
        }
    }
}
async fn async_handle_client(route: Arc<RoutesMap>, mut stream: TcpStream) -> Result<()> {
    println!("Client Connected");
    let mut buffer = [0; 1000];
    stream.read(&mut buffer).await?;
    let buffer_utf8 = String::from_utf8_lossy(&buffer[..]).to_string();
    // println!("{}", buffer_utf8.to_string());
    let handler = match handle_http(&buffer_utf8) {
        Ok(e) => e,
        Err(e) => {
            stream
                .write(HttpBuilder::build_badrequest().as_bytes())
                .await?;
            println!("client handler error {}", e);
            return Ok(()); // errors in handle_http arent that serious
        }
    };
    let uri = &handler.req_line.uri;
    let route = route.get(uri);
    let http_response = HttpBuilder::new(handler, route).build();

    let stream_send = stream.write(http_response.as_bytes()).await?;
    println!("{stream_send} Bytes sent to the client");
    stream.shutdown().await?;
    Ok(())
}

#[tokio::main]
async fn main() {}
