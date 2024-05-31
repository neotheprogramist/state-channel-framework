use core::fmt;

use crate::models::Quote;
use starknet::{
    core::{crypto::compute_hash_on_elements, types::FieldElement},
    signers::{SigningKey, VerifyingKey},
};
use starknet_crypto::poseidon_hash_many;
pub struct Server {
    private_key: SigningKey,
}

use starknet::core::types::FieldElement as StarknetFieldElement;
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

impl Server {
    pub fn new() -> Self {
        let private_key = SigningKey::from_random();
        Server { private_key }
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
        // Print converted data
        for (i, element) in data.iter().enumerate() {
            println!("Data[{:x}]: {:?}", i, element);
        }

        // Hash the data
        let hash = poseidon_hash_many(&data);
        println!("Hash: {:x?}", hash);

        let signature = self.private_key.sign(&crypto_to_starknet(&hash)).unwrap();
        (signature.r, signature.s)
    }
}
impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Server")
            .field("private_key", &"Hidden for security")
            .finish()
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        let cloned_key = self.private_key.clone();
        Server {
            private_key: cloned_key,
        }
    }
}
