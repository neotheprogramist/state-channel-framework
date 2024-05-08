use prover_sdk::ProverSDK;
use reqwest::Error as ReqwestError;
use thiserror::Error;
use tokio::fs::File;
mod models;
mod prover_sdk;
use serde_json::Value;
use tokio::io::AsyncReadExt;
use url::ParseError;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     #[arg(long,default_value_t="Sample")]
//     input_file: String,

//     #[arg(long,default_value_t = "sample")]
//     signing_key: String,
// }

#[derive(Debug, Error)]
enum ProverSdkErrors {
    #[error("HTTP request failed")]
    RequestFailed(#[from] ReqwestError),

    #[error("JSON parsing failed")]
    JsonParsingFailed,

    #[error("Failed to serialize")]
    SerdeError(#[from] serde_json::Error),

    #[error("Nonce not found in the response")]
    NonceNotFound,

    #[error("JWT token not found in the response")]
    JwtTokenNotFound,

    #[error("Reading input file failed")]
    ReadFileError(#[from] std::io::Error),

    #[error("Expiration date not found")]
    ExpirationNotFound,

    #[error("Signing key not found")]
    SigningKeyNotFound,
    #[error("Failed to parse to url")]
    UrlParseError(#[from] ParseError),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let args: Args = Args::parse();

    // Authentication with the prover
    let private_key_hex = "f91350db1ca372b54376b519be8bf73a7bbbbefc4ffe169797bc3f5ea2dec740";

    let sdk = ProverSDK::new().auth(private_key_hex).await?.build()?;

    let data = read_json_file("resources/input.json").await?;
    let result = sdk.prove(data).await?;

    Ok(())
}

async fn read_json_file(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path).await?;

    let mut json_string = String::new();
    file.read_to_string(&mut json_string).await?;

    let json_value: Value = serde_json::from_str(&json_string)?;

    Ok(json_value)
}
