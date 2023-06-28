//mod user_routes;
pub(crate) mod unknowned;
pub(crate) mod response;
pub(crate) mod init_route;
pub(crate) mod engine_routes;

use match_request::{match_request, Error, Params};
use hyper::{Request, Response, Body};
use futures::future::{BoxFuture};
use multipart::server::FieldHeaders;
use self::{
    unknowned::unknowed_route,
    init_route::init_route,
    engine_routes::{duid_engine_js, duid_engine_wasm}
};


// A boxed type definition for your async views.
pub type RouterHandler = Box<dyn Fn(Request<Body>, Params) -> BoxFuture<'static, Response<Body>> + Send + Sync>;


#[derive(Debug)]
pub(crate) struct FormData {
    pub(crate) headers: FieldHeaders,
    pub(crate) data: String
}


#[macro_export]
macro_rules! route_handler {
    ($closure:expr) => {{
        #[allow(unused_mut)]
        let mut closure = $closure;
        let b: crate::server::router::RouterHandler
         = Box::new(move |req, params| {
            Box::pin(closure(req, params))
        });
        b
    }};
}

// An example request router.
pub async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    let method = req.method();
    let path = req.uri().path();

    let (handler, params) = match_request!(method, path, {
        "/" => {
            GET => crate::route_handler!(init_route), 
            POST => crate::route_handler!(unknowed_route), 
        },
        "/duid_engine" => {
            GET => crate::route_handler!(duid_engine_js), 
            POST => crate::route_handler!(unknowed_route), 
        },
        "/favicon.ico" => {
            POST => crate::route_handler!(unknowed_route), 
            GET => crate::route_handler!(unknowed_route), 
        },
        "/duid_engine_bg.wasm" => {
            GET => crate::route_handler!(duid_engine_wasm),
            POST => crate::route_handler!(unknowed_route),
        },
        "/*" => {
            POST => crate::route_handler!(unknowed_route),
            GET => crate::route_handler!(unknowed_route),
        }
    })?;

    Ok(handler(req, params).await)
}