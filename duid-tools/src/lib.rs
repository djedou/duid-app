pub mod server;
use std::fs;

use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;


pub fn init_trace() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();
}
