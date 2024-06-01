use crate::server::ServerError;
use starknet::core::crypto::compute_hash_on_elements;
use starknet::core::types::FieldElement;
use starknet::signers::{SigningKey, VerifyingKey};
use utils::models::Quote;
/// Helper function to convert a `stark_curve::Scalar` to a hexadecimal string.
pub fn scalar_to_hex(bytes: &[u8]) -> String {
    prefix_hex::encode(bytes.to_vec())
}

#[derive(Debug, Clone)]
pub struct SeverSignature {
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
}

#[derive(Debug, Clone)]
pub struct MockAccount {
    pub secret_key: SigningKey,
}

impl Default for MockAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl MockAccount {
    pub fn new() -> Self {
        let secret_key = SigningKey::from_random();

        Self { secret_key }
    }
    pub fn public_key(&self) -> VerifyingKey {
        self.secret_key.verifying_key()
    }
    pub fn sign_message(&self, quote: Quote) -> Result<(String, String), ServerError> {
        let price = FieldElement::from_dec_str(&quote.price.to_string())?;
        let address = FieldElement::from_hex_be(&quote.address)?;
        let quantity_hex = format!("{:x}", &quote.quantity);
        let quantity = FieldElement::from_hex_be(&quantity_hex)?;
        let nonce = FieldElement::from_hex_be(&quote.nonce.to_string())?;

        let data = [quote.price, quote.nonce, quote.quantity, quote.address];
        let hash = compute_hash_on_elements(&data);
        tracing::info!("HASHING {}", hash);
        let signature: starknet::core::crypto::Signature = self.secret_key.sign(&hash).unwrap();

        Ok((signature.r, signature.s))
    }
}
