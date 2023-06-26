//use serde_json::json;
use hyper::{Body, Response, Request};
use match_request::Params;


pub(crate) async fn init_route(_req: Request<Body>, _params: Params) -> Response<Body> {
    Response::new(Body::from(
    r#"<div id="app">Loading....</div>
    <script> 
        (async () => {
            fetch('./duid_engine.wasm').then(res => 
                res.arrayBuffer()
            ).then(bytes => 
              WebAssembly.instantiate(bytes)
            ).then(wasm => {
                console.log("Bravo Djedou!");
                console.log(wasm);
                wasm.instance.exports.duid_engine();
            });
        })().catch((e) => console.error(e));
    </script>"#))
}
