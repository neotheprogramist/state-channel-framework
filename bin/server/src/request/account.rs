use crate::request::models::Quote;
use crate::server::ServerError;
use starknet::core::crypto::compute_hash_on_elements;
use starknet::core::types::FieldElement;
use starknet::signers::{SigningKey, VerifyingKey};

/// Helper function to convert a `stark_curve::Scalar` to a hexadecimal string.
pub fn scalar_to_hex(bytes: &[u8]) -> String {
    prefix_hex::encode(bytes.to_vec())
}

#[derive(Debug, Clone)]
pub struct SeverSignature {
    pub server_signature_r: String,
    pub server_signature_s: String,
}

#[derive(Debug, Clone)]
pub struct MockAccount {
    pub secret_key: SigningKey,
    pub public_key: VerifyingKey,
}

impl Default for MockAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl MockAccount {
    pub fn new() -> Self {
        let secret_key = SigningKey::from_random();
        let public_key = secret_key.verifying_key();

        MockAccount {
            secret_key,
            public_key,
        }
    }
    pub fn sign_message(&self, quote: Quote) -> Result<(String, String), ServerError> {
        let price = FieldElement::from_dec_str(&quote.price.to_string())?;
        let address = FieldElement::from_hex_be(&quote.address)?;
        let quantity_hex = format!("{:x}", &quote.quantity);
        let quantity = FieldElement::from_hex_be(&quantity_hex)?;
        let nonce = FieldElement::from_hex_be(&quote.nonce.to_string())?;

        let data = [price, nonce, quantity, address];
        let hash = compute_hash_on_elements(&data);

        let signature = self.secret_key.sign(&hash).unwrap();

        // Converting signature parts to hex string
        let r_hex = format!("0x{:x}", signature.r);
        let s_hex = format!("0x{:x}", signature.s);

        Ok((r_hex, s_hex))
    }
}
