use hyper::{Body, Response, Request};
use crate::server::router::response::{ok, ok_js, ok_wasm};
use match_request::Params;
use std::fs;

pub(crate) async fn duid_engine_js(_req: Request<Body>, _params: Params) -> Response<Body> {
    match read_file_vec("./pkg/duid_engine.js") {
        Ok(res) => {
            ok_js(Body::from(res))
        },
        Err(_) => {
            ok(Body::from("Bravo"))
        }
    }
}

pub(crate) async fn duid_engine_wasm(_req: Request<Body>, _params: Params) -> Response<Body> {
    match read_file_vec("./pkg/duid_engine_bg.wasm") {
        Ok(res) => {
            ok_wasm(Body::from(res))
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