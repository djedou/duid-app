use duid_app::{
    /*duid_core::{
        console::info,
        core::router::Router
    },*/
    duid_vm::run_vm
};
use std::rc::Rc;
//include!(concat!(env!("OUT_DIR"), "/routes.rs"));

pub struct DuidEngine {
    //pub router: Router
}

impl DuidEngine {

    pub fn start() {
        info!("Bravo Djedou Arnaud, it works from wasms!!!!!");
        run_vm();
        /*let duid_engine = DuidEngine {
            router: Router::new()
        };

        duid_engine.render_route(None);*/
    }

    /*fn render_route(&self, route: Option<Rc<&'static str>>) {
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
        //duid_app::duid_core::user_app!(user_app_routes());
    }*/
}

