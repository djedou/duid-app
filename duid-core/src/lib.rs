/*
/// Duid core module.
//pub mod core;
/// Duid compiler module.
pub mod compiler;
pub(crate) mod arena;
pub(crate) mod dom;
pub(crate) mod effects;
pub(crate) mod tailwindcss_system;*/

pub mod console {
    pub use tracing::{info, error, trace, debug, warn};
}

pub fn init_trace() {
    console_log::init_with_level(tracing::log::Level::Debug).unwrap();
    std::panic::set_hook(Box::new(|info| {
        tracing::error!("{:?}", info);
    }));
}
