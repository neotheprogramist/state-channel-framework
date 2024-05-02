use clap::Parser;
use ed25519_dalek::ed25519::signature::SignerMut;
use reqwest::Client;
use rand::rngs::OsRng;
//ed25519 imports
use ed25519_dalek::{Signature, Signer};
use ed25519_dalek::SigningKey;
use ed25519_dalek::{VerifyingKey, Verifier};
use serde_json::{Value, Error as SerdeError};
use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = ("http://localhost:7003/server/requestQuote".to_string()))]
    url_request_quote: String,

    #[arg(long, default_value_t = ("http://localhost:7003/server/acceptContract".to_string()))]
    url_accept_contract: String,

    #[arg(long, default_value_t = ("http://localhost:7003/auth".to_string()))]
    url_auth:String,

    #[arg(short, long, default_value_t =("Sample address".to_string()))]
    address: String,

    #[arg(short, long, default_value_t = 1)]
    quantity: u64,
}

#[derive(Debug, Error)]
pub enum ValidatorErrors {

    #[error("HTTP request failed")]
    RequestFailed(#[from] ReqwestError),


    #[error("JSON parsing failed")]
    JsonParsingFailed,

    #[error("Nonce not found in the response")]
    NonceNotFound,

    #[error("JWT token not found in the response")]
    JwtTokenNotFound,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Get Cli args
    let args: Args = Args::parse();

    //Generate (Pk,Sk) keypair for the validator using Ed25519 signature(out validator uses this type of signature). TODO:Ask about it 
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let public_key: VerifyingKey = signing_key.verifying_key();
    
    let jwt_token = authenticate_with_prover(&signing_key,&public_key,&args.url_auth).await?;



    // // 1. Get address and quantity from cli
    // let request_quotation = RequestQuotation {
    //     address: args.address,
    //     quantity: args.quantity,
    // };


    // //Init http client
    // let client = Client::new();
    // // 2. Request quote from server
    // let response = client
    //     .post(&args.url_request_quote)
    //     .header(reqwest::header::CONTENT_TYPE, "application/json")
    //     .json(&request_quotation)
    //     .send()
    //     .await?;

    // if response.status().is_success() {
    //     println!("Request successful!");
    //     let response_data: RequestQuotationResponse = response.json().await?;
    //     println!(
    //         "Response \n  quote:{} \n sever_signature:{}",
    //         response_data.quote, response_data.server_signature
    //     );
    //     let data_to_sign = serde_json::to_string(&response_data)?;
    //     let quote_json = serde_json::to_string(&data_to_sign).unwrap();
    //     let quote_bytes = quote_json.as_bytes();
        
    //     //3.Client signs the data with stark_curve
    //     let mut rng = OsRng; 
    //     let mut mock_account = MockAccount::new(&mut rng);
    //     let client_signature = mock_account.sign_message(&quote_bytes, &mut rng);

    //     //formating client signature
    //     let client_signature = match client_signature {
    //         Ok(signature) => {
    //             let signature_json = format!(
    //                 "{{\"r\": \"{}\", \"s\": \"{}\"}}",
    //                 scalar_to_hex(&signature.r),
    //                 scalar_to_hex(&signature.s)
    //             );
    //             println!("Serialized Signature: {}", signature_json);
    //             signature_json
    //         }
    //         Err(e) => {
    //             println!("Failed to sign message: {}", e);
    //             return Err(e.into());
    //         }
    //     };

    //     let request_quotation = AgreeToQuotation {
    //         quote: response_data.quote,
    //         server_signature: response_data.server_signature,
    //         client_signature: client_signature.to_string(),
    //     };
    //     //4. Accept the contract 
    //     let agree_to_quotatinon_response = client
    //         .post(&args.url_accept_contract)
    //         .header(reqwest::header::CONTENT_TYPE, "application/json")
    //         .json(&request_quotation)
    //         .send()
    //         .await?;

    //     if agree_to_quotatinon_response.status().is_success() {
    //         println!("Agreee to quotation successful!");
    //     } else {
    //         println!(
    //             "Agreee to quotation  failed with status: {}",
    //             agree_to_quotatinon_response.status()
    //         );
    //     }
    // } else {
    //     println!("Request failed with status: {}", response.status());
    // }

    Ok(())
}

// Convert byte array to hexadecimal string
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

// Obtain bytes from hexadecimal string
fn hex_string_to_bytes(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_str)
}



async fn proof(jwt_token:&String, data: Value, url: &str) -> Result<String, ValidatorErrors> {
    let client = Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, format!("Bearer {}", jwt_token).parse().unwrap());
    headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());

    let response = client
        .post(url)
        .headers(headers)
        .json(&data)
        .send()
        .await?;

    let response_data = response.text().await?;
    println!("Making a request to: {}", url);
    println!("{}", response_data);

    Ok(response_data)
}


async fn authenticate_with_prover(signing_key:&SigningKey, public_key: &VerifyingKey, url: &String) -> Result<String, ValidatorErrors> {
    // (Prover authentication) call get nonce
    let nonce = get_nonce(&public_key,url).await?;
    println!("Got Nonce: {}", nonce);

    // Sign the nonce 
    let signed_nonce = signing_key.sign(nonce.as_bytes()); //slice string to bytes
    
    // Validate our signature and get Jwt_token
    let jwt_token = validate_signature(&public_key, &nonce, &signed_nonce,url).await?;

    Ok(jwt_token)
}


async fn get_nonce(public_key: &VerifyingKey,url: &String) -> Result<String, ValidatorErrors> {
    print!("Get nonce called");
    let client = Client::new();
    let url_with_params = format!("{}?public_key={}", url, bytes_to_hex_string(public_key.as_bytes()));

    let response = match client.get(&url_with_params).send().await {
        Ok(response) => response,
        Err(err) => return Err(ValidatorErrors::RequestFailed(err)),
    };

    let response_text = match response.text().await {
        Ok(text) => text,
        Err(err) => return Err(ValidatorErrors::RequestFailed(err)),
    };
    println!("Response body: {}", response_text);
    let json_body: Value = match serde_json::from_str(&response_text) {
        Ok(json) => json,
        Err(err) => return Err(ValidatorErrors::JsonParsingFailed),
    };
    let nonce = match json_body["nonce"].as_str() {
            Some(nonce) => nonce.to_string(),
            None => return Err(ValidatorErrors::NonceNotFound), 
        };
    Ok(nonce)
}

async fn validate_signature(public_key:&VerifyingKey,nonce:&String, signed_nonce: &Signature,url: &String) -> Result<String, ValidatorErrors> {
    print!("Validate_signature called");
    let client = Client::new();

    // Create JSON object with public key, nonce, and signature
    let data = json!({
        "public_key": bytes_to_hex_string(&public_key.to_bytes()),
        "nonce": nonce,
        "signature":bytes_to_hex_string(&signed_nonce.to_bytes()),
    });

    // Send Post request to validation
    let response = client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await?;

    let json_body: Value = match response.json().await {
        Ok(json) => json,
        Err(_e) => return Err(ValidatorErrors::JsonParsingFailed),
    };
    let jwt_token = match json_body["jwt_token"].as_str() {
            Some(nonce) => nonce.to_string(),
            None => return Err(ValidatorErrors::NonceNotFound), 
        };
    Ok(jwt_token)
}