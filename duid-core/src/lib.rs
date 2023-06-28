
/// Duid core module.
pub mod core;
/// Duid compiler module.
pub mod compiler;

/*pub mod web_sys {
    pub use web_sys::{
        EventTarget, HtmlElement, HtmlStyleElement, Node, CanvasRenderingContext2d, HtmlCanvasElement, 
        Document, Window, History, Location, 
    };
}*/


pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

pub fn init_trace() {
    console_log::init_with_level(tracing::log::Level::Debug).unwrap();
    std::panic::set_hook(Box::new(|info| {
        tracing::error!("{:?}", info);
    }));
}
