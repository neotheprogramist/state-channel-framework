use clap::Parser;
mod account;
mod models;
mod requests;
mod tests;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = ("http://localhost:7005/server/requestQuote".to_string()))]
    url_request_quote: String,

    #[arg(long, default_value_t = ("http://localhost:7005/server/acceptContract".to_string()))]
    url_accept_contract: String,

    #[arg(long, default_value_t = ("http://localhost:7005/server/requestSettlementProof".to_string()))]
    url_request_settlement_proof: String,

    #[arg(short, long, default_value_t = ("Sample client address".to_string()))]
    address: String,

    #[arg(short, long, default_value_t = 1)]
    quantity: u64,
}

fn main() {}
