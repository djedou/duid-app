use duid_tools::{server::server, init_trace};
use clap::Parser;
use tracing::info;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value="0.0.0.0")]
    pub(crate) host: String,
    #[arg(long, default_value="3000")]
    pub(crate) port: u16
}

#[tokio::main]
async fn main() {
    init_trace();

    let args = Args::parse();
    let address = format!("{}:{}", args.host, args.port);
    info!("Server: http://{}", address);
    server(&address).await
}