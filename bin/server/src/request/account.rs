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
    pub fn sign_message(&self, quote: Quote) -> Result<(FieldElement, FieldElement), ServerError> {
        tracing::info!(
            "DATA {} {:x} {} {:x}",
            quote.price,
            quote.nonce,
            quote.quantity,
            quote.address
        );

        let data = [quote.price, quote.nonce, quote.quantity, quote.address];
        let hash = compute_hash_on_elements(&data);
        tracing::info!("HASHING {}", hash);
        let signature: starknet::core::crypto::Signature = self.secret_key.sign(&hash).unwrap();

        Ok((signature.r, signature.s))
    }
}
