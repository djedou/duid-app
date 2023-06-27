
/// Duid core module.
pub mod core;
/// Duid server module.
pub mod server;
/// Duid compiler module.
pub mod compiler;

pub mod web_sys {
    pub use web_sys::{
        /*EventTarget, HtmlElement, HtmlStyleElement, Node, CanvasRenderingContext2d, HtmlCanvasElement, 
        Document, Window, History, Location, */console
    };
}