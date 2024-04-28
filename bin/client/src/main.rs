use clap::Parser;
use reqwest::Client;
use serde::{Deserialize,Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotation {
    pub address: String,
    pub quantity: u64,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long, default_value_t = ("http://localhost:7003/server/requestQuote".to_string()))]
    url: String,

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
    let response = client.post(&args.url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&request_quotation)
        .send()
        .await?;

    // Check if the request was successfulcar
    if response.status().is_success() {
        println!("Request successful!");

    } else {
        println!("Request failed with status: {}", response.status());
    }
    // Print the content of the response
    let response_content = response.text().await?;
    println!("Response content: {}", response_content);
        
    Ok(())
}