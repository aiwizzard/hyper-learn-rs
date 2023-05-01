use std::{net::SocketAddr, convert::Infallible};

use hyper::{service::{make_service_fn, service_fn}, Server, Request, Body, Response, Client};

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    return client.request(req).await;
}

#[tokio::main]
async fn main() {
    let make_service = make_service_fn(|_| async {Ok::<_, Infallible>(service_fn(handle))});
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}
