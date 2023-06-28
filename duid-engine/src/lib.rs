mod engine;
//pub mod nodes;
pub(crate) mod memory;
/// Duid router module
pub mod router;

pub use self::engine::DuidEngine;
use wasm_bindgen::prelude::*;
use tracing::info;

#[wasm_bindgen]
pub fn duid_engine() {
    
   let _ = DuidEngine::start();
   info!("Duid Engine Ended !!!!!");
}
