use regex::Regex;
use starknet::core::types::contract::ComputeClassHashError;
use starknet::core::types::FieldElement;
use starknet::core::types::FromStrError;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("failed to parse url")]
    ParsingError(#[from] ParseError),

    #[error("FromStrError error: {0}")]
    FromStrError(#[from] FromStrError),

    #[error("SerdeJsonError error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("ReadFileError error: {0}")]
    ReadFileError(#[from] std::io::Error),

    #[error("JsonError error: {0}")]
    JsonError(#[from] starknet::core::types::contract::JsonError),

    #[error("ClassHashError error: {0}")]
    ClassHashError(#[from] ComputeClassHashError),

    #[error("Account error: {0}")]
    AccountFailure(String),

    #[error("Deployment error: {0}")]
    DeploymentFailure(String),

    #[error("Box error: {0}")]
    BoxError(#[from] Box<dyn std::error::Error>),
}

pub fn parse_class_hash_from_error(error_msg: &str) -> FieldElement {
    println!("Error message: {}", error_msg);
    let re = Regex::new(r#"StarkFelt\("(0x[a-fA-F0-9]+)"\)"#).unwrap();

    // Attempt to capture the class hash
    if let Some(captures) = re.captures(error_msg) {
        if let Some(contract_address) = captures.get(1) {
            return FieldElement::from_hex_be(contract_address.as_str())
                .expect("Failed to parse class hash");
        }
    }

    panic!("Failed to extract class hash from error message");
}
