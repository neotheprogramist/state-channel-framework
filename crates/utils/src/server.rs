use core::fmt;

use starknet::{
    core::{crypto::compute_hash_on_elements, types::FieldElement},
    signers::{SigningKey, VerifyingKey},
};

use crate::models::Quote;
pub struct Server {
    private_key: SigningKey,
}

impl Server {
    pub fn new() -> Self {
        let private_key = SigningKey::from_random();
        Server { private_key }
    }

    pub fn public_key(&self) -> VerifyingKey {
        self.private_key.verifying_key()
    }

    pub fn sign_message(&self, message: &[FieldElement]) -> (FieldElement, FieldElement) {
        let hash = compute_hash_on_elements(&message);
        let signature = self.private_key.sign(&hash).unwrap();
        (signature.r, signature.s)
    }

    pub fn sing_quote(&self, quote: Quote) -> (FieldElement, FieldElement) {
        let data = [
            quote.price,
            quote.nonce,
            quote.quantity,
            self.public_key().scalar(),
        ];
        let hash = compute_hash_on_elements(&data);
        let signature = self.private_key.sign(&hash).unwrap();
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
