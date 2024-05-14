use clap::Parser;
use starknet::core::types::FieldElement;
use url::Url;

mod account;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specify the StarkNet RPC URL.
    #[arg(short, long, env)]
    rpc_url: Url,

    /// Specify the Chain Id.
    #[arg(short, long, env)]
    chain_id: FieldElement,

    /// Specify the Account Address.
    #[arg(short, long, env)]
    address: FieldElement,

    /// Specify the Private Key.
    #[arg(short, long, env)]
    private_key: FieldElement,

    /// Specify the number of agreements.
    #[arg(short, long, env)]
    number_of_agreements: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let args = Args::parse();

    let account = account::get_account(args.rpc_url.clone(), args.chain_id, args.address, args.private_key);

    tracing::info!("{:?}", args);
}
