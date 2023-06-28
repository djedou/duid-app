use duid_app::duid_core::{
    console::info,
    core::router::Router,
    init_trace
};
use std::rc::Rc;

pub struct DuidEngine {
    pub router: Router
}

impl DuidEngine {

    pub fn start() -> DuidEngine {
        init_trace();

        info!("Bravo Djedou, it works from wasm!!!!!");

        let duid_engine = DuidEngine {
            router: Router::new()
        };

        duid_engine.render_route(None);
        duid_engine
    }

    fn render_route(&self, route: Option<Rc<&'static str>>) {
        match route {
            Some(r) => {
                self.load_route_wasm(&r);
            },
            None => {
                self.load_route_wasm("/.");
            }
        }
    }

    fn load_route_wasm(&self, route: &'static str) {

    }
}

