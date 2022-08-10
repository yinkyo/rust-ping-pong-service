extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::str::FromStr;
use dotenv;
use hyper::server::conn::Http;
use hyper::{Body, Request, Response};
use hyper::service::service_fn;
use tokio::net::TcpListener;

async fn ping_service_fn(
    req: Request<Body>
) -> Result<
    Response<Body>,
    Infallible
> {
    Ok(
        Response::new(
            Body::from(
                format!("PONG ! for {:?} || Request Body {:?}", req, req.body().to_owned())
            )
        )
    )
}

#[tokio::main]
async fn main() -> 
    Result<
        (),
        Box<dyn Error + Send + Sync>
    >
{
    dotenv::dotenv().ok();
    env_logger::builder()
        .init();

    // Opening up SocketAddress at certain port that will listen to connections
    let address = SocketAddr::from_str("127.0.0.1:8000").unwrap();
    let listener = TcpListener::bind(address).await?;

    info!("Ping Service is online and running at {:?}", address);

    // Listen to the TCP connection stream
    // And spawn new Server instance upon incomming connection
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let ping_service_handler = service_fn(ping_service_fn);
            info!("Spawning Ping Service server handler for TPC Stream {:?}", stream);

            if let Err(err) = Http::new()
                .serve_connection(
                    stream,
                    ping_service_handler)
                .await {
                    error!("Error while serving Ping Service {:?}", err);
                }
        });
    }
}
