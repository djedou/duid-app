
use std::env;
use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;
use hyper::{
    Body, Request, Response, server::Server,
    service::{make_service_fn, service_fn}
};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;


async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    /*match router(sys, req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(Response::new(Body::from("Hello World")))
    }*/

    Ok(Response::new(Body::from("Hello World")))
}



pub async fn server() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    let address = match env::var("Address") {
        Ok(a) => a,
        Err(_) => String::from("192.168.1.5:4010")
    };

    info!("Server: http://{}", address);
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from_str(address.as_ref()).expect("No able to parse address!");

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
