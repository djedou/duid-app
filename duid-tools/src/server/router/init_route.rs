//use serde_json::json;
use hyper::{Body, Response, Request};
use match_request::Params;


pub(crate) async fn init_route(_req: Request<Body>, _params: Params) -> Response<Body> {
    Response::new(Body::from(
    r#"<div id="app">Loading....</div>
    <script type="module">
        import init, { duid_engine } from '/duid_engine';
        (async () => {
            await init();
            await duid_engine();
        })();
    </script>"#))
}
