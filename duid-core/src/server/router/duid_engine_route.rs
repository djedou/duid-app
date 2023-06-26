use hyper::{Body, Response, Request};
use crate::server::router::response::ok;
use match_request::Params;
use std::fs;

pub(crate) async fn duid_engine_route(_req: Request<Body>, _params: Params) -> Response<Body> {
    println!("duid engine route!");
    match read_file_vec("./src/engine/duid_engine_bg.wasm") {
        Ok(res) => {
            ok(Body::from(res))
        },
        Err(_) => {
            ok(Body::from("Bravo"))
        }
    }
}

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}