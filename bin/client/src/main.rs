use clap::Parser;
use models::{
    AgreeToQuotation, RequestQuotation, RequestQuotationResponse, SettlementProofResponse,
};
use reqwest::Client;
mod account;
mod models;
use crate::account::scalar_to_hex;
use account::MockAccount;
use dialoguer::console::style;
use dialoguer::Confirm;
use rand::rngs::OsRng;
use serde_json::Value;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let client = Client::new();

    // Create Contract
    let request_quotation_response = request_quote(
        &args.address,
        args.quantity,
        &args.url_request_quote,
        &client,
    )
    .await?;
    println!("Contract");
    println!("price per BTC: {}", request_quotation_response.quote.price);
    println!("quantity: {}", request_quotation_response.quote.quantity);
    println!(
        "Sum: {}",
        (request_quotation_response.quote.quantity as f64) * request_quotation_response.quote.price
    );
    println!(
        "Client address: {}",
        request_quotation_response.quote.address
    );

    let accepted_contract = Confirm::new()
        .with_prompt("Do you accept the contract?")
        .interact()
        .unwrap_or(false);

    if accepted_contract {
        accept_contract(
            request_quotation_response,
            &args.url_accept_contract,
            &client,
        )
        .await?;
    } else {
        println!("{}", style("Contract declined.").red());
    }

    let settle_proof = Confirm::new()
        .with_prompt("Do you want to settle the proof?")
        .interact()
        .unwrap_or(false);

    if settle_proof {
        // Request settlement
        let settlement_proof =
            request_settlement_proof(&args.url_request_settlement_proof, &args.address, &client)
                .await?;
        println!("Settlement proof");
        println!("Address: {}", settlement_proof.address);
        println!("Balance: {}", settlement_proof.balance);
        println!("Diff: {}", settlement_proof.diff);
    } else {
        println!("{}", style("Contract declined.").red());
    }

    Ok(())
}

async fn request_settlement_proof(
    url: &str,
    address: &String,
    client: &Client,
) -> Result<SettlementProofResponse, Box<dyn std::error::Error>> {
    let url_with_params = format!("{}?address={}", url, address);

    let response = match client.get(url_with_params).send().await {
        Ok(response) => response,
        Err(err) => return Err(err.into()),
    };

    let response_text = match response.text().await {
        Ok(text) => {
            println!("{}", text);
            text
        }
        Err(err) => return Err(err.into()),
    };
    let json_body: Value = match serde_json::from_str(&response_text) {
        Ok(json) => json,
        Err(err) => return Err(err.into()),
    };
    let address = match json_body["address"].as_str() {
        Some(address) => address.to_string(),
        None => return Err("Address not found in JSON response".into()),
    };

    let balance = match json_body["balance"].as_str() {
        Some(balance_str) => match balance_str.parse::<f64>() {
            Ok(balance) => balance,
            Err(_) => return Err("Failed to parse diff as f64".into()),
        },
        None => return Err("Balance not found in JSON response".into()),
    };

    let diff = match json_body["diff"].as_str() {
        Some(diff_str) => match diff_str.parse::<f64>() {
            Ok(diff) => diff,
            Err(_) => return Err("Failed to parse diff as f64".into()),
        },
        None => return Err("Diff not found in JSON response".into()),
    };

    Ok(SettlementProofResponse {
        address,
        balance,
        diff,
    })
}

async fn accept_contract(
    request_quotation_response: RequestQuotationResponse,
    url: &str,
    client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let data_to_sign = serde_json::to_string(&request_quotation_response)?;
    let quote_data = serde_json::to_string(&data_to_sign).unwrap();
    let quote_bytes = quote_data.as_bytes();
    let mut rng = OsRng;
    let mock_account = MockAccount::new(&mut rng);
    let client_signature = mock_account.sign_message(&quote_bytes, &mut rng);

    let client_signature = match client_signature {
        Ok(signature) => {
            let signature_json = format!(
                "{{\"r\": \"{}\", \"s\": \"{}\"}}",
                scalar_to_hex(&signature.r),
                scalar_to_hex(&signature.s)
            );
            signature_json
        }
        Err(e) => {
            println!("Failed to sign message: {}", e);
            return Err(e.into());
        }
    };

    let request_quotation = AgreeToQuotation {
        quote: request_quotation_response.quote,
        server_signature: request_quotation_response.server_signature,
        client_signature: client_signature.to_string(),
    };

    //4. Accept the contract
    let agree_to_quotation_response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_quotation)
        .send()
        .await?;

    if agree_to_quotation_response.status().is_success() {
        println!("{}", style("Contract created successfully!").green());
        Ok(())
    } else {
        println!(
            "Contract failed with status: {}",
            agree_to_quotation_response.status()
        );
        Err("Contract failed".into())
    }
}

async fn request_quote(
    address: &str,
    quantity: u64,
    url: &str,
    client: &Client,
) -> Result<RequestQuotationResponse, Box<dyn std::error::Error>> {
    let request_quotation = RequestQuotation {
        address: address.to_string(),
        quantity,
    };

    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_quotation)
        .send()
        .await?;

    if response.status().is_success() {
        let response_data: RequestQuotationResponse = response.json().await?;
        Ok(response_data)
    } else {
        Err("Request failed".into())
    }
}
