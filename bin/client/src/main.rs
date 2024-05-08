use clap::Parser;
mod account;
mod models;
mod requests;
mod tests;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
   
    #[arg(long, default_value_t = String::from("http://localhost:7005/server/requestQuote"))]
    url_request_quote: String,
    
    #[arg(long, default_value_t = String::from("http://localhost:7005/server/acceptContract"))]
    url_accept_contract: String,

    #[arg(long, default_value_t = String::from("http://localhost:7005/server/requestSettlementProof"))]
    url_request_settlement_proof: String,

    #[arg(short, long, default_value_t = String::from("Sample client address"))]
    address: String,

    #[arg(short, long, default_value_t = 1)]
    quantity: u64,
}

fn main() {}
