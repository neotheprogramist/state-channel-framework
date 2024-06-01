use core::fmt;

use starknet::core::types::FieldElement as StarknetFieldElement;
use starknet::{
    core::{types::FieldElement},
    signers::{SigningKey, VerifyingKey},
};
use starknet_crypto::poseidon_hash_many;
use starknet_crypto::FieldElement as CryptoFieldElement;

pub fn crypto_to_starknet(fe: &CryptoFieldElement) -> StarknetFieldElement {
    let bytes = fe.to_bytes_be();
    StarknetFieldElement::from_bytes_be(&bytes).unwrap()
}

pub fn starknet_to_crypto(fe: &StarknetFieldElement) -> CryptoFieldElement {
    // Similar conversion assuming a method to get bytes
    let bytes = fe.to_bytes_be(); // Hypothetical method to get bytes
    CryptoFieldElement::from_bytes_be(&bytes).unwrap()
}
use crate::models::Quote;
pub struct Client {
    private_key: SigningKey,
}

impl Client {
    pub fn new() -> Self {
        let private_key = SigningKey::from_random();
        Client { private_key }
    }

    pub fn public_key(&self) -> VerifyingKey {
        self.private_key.verifying_key()
    }

    pub fn sing_quote(&self, quote: Quote) -> (FieldElement, FieldElement) {
        let data = [
            starknet_to_crypto(&self.public_key().scalar()),
            starknet_to_crypto(&quote.quantity),
            starknet_to_crypto(&quote.nonce),
            starknet_to_crypto(&quote.price),
        ];
        let hash = poseidon_hash_many(&data);

        let signature = self.private_key.sign(&crypto_to_starknet(&hash)).unwrap();
        (signature.r, signature.s)
    }
}
impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("private_key", &"Hidden for security")
            .finish()
    }
}
impl Clone for Client {
    fn clone(&self) -> Self {
        let cloned_key = self.private_key.clone();
        Client {
            private_key: cloned_key,
        }
    }
}
