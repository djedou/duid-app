pub(crate) mod router;

use hyper::{
    Body, Request, Response, server::Server,
    service::{make_service_fn, service_fn}
};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use crate::server::router::router;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match router(req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(Response::new(Body::from("Hello World")))
    }
}



pub async fn server(address: &str) {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from_str(address).expect("No able to parse address!");

    let make_service = make_service_fn(move |_conn| {
        let service = service_fn(move |req| {
            handle(req)
        });

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });
    
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
