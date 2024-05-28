use clap::Parser;

pub mod request;
pub mod server;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Host address to bind the server
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[clap(long, default_value = "7007")]
    port: u16,
}
