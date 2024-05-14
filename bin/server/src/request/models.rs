use super::account::MockAccount;
use crate::ServerError;
use bytes::{Bytes, BytesMut};
use elliptic_curve::Field;
use hex::ToHex;
use rand::RngCore;
use rand_core::OsRng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;
use starknet::core::types::FieldElement;
use starknet::signers::SigningKey;
use std::io::Read;
use std::ops::Deref;
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
    pub address: String,
    pub quantity: i64,
}
//TODO  :delete
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotationWithPrice {
    pub address: String,
    pub quantity: i64,
    pub price: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotationResponse {
    pub quote: Quote,
    pub server_signature_r: String,
    pub server_signature_s: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub address: String,
    pub quantity: i64,
    pub nonce: String,
    pub price: i64,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct AgreeToQuotation {
    pub quote: Quote,
    pub server_signature_r: String,
    pub server_signature_s: String,
    pub client_signature_r: String,
    pub client_signature_s: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementProofResponse {
    pub address: String,
    pub balance: f64,
    pub diff: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettlementProofResponseWithData {
    pub contracts: Vec<Contract>,
    pub address: String,
    pub balance: f64,
    pub diff: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSettlementProofRequest {
    pub address: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSettlementProofRequestWithPrice {
    pub address: String,
    pub price: i64,
}
//TODO: is signature string ?
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestAcceptContract {
    pub quote: Quote,
    pub server_signature_r: String,
    pub server_signature_s: String,
    pub client_signature_r: String,
    pub client_signature_s: String,
}

#[derive(Debug, Clone)]
pub struct Nonce(Bytes);

impl Nonce {
    pub fn new() -> Self {
        let secret_key = SigningKey::from_random();
        let bytes_repr: [u8; 32] = secret_key.secret_scalar().to_bytes_be(); // Assuming this returns [u8; 32]
        let bytes = Bytes::copy_from_slice(&bytes_repr); // Use the array directly

        Self(bytes)
    }
    pub fn new_field_element() -> Result<FieldElement, ServerError> {
        let secret_key = SigningKey::from_random();
        let bytes_repr: [u8; 32] = secret_key.secret_scalar().to_bytes_be(); // Assuming this returns [u8; 32]
        let bytes = Bytes::copy_from_slice(&bytes_repr); // Use the array directly
        let field_element = FieldElement::from_bytes_be(&bytes_repr)?;
        Ok(field_element)
    }
}

impl Deref for Nonce {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl FromStr for Nonce {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            prefix_hex::decode::<Vec<u8>>(s)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
                .into(),
        ))
    }
}

impl std::fmt::Display for Nonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.to_vec()))
    }
}
impl Serialize for Nonce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Nonce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Ok(Self(Bytes::from(bytes)))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thing {
    pub id: Id,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    pub id: Thing,
    pub address: String,
    pub quantity: i64,
    pub nonce: String,
    pub price: i64,
    pub server_signature_r: String,
    pub server_signature_s: String,
    pub client_signature_r: String,
    pub client_signature_s: String,
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
