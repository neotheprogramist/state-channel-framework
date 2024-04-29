use clap::Parser;
use reqwest::Client;
use serde::{Deserialize,Serialize};
use serde_with::{serde_as, DisplayFromStr};
use models::{Nonce,Quote,RequestQuotation,RequestQuotationResponse,AgreeToQuotation};
use account::MockAccount;
use ed25519_dalek::Signature;
mod models;
mod account;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value_t = ("http://localhost:7003/server/requestQuote".to_string()))]
    url_request_quote: String,

    #[arg(long, default_value_t = ("http://localhost:7003/server/acceptContract".to_string()))]
    url_accept_contract: String,

    #[arg(short, long)]
    address: String,

    #[arg(short, long,default_value_t=1)]
    quantity: u64,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args: Args = Args::parse();

    // Build HTTP client
    let client = Client::new();
    let request_quotation = RequestQuotation {
        address: args.address,
        quantity: args.quantity,
    };

    let response = client.post(&args.url_request_quote)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_quotation)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Request successful!");
        let response_data: RequestQuotationResponse = response.json().await?;
        println!("Response \n  quote:{} \n sever_signature:{}", response_data.quote,response_data.server_signature);
        // Serialize the response data for signing
        let data_to_sign = serde_json::to_string(&response_data)?;

        let mut mock_account = MockAccount::new();
        let client_signature: Signature = mock_account.sign_message(data_to_sign.as_bytes());

        println!("Client signature: {}", client_signature);
        let request_quotation = AgreeToQuotation {
            quote: response_data.quote,
            server_signature:response_data.server_signature,
            client_signature: client_signature.to_string()
        };

        let agree_to_quotatinon_response = client.post(&args.url_accept_contract)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_quotation)
        .send()
        .await?;
        if agree_to_quotatinon_response.status().is_success() {
            println!("Agreee to quotation successful!");
        } else {
            println!("Agreee to quotation  failed with status: {}", agree_to_quotatinon_response.status());

        }
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

