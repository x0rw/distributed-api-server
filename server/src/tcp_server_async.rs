use crate::{builder::HttpBuilder, handler::handle_http, routes::RoutesMap, Result};
use std::io::{Read, Write};
use tokio::io::{AsyncBufRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

pub struct TcpServer {
    hostaddr: String,
    listener: TcpListener,
    routes: RoutesMap,
}

impl TcpServer {
    pub async fn new(hostaddr: String, routes: RoutesMap) -> Result<Self> {
        Ok(Self {
            hostaddr: "moved".to_string(),
            listener: TcpListener::bind(hostaddr).await?,
            routes,
        })
    }
    pub async fn launch(&self) -> Result<()> {
        loop {
            let (mut stream, _sockaddr) = self.listener.accept().await.unwrap();
            tokio::spawn(self.handle_client(stream));
        }
    }

    // all read and write sys calls should be done here
    // errors propagated from this layer are crucial
    async fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
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
        let route = self.routes.get(uri);
        let http_response = HttpBuilder::new(handler, route).build();

        let stream_send = stream.write(http_response.as_bytes()).await?;
        println!("{stream_send} Bytes sent to the client");
        Ok(())
    }
}

#[tokio::main]
async fn main() {}
