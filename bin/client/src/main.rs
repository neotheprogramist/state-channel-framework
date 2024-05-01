use clap::Parser;
use models::{AgreeToQuotation, RequestQuotation, RequestQuotationResponse};
use reqwest::Client;
mod account;
mod models;
use crate::account::scalar_to_hex;
use account::MockAccount;
use rand::rngs::OsRng;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = ("http://localhost:7003/server/requestQuote".to_string()))]
    url_request_quote: String,

    #[arg(long, default_value_t = ("http://localhost:7003/server/acceptContract".to_string()))]
    url_accept_contract: String,

    #[arg(short, long)]
    address: String,

    #[arg(short, long, default_value_t = 1)]
    quantity: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    // 1. Get address and quantity from cli
    let client = Client::new();
    let request_quotation = RequestQuotation {
        address: args.address,
        quantity: args.quantity,
    };

    // 2. Request quote from server
    let response = client
        .post(&args.url_request_quote)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_quotation)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Request successful!");
        let response_data: RequestQuotationResponse = response.json().await?;
        println!(
            "Response \n  quote:{} \n sever_signature:{}",
            response_data.quote, response_data.server_signature
        );
        let data_to_sign = serde_json::to_string(&response_data)?;
        let quote_json = serde_json::to_string(&data_to_sign).unwrap();
        let quote_bytes = quote_json.as_bytes();
        
        //3.Client signs the data with stark_curve
        let mut rng = OsRng; 
        let mut mock_account = MockAccount::new(&mut rng);
        let client_signature = mock_account.sign_message(&quote_bytes, &mut rng);

        //formating client signature
        let client_signature = match client_signature {
            Ok(signature) => {
                let signature_json = format!(
                    "{{\"r\": \"{}\", \"s\": \"{}\"}}",
                    scalar_to_hex(&signature.r),
                    scalar_to_hex(&signature.s)
                );
                println!("Serialized Signature: {}", signature_json);
                signature_json
            }
            Err(e) => {
                println!("Failed to sign message: {}", e);
                return Err(e.into());
            }
        };

        let request_quotation = AgreeToQuotation {
            quote: response_data.quote,
            server_signature: response_data.server_signature,
            client_signature: client_signature.to_string(),
        };
        //4. Accept the contract 
        let agree_to_quotatinon_response = client
            .post(&args.url_accept_contract)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&request_quotation)
            .send()
            .await?;

        if agree_to_quotatinon_response.status().is_success() {
            println!("Agreee to quotation successful!");
        } else {
            println!(
                "Agreee to quotation  failed with status: {}",
                agree_to_quotatinon_response.status()
            );
        }
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
