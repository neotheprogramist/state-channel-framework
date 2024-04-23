use std::net::AddrParseError;

use clap::Parser;
use server::start;
use thiserror::Error;

mod prove;
mod server;

#[derive(Debug, Error)]
enum ProverError {
    #[error("server error")]
    Server(#[from] std::io::Error),

    #[error("failed to parse address")]
    AddressParse(#[from] AddrParseError),
}

/// Command line arguments for the server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Host address to bind the server
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[clap(long, default_value = "3618")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), ProverError> {
    let args = Args::parse();

    // Construct the full address string
    let address = format!("{}:{}", args.host, args.port);

    // Start the server with the specified address
    start(address.parse()?).await?;

    Ok(())
}
