use std::net::SocketAddr;

use hyper::{service::service_fn, Server, Request, Body, Response, Client};
use tower::make::Shared;

async fn log(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();

    if path.starts_with("/api") {
        println!("API path: {}", path);
    } else {
        println!("Generic path: {}", path);
    }
    return handle(req).await;
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    return client.request(req).await;
}

#[tokio::main]
async fn main() {
    let make_service = Shared::new(service_fn(log));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}
