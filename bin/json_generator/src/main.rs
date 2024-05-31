use crate::requests::{create_agreement, request_settlement_proof_with_price_and_data};
use axum::Router;
use clap::Parser;
use generate_data::generate_identical_but_shuffled_prices;
use models::{Agreement, InputData, OutputData};
use serde::ser::StdError;
use server::request::models::AppState;
use starknet::core::types::FieldElement;
use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use to_json::save_to_file;
use tracing_subscriber::FmtSubscriber;
use utils::client::Client;
use utils::server::Server;
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
    #[arg(short, long, default_value_t = 1)]
    agreements_count: u64,

    #[arg(long, default_value_t = String::from("target/generator_output/in.json"))]
    path_in: String,

    #[arg(long, default_value_t = String::from("target/generator_output/out.json"))]
    path_out: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let args: Args = Args::parse();
    let (buy_prices, sell_prices) =
        generate_identical_but_shuffled_prices(args.agreements_count / 2);

    let db = Surreal::new::<Mem>(())
        .await
        .expect("Failed to initialize the database");
    let _ = db.use_ns("test").use_db("test").await;
    let server = Server::new();
    let state: AppState = AppState {
        db,
        server_mock: server.clone(),
    };

    let client = Client::new();
    let router: Router = server::request::router(&state);

    //first 50 buys then 50 sells with the same sum prices
    for buying_price in buy_prices {
        create_agreement(
            FieldElement::ONE,
            FieldElement::from_dec_str(&buying_price.to_string())?,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
            client.clone(),
        )
        .await?;
    }

    for selling_price in sell_prices {
        create_agreement(
            FieldElement::ZERO - FieldElement::ONE,
            FieldElement::from_dec_str(&selling_price.to_string())?,
            URL_REQUEST_QUOTE,
            URL_ACCEPT_CONTRACT,
            router.clone(),
            client.clone(),
        )
        .await?;
    }
    let settlement_price = 1500u64.into();
    let settlement_proof = request_settlement_proof_with_price_and_data(
        URL_REQUEST_SETTLEMENT_PROOF_WITH_DATA,
        client.clone(),
        settlement_price,
        router.clone(),
    )
    .await?;

    let agreements: Vec<Agreement> = settlement_proof
        .contracts
        .iter()
        .map(|contract| Agreement {
            quantity: format!("0x{:x}", contract.quantity),
            nonce: format!("0x{:x}", contract.nonce),
            price: format!("0x{:x}", contract.price),
            server_signature_r: format!("0x{:x}", contract.server_signature_r),
            server_signature_s: format!("0x{:x}", contract.server_signature_s),
            client_signature_r: format!("0x{:x}", contract.client_signature_r),
            client_signature_s: format!("0x{:x}", contract.client_signature_s),
        })
        .collect();

    save_to_file(
        args.path_in.to_string(),
        &InputData {
            client_public_key: format!("0x{:x}", client.public_key().scalar()),
            server_public_key: format!("0x{:x}", server.public_key().scalar()),
            agreements,
            settlement_price: format!("0x{:x}", settlement_price),
        },
    )
    .await?;
    save_to_file(
        args.path_out.to_string(),
        &OutputData {
            settlement_price: format!("0x{:x}", settlement_price),
            expected_diff: format!("0x{:x}", settlement_proof.diff),
        },
    )
    .await?;

    Ok(())
}
