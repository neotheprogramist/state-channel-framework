use clap::Parser;
<<<<<<< HEAD
mod account;
mod models;
=======
>>>>>>> eccd0f5809fe7f255d5d3940c022adb1d6778b23
mod requests;
mod tests;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
<<<<<<< HEAD
   
    #[arg(long, default_value_t = String::from("http://localhost:7005/server/requestQuote"))]
    url_request_quote: String,
    
=======
    #[arg(long, default_value_t = String::from("http://localhost:7005/server/requestQuote"))]
    url_request_quote: String,

>>>>>>> eccd0f5809fe7f255d5d3940c022adb1d6778b23
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
