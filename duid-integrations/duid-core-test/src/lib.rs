mod engine;
// We need this
pub mod app;

pub use self::engine::DuidEngine;
use wasm_bindgen::prelude::*;
use duid_app::duid_core::{
   init_trace, 
   console::info
};

#[wasm_bindgen]
pub fn duid_engine() {
   init_trace();
   DuidEngine::start();
}
