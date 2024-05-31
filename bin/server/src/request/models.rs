use crate::server::ServerError;

use super::account::MockAccount;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet::core::types::FieldElement;
use starknet::signers::SigningKey;
use std::{io, str::FromStr};
use surrealdb::engine::local::Db;
use surrealdb::sql::Id;
use surrealdb::Surreal;

impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Address: {}, Quantity: {}, Nonce: {:?}, Price: {}",
            self.address, self.quantity, self.nonce, self.price
        )
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotation {
    pub address: FieldElement,
    pub quantity: FieldElement,
}
//TODO  :delete
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotationWithPrice {
    pub address: FieldElement,
    pub quantity: FieldElement,
    pub price: FieldElement,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestQuotationResponse {
    pub quote: Quote,
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub address: FieldElement,
    pub quantity: FieldElement,
    pub nonce: FieldElement,
    pub price: FieldElement,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgreeToQuotation {
    pub quote: Quote,
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
    pub client_signature_r: FieldElement,
    pub client_signature_s: FieldElement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementProofResponse {
    pub address: FieldElement,
    pub balance: FieldElement,
    pub diff: FieldElement,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettlementProofResponseWithData {
    pub contracts: Vec<Contract>,
    pub address: FieldElement,
    pub balance: FieldElement,
    pub diff: FieldElement,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSettlementProofRequest {
    pub address: FieldElement,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSettlementProofRequestWithPrice {
    pub address: FieldElement,
    pub price: FieldElement,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestAcceptContract {
    pub quote: Quote,
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
    pub client_signature_r: FieldElement,
    pub client_signature_s: FieldElement,
}

#[derive(Debug, Clone)]
pub struct Nonce(FieldElement);

impl Nonce {
    pub fn new() -> Result<Self, ServerError> {
        let secret_key = SigningKey::from_random();
        let bytes_repr: [u8; 32] = secret_key.secret_scalar().to_bytes_be();
        let nonce = FieldElement::from_byte_slice_be(&bytes_repr)?;
        Ok(Self(nonce))
    }
    // Returns a reference to the internal FieldElement
    pub fn as_field_element(&self) -> &FieldElement {
        &self.0
    }
}

impl FromStr for Nonce {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FieldElement::from_str(s).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e.to_string())
        })?))
    }
}

impl std::fmt::Display for Nonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thing {
    pub id: Id,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contract {
    pub id: Thing,
    pub address: FieldElement,
    pub quantity: FieldElement,
    pub nonce: FieldElement,
    pub price: FieldElement,
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
    pub client_signature_r: FieldElement,
    pub client_signature_s: FieldElement,
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " \n address :{} \n quantity :{} \n nonce :{} \n price :{} \n server_signature_r :{} \n server_signature_s :{} \n client_signature_r :{} \n client_signature_s :{} \n",
    self.address,self.quantity,self.nonce,self.price,self.server_signature_r,self.server_signature_s,self.client_signature_r,self.client_signature_s)
    }
}
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Surreal<Db>,
    pub mock_account: MockAccount,
}
