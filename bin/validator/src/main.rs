use clap::Parser;
use ed25519_dalek::SigningKey;
use prover_sdk::ProverSDK;
use rand::rngs::OsRng;
use reqwest::Error as ReqwestError;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::time::Duration;
mod models;
mod prover_sdk;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = ("http://localhost:7003/auth".to_string()))]
    url_auth: String,

    #[arg(long, default_value_t = ("http://localhost:7003/prove/state-diff-commitment".to_string()))]
    url_prover: String,

    #[arg(long)]
    input_file: String,

    #[arg(long)]
    signing_key: String,

    #[arg(long)]
    public_key: String,
}

#[derive(Debug, Error)]
pub enum ValidatorErrors {
    #[error("HTTP request failed")]
    RequestFailed(#[from] ReqwestError),

    #[error("Failed to serialize")]
    SerdeError(#[from] serde_json::Error),

    #[error("JSON parsing failed")]
    JsonParsingFailed,

    #[error("Nonce not found in the response")]
    NonceNotFound,

    #[error("JWT token not found in the response")]
    JwtTokenNotFound,

    #[error("Reading input file failed")]
    ReadFileError(#[from] std::io::Error),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let sdk = ProverSDK::new();

    // Authentication with the prover
    let signing_key = SigningKey::generate(&mut OsRng);
    let public_key = signing_key.verifying_key();

    //    let jwt_response = sdk.authenticate_with_prover(&args.signing_key, &args.public_key, &args.url_auth).await?;
    let jwt_response = sdk
        .authenticate_with_prover(&signing_key, &public_key, &args.url_auth)
        .await?;

    let jwt_token = &jwt_response.jwt_token;

    // Create a new client for proofing with cookies
    let cookie_client = ProverSDK::with_cookies(jwt_token)?;

    loop {
        let mut file = File::open(&args.input_file).await?;
        let mut data = String::new();
        file.read_to_string(&mut data).await?;

        cookie_client
            .proof(serde_json::from_str(&data)?, &args.url_prover)
            .await?;

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
    Ok(())
}
