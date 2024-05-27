use crate::requests::{create_agreement, request_settlement_proof_with_price_and_data};
use axum::Router;
use clap::Parser;
use generate_data::generate_identical_but_shuffled_prices;
use serde::ser::StdError;
use server::request::account::MockAccount;
use server::request::models::AppState;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use to_json::{prepare_and_save_data, save_out};
mod generate_data;
pub mod models;
pub mod requests;
mod to_json;

const URL_ACCEPT_CONTRACT: &str = "/acceptContract";
const URL_REQUEST_QUOTE: &str = "/requestQuoteWithPrice";
const URL_REQUEST_SETTLEMENT_PROOF_WITH_DATA: &str = "/requestSettlementProofWithPriceAndData";
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("http://localhost:7005/server/requestQuote"))]
    url_request_quote: String,

    #[arg(long, default_value_t = String::from("http://localhost:7005/server/acceptContract"))]
    url_accept_contract: String,

    #[arg(long, default_value_t = String::from("http://localhost:7005/server/requestSettlementProof"))]
    url_request_settlement_proof: String,

    #[arg(short, long, default_value_t = 1)]
    agreements_count: u64,

    #[arg(long, default_value_t = String::from("target/generator_output/in.json"))]
    path_in: String,

    #[arg(long, default_value_t = String::from("target/generator_output/out.json"))]
    path_out: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args: Args = Args::parse();
    let agreements_count = args.agreements_count / 2;
    let (buy_prices, sell_prices) = generate_identical_but_shuffled_prices(agreements_count);

    let address = "0x4b3f4ba8c00a02b66142a4b1dd41a4dfab4f92650922a3280977b0f03c75ee1";
    let db = Surreal::new::<Mem>(())
        .await
        .expect("Failed to initialize the database");
    let _ = db.use_ns("test").use_db("test").await;
    let server_mock_account = MockAccount::new();
    let state: AppState = AppState {
        db,
        mock_account: server_mock_account.clone(),
    };

    let client_mock_account = MockAccount::new();

    let router: Router = server::request::router(&state);

    //first 50 buys then 50 sells with the same sum prices
    for buying_price in buy_prices {
        create_agreement(
            1,
            buying_price as i64,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
            client_mock_account.clone(),
        )
        .await?;
    }

    for selling_price in sell_prices {
        create_agreement(
            -1,
            selling_price as i64,
            address,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
            client_mock_account.clone(),
        )
        .await?;
    }
    println!("CORRECT");
    let settlement_price = 1500i64;
    // Request settlement
    let settlement_proof = request_settlement_proof_with_price_and_data(
        URL_REQUEST_SETTLEMENT_PROOF_WITH_DATA,
        &address.to_string(),
        settlement_price,
        router.clone(),
    )
    .await?;

    //Save to files
    prepare_and_save_data(
        args.path_in.to_string(),
        settlement_proof.clone(),
        client_mock_account.clone(),
        server_mock_account.clone(),
    )
    .await?;
    save_out(
        args.path_out.to_string(),
        settlement_price,
        settlement_proof.diff,
    )
    .await?;
    Ok(())
}
