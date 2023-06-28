use tracing::info;
use crate::router::Router;
use std::rc::Rc;

pub struct DuidEngine {
    pub router: Router
}

impl DuidEngine {

    pub fn start() -> DuidEngine {
        console_log::init_with_level(tracing::log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(|info| {
            tracing::error!("{:?}", info);
        }));

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

