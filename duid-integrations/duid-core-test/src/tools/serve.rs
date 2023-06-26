use tracing::{info, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;
use duid_app::duid_core::server::server;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value="0.0.0.0")]
    pub(crate) host: String,
    #[arg(long, default_value="3000")]
    pub(crate) port: u16
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber).unwrap();

    let args = Args::parse();
    
    let address = format!("{}:{}", args.host, args.port);

    info!("Server: http://{}", address);
    server(&address).await
}