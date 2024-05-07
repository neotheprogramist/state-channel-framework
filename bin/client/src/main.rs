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
use rand::rngs::OsRng;
use serde_json::Value;
use crate::models::RequestQuotationWithPrice;
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
//diff=(settlement_price×net_quantity)−(total buying price−total selling price)
#[cfg(test)]
mod tests {
    use super::*;
    async fn create_agreement(quantity:i64, price:i64,address: &str,url_request_quote:&str, client: &Client,url_accept_contract:&str)-> Result<(),  Box<dyn std::error::Error>> {
        let request_quotation_response = request_quote_with_price(
            address,
            quantity,
            url_request_quote,
            price,
            &client,
        )
        .await?;

        accept_contract(
                request_quotation_response,
                url_accept_contract,
                &client,
            )
            .await?;

        Ok(())
    }
    #[tokio::test]
    async fn test_set_price() -> Result<(),  Box<dyn std::error::Error>>  {
        let address = "my_address";
        let client = Client::new();
        let url_request_quote ="http://localhost:7006/server/requestQuoteWithPrice"; 
        let url_accept_contract ="http://localhost:7006/server/acceptContract"; 
        let url_request_settlement_proof ="http://localhost:7006/server/requestSettlementProof"; 
        

        Ok(())
    }

    // #[tokio::test]
    // async fn test_10_contracts() -> Result<(), Box<dyn std::error::Error>> {
    //     let address = "test_case";
    //     let client = Client::new();
    
    //     let url_request_quote ="http://localhost:7006/server/requestQuoteWithPrice"; 
    //     let url_accept_contract ="http://localhost:7006/server/acceptContract"; 
    //     let url_request_settlement_proof ="http://localhost:7006/server/requestSettlementProofWithPrice"; 
    
    //     let quantities = [1, 10, -5, -1, -1, -1, 3, 4, -1, 4];
    //     let prices = [1000, 544, 1000, 1000, 1000, 1000, 1300, 999, 1000, 999];
    
    //     let mut sum = 0;
    //     let mut buy_sum = 0;
    //     let mut sell_sum = 0;
    
    //     // Create agreements and calculate sums
    //     for (quantity, buying_price) in quantities.iter().zip(prices.iter()) {
    //         create_agreement(*quantity, *buying_price, address, url_request_quote, &client, url_accept_contract).await?;
    //         sum += quantity;
    //         if *quantity > 0 {
    //             buy_sum += quantity * buying_price;
    //         } else {
    //             sell_sum += quantity.abs() * buying_price;
    //         }
    //     }
    
    //     let settlement_price: i64 = 1500;
    //     let expected = (settlement_price * sum) - (buy_sum - sell_sum);
    
    //     // Request settlement
    //     let settlement_proof = request_settlement_proof_with_price(
    //         url_request_settlement_proof, 
    //         &address.to_string(), 
    //         settlement_price, 
    //         &client
    //     ).await?;
    
    //     assert_eq!(settlement_proof.diff, expected, "The calculated diff does not match the expected gain.");
    
    //     Ok(())
    // }

    #[tokio::test]
    async fn test_10_contracts() -> Result<(),  Box<dyn std::error::Error>>  {
        let address = "test_case";
        let client = Client::new();

        let url_request_quote ="http://localhost:7006/server/requestQuoteWithPrice"; 
        let url_accept_contract ="http://localhost:7006/server/acceptContract"; 
        let url_request_settlement_proof ="http://localhost:7006/server/requestSettlementProofWithPrice"; 

        let mut sum =0;
        let mut buy_sum=0;
        let mut sell_sum=0;
        let quantity:i64  = 1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        buy_sum+=buying_price;

        let quantity:i64  = 10;
        let buying_price = 544;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        buy_sum+=buying_price;

        //SELL
        let quantity:i64  = -5;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sell_sum+=buying_price;

        sum+=quantity;
        //SELL
        let quantity:i64  = -1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        sell_sum+=buying_price;

        //SELL
        let quantity:i64  = -1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        sell_sum+=buying_price;

        //SELL
        let quantity:i64  = -1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        sell_sum+=buying_price;

        let quantity:i64  = 3;
        let buying_price = 1300;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        buy_sum+=buying_price;

        let quantity:i64  = 4;
        let buying_price = 999;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        buy_sum+=buying_price;

        //SELL
        let quantity:i64  = -1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        sell_sum+=buying_price;

        let quantity:i64  = 4;
        let buying_price = 999;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;
        sum+=quantity;
        buy_sum+=buying_price;

        let settlement_price :i64 = 1500;
        let expected = (settlement_price*sum)-(buy_sum-sell_sum);
          // Request settlement
        let settlement_proof =
        request_settlement_proof_with_price(url_request_settlement_proof, &address.to_string(),settlement_price, &client)
              .await?;

        assert_eq!(settlement_proof.diff,expected);

        Ok(())
    }
    #[tokio::test]
    async fn test_main_simple() -> Result<(),  Box<dyn std::error::Error>>  {
        let address = "test_case";
        let client = Client::new();

        let url_request_quote ="http://localhost:7006/server/requestQuoteWithPrice"; 
        let url_accept_contract ="http://localhost:7006/server/acceptContract"; 
        let url_request_settlement_proof ="http://localhost:7006/server/requestSettlementProofWithPrice"; 

        let quantity:i64  = 1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;


        let quantity:i64  = 2;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;


        let quantity:i64  = -2;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;



        let quantity:i64  = -1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;


        let settlement_price :i64 = 1500;
          // Request settlement
        let settlement_proof =
        request_settlement_proof_with_price(url_request_settlement_proof, &address.to_string(),settlement_price, &client)
              .await?;

        let expected_gain = 0;
        assert_eq!(settlement_proof.diff,expected_gain);

        Ok(())
    }

    #[tokio::test]
    async fn test_idroo() -> Result<(),  Box<dyn std::error::Error>>  {
        let address = "test_case";
        let client = Client::new();

        let url_request_quote ="http://localhost:7006/server/requestQuoteWithPrice"; 
        let url_accept_contract ="http://localhost:7006/server/acceptContract"; 
        let url_request_settlement_proof ="http://localhost:7006/server/requestSettlementProofWithPrice"; 

        let quantity:i64  = 1;
        let buying_price = 1000;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;


        let quantity:i64  =1;
        let buying_price = 1100;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;


        let quantity:i64  = -1;
        let buying_price = 1200;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;



        let quantity:i64  = -1;
        let buying_price = 1300;
        create_agreement(quantity, buying_price, address,url_request_quote, &client,url_accept_contract).await?;


        let settlement_price :i64 = 1500;
          // Request settlement
        let settlement_proof =
        request_settlement_proof_with_price(url_request_settlement_proof, &address.to_string(),settlement_price, &client)
              .await?;

        let expected_gain = -400;
        assert_eq!(settlement_proof.diff,expected_gain);

        Ok(())
    }


    #[tokio::test]
    async fn test_pool() -> Result<(),  Box<dyn std::error::Error>>  {
        let url_request_quote ="http://localhost:7006/server/requestQuoteWithPrice"; 
        let url_accept_contract ="http://localhost:7006/server/acceptContract"; 
        let url_request_settlement_proof: &str ="http://localhost:7006/server/requestSettlementProofWithPrice"; 
        let address = "test_case";
        let client = Client::new();

        let mut buying_prices = vec![1000, 1000, 1000, 1000];
        let mut buy_or_sell = vec![1, 1, -1, -1];
    

        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();


        while !buying_prices.is_empty() {
            let index = rng.gen_range(0..buying_prices.len());
            let quantity = buy_or_sell.remove(index);
            let buying_price = buying_prices.remove(index);
    
            create_agreement(quantity, buying_price, address, url_request_quote, &client, url_accept_contract).await?;
        }

        let settlement_price :i64 = 1500;
        let settlement_proof =
        request_settlement_proof_with_price(url_request_settlement_proof, &address.to_string(),settlement_price, &client)
              .await?;

        let expected_gain = 0;
        assert_eq!(settlement_proof.diff,expected_gain);

        Ok(())
    }
    use proptest::prelude::*;
    use futures::future::try_join_all; // Use this crate to handle concurrent futures
    use proptest::collection::{vec, VecStrategy};
    use proptest::strategy::{Just, Strategy};

    //TOO many global rejecs 
    proptest! {

        #[test]
        fn test_50buys_50sells(ops in vec(proptest::sample::select(vec![1i64; 50].into_iter().chain(vec![-1i64; 50]).collect::<Vec<_>>()), 100).prop_shuffle()) {
            // Ensure we have 50 of each -1 and 1 in ops
     
           // Create a single Tokio runtime and reqwest client for the whole test
           let runtime = tokio::runtime::Runtime::new().unwrap();
           let client = reqwest::Client::new();
   
           runtime.block_on(async {
               let address = "test_address".to_string();
               let url_request_quote = "http://localhost:7006/server/requestQuoteWithPrice".to_string();
               let url_accept_contract = "http://localhost:7006/server/acceptContract".to_string();
               let url_request_settlement_proof = "http://localhost:7006/server/requestSettlementProofWithPrice".to_string();
   
               // Process all agreements concurrently
               let futures = ops.into_iter().map(|quantity| {
                   let client = &client;
                   let url_request_quote = &url_request_quote;
                   let url_accept_contract = &url_accept_contract;
                   let address = &address;
                   async move {
                       let price = 1000; // Fixed price
                       create_agreement(quantity, price, address, url_request_quote, client, url_accept_contract).await
                   }
               }).collect::<Vec<_>>();
   
               let results = try_join_all(futures).await;
               assert!(results.is_ok(), "One or more agreements failed to create successfully.");
   
               // Check settlement proof
               let settlement_price = 1500i64;
               let settlement_proof = request_settlement_proof_with_price(&url_request_settlement_proof, &address, settlement_price, &client).await;
               assert_eq!(settlement_proof.unwrap().diff, 0, "Expected gain did not match.");
           });

       }

   }

   
   fn unique_price_strategy() -> impl Strategy<Value = Vec<i64>> {
    let mut prices = (100..=1000).step_by(9).collect::<Vec<i64>>(); // Generate a range of prices
    proptest::collection::vec(proptest::sample::select(prices), 100).prop_shuffle()
    }
   proptest! {
    #[test]
    fn test_100_buys(prices in unique_price_strategy()) {
        // Setup a fixed settlement price
        let settlement_price: i64 = 1500;

        // Ensure we have 100 unique prices
        prop_assert_eq!(prices.len(), 100, "Did not get 100 unique prices.");
        prop_assert!(prices.iter().all(|&price| price >= 100 && price <= 1000), "Prices are out of the expected range.");

        // Calculate the total sum of selected prices
        let total_price_sum: i64 = prices.iter().sum();

        // Calculate the expected result
        let expected_result = total_price_sum - 100 * settlement_price;

        // Create a single Tokio runtime and reqwest client for the whole test
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = reqwest::Client::new();
        
        let actual_result = runtime.block_on(async {
            let address = "test_address".to_string();
            let url_request_quote = "http://localhost:7006/server/requestQuoteWithPrice".to_string();
            let url_accept_contract = "http://localhost:7006/server/acceptContract".to_string();
            let url_request_settlement_proof = "http://localhost:7006/server/requestSettlementProofWithPrice".to_string();

            // Process all agreements concurrently (Assuming each price should be processed)
            let futures = prices.iter().map(|&price| {
                let client = &client;
                let url_request_quote = &url_request_quote;
                let url_accept_contract = &url_accept_contract;
                let address = &address;
                async move {
                    create_agreement(1, price, address, url_request_quote, client, url_accept_contract).await
                }
            }).collect::<Vec<_>>();

            let results = try_join_all(futures).await;
            assert!(results.is_ok(), "One or more agreements failed to create successfully.");

            // Check settlement proof
            let settlement_proof = request_settlement_proof_with_price(&url_request_settlement_proof, &address, settlement_price, &client).await;
            assert!(settlement_proof.is_ok(), "Failed to get settlement proof.");
            settlement_proof.unwrap().diff
        });

        // Assert to check if the actual result meets the expected result
        prop_assert_eq!(actual_result, expected_result, "The calculated result did not match the expected outcome.");
    }
}

    proptest! {
        // Correct the closure syntax for async
        #[test]
        fn test_multiple_agreements(ops in proptest::collection::vec((2i64..3, 1000i64..1001), 1)) {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let client = reqwest::Client::new();
                let address = "test_address";
                let url_request_quote = "http://localhost:7006/server/requestQuoteWithPrice";
                let url_accept_contract = "http://localhost:7006/server/acceptContract";
    
                for (quantity, price) in ops {
                    let result = create_agreement(quantity, price, address, url_request_quote, &client, url_accept_contract).await;
                    assert!(result.is_ok());
                }
            });
                  
    
        }
    }
  
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Args = Args::parse();
    // let client = Client::new();

    // // Create Contract
    // let request_quotation_response = request_quote(
    //     &args.address,
    //     args.quantity,
    //     &args.url_request_quote,
    //     &client,
    // )
    // .await?;
    // println!("Contract");
    // println!("price per BTC: {}", request_quotation_response.quote.price);
    // println!("quantity: {}", request_quotation_response.quote.quantity);
    // println!(
    //     "Sum: {}",
    //     (request_quotation_response.quote.quantity as u64) * request_quotation_response.quote.price
    // );
    // println!(
    //     "Client address: {}",
    //     request_quotation_response.quote.address
    // );

    // let accepted_contract = Confirm::new()
    //     .with_prompt("Do you accept the contract?")
    //     .interact()
    //     .unwrap_or(false);

    // if accepted_contract {
    //     accept_contract(
    //         request_quotation_response,
    //         &args.url_accept_contract,
    //         &client,
    //     )
    //     .await?;
    // } else {
    //     println!("{}", style("Contract declined.").red());
    // }

    // let settle_proof = Confirm::new()
    //     .with_prompt("Do you want to settle the proof?")
    //     .interact()
    //     .unwrap_or(false);

    // if settle_proof {
    //     // Request settlement
    //     let settlement_proof =
    //         request_settlement_proof(&args.url_request_settlement_proof, &args.address, &client)
    //             .await?;
    //     println!("Settlement proof");
    //     println!("Address: {}", settlement_proof.address);
    //     println!("Balance: {}", settlement_proof.balance);
    //     println!("Diff: {}", settlement_proof.diff);
    // } else {
    //     println!("{}", style("Contract declined.").red());
    // }

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

    let balance: f64 = match json_body["balance"].as_f64() {
        Some(balance) => balance,
        None => return Err("Balance not found in JSON response or not a valid float".into()),
    };
    let diff: i64 = match json_body["diff"].as_i64() {
        Some(diff) => diff,
        None => return Err("Diff not found in JSON response".into()),
    };

    Ok(SettlementProofResponse {
        address,
        balance,
        diff,
    })
}

async fn request_settlement_proof_with_price(
    url: &str,
    address: &String,
    price:i64,
    client: &Client,
) -> Result<SettlementProofResponse, Box<dyn std::error::Error>> {
    let url_with_params = format!("{}?address={}&price={}", url, address,price);

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

    let balance: f64 = match json_body["balance"].as_f64() {
        Some(balance) => balance,
        None => return Err("Balance not found in JSON response or not a valid float".into()),
    };
    let diff: i64 = match json_body["diff"].as_i64() {
        Some(diff) => diff,
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
    quantity: i64,
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

async fn request_quote_with_price(
    address: &str,
    quantity: i64,
    url: &str,
    price: i64,
    client: &Client,
) -> Result<RequestQuotationResponse, Box<dyn std::error::Error>> {
    let request_quotation = RequestQuotationWithPrice {
        address: address.to_string(),
        quantity,
        price
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
