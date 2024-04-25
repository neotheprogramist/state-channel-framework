
use bytes::{Bytes, BytesMut};
use rand::RngCore;
use serde_with::{serde_as, DisplayFromStr};
use std::{io, ops::Deref, str::FromStr};
use serde::{Deserialize, Serialize};
use ed25519_dalek::Signature; // Ensure these are properly imported

#[derive(Debug, Clone)]
pub struct Nonce(Bytes);

impl Nonce {
    pub fn new(size: usize) -> Self {
        let mut bytes = BytesMut::zeroed(size);
        rand::thread_rng().fill_bytes(bytes.as_mut());
        Self(bytes.into())
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

impl Deref for Nonce {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message(String);
impl From<Nonce> for Message {
    fn from(value: Nonce) -> Self {
        Self(format!(
            "Confirm identity by signing random data:\n{}",
            value,
        ))
    }
}

impl Deref for Message {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Message {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateNonceRequest {
    pub public_key: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateNonceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub message: Message,
    pub expiration: usize,
}

// Define a struct for the query parameters
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicKeyQuery {
    pub public_key: String,
}
// Define a struct for the JSON body
#[derive(Debug,Serialize, Deserialize)]
pub struct ProgramInput {
    // Add fields here that match the JSON structure being sent from Python
    // Example:
    value: i32,
}


#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateSignatureRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub signature: Signature,
    pub public_key: String,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub session_id: SessionId,
    pub expiration: usize,
}

#[derive(Debug, Clone)]
pub struct SessionId(Bytes);
impl SessionId {
    pub fn new(size: usize) -> Self {
        let mut bytes = BytesMut::zeroed(size);
        rand::thread_rng().fill_bytes(bytes.as_mut());
        Self(bytes.into())
    }
}

impl FromStr for SessionId {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            prefix_hex::decode::<Vec<u8>>(s)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
                .into(),
        ))
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.to_vec()))
    }
}