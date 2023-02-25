use std::{convert::Infallible, net::SocketAddr};

use hyper::{body::HttpBody, Body, Request, Response};

async fn handler(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let bytes = req.body_mut().data().await.unwrap().unwrap();
    let msg = String::from_utf8_lossy(&bytes);

    let received = didcomm_rs::Message::receive(&msg, None, None, None).unwrap();
    dbg!(received);

    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3010));

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(hyper::service::service_fn(handler))
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        panic!("server error: {}", e);
    }
}
