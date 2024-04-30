use bytes::{Bytes, BytesMut};
use rand::RngCore;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::serde_as;
use std::ops::Deref;
use std::{io, str::FromStr};
impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Address: {}, Quantity: {}, Nonce: {:?}, Price: {}",
            self.address, self.quantity, self.nonce, self.price
        )
    }
}

//TODO: is signature string ?
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestAcceptContract {
    pub quote: Quote,
    pub server_signature: String,
    pub client_signature: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotation {
    pub address: String,
    pub quantity: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub address: String,
    pub quantity: u64,
    pub nonce: Nonce,
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuotationResponse {
    pub quote: Quote,
    pub server_signature: String,
}

#[derive(Debug, Clone)]
pub struct Nonce(Bytes);

impl Nonce {
    pub fn new(size: usize) -> Self {
        let mut bytes = BytesMut::zeroed(size);
        rand::thread_rng().fill_bytes(bytes.as_mut());
        Self(bytes.into())
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
