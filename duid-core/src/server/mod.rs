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

    Ok(Response::new(Body::from(r#"<div id="app"></div>
                                    <script type="module">
                                        import init, { duid } from './pkg/button_app.js';
                                        (async () => {
                                            console.log("Bravo Djedou!")
                                            await init();
                                            await duid('app');
                                        })();
                                    </script>"#)))
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
