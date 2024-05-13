use bytes::Buf;
use elliptic_curve::group::GroupEncoding;
use elliptic_curve::point::AffineCoordinates;
use elliptic_curve::Field;
use elliptic_curve::Group;
use elliptic_curve::PrimeField;
use rand_core::OsRng;
use rand_core::{CryptoRng, RngCore};
use sha2::{Digest, Sha256};
use secp256k1::{Secp256k1, Message};
use secp256k1::hashes::{sha256, Hash};
use secp256k1::{PublicKey,SecretKey};
use secp256k1::All;

/// Helper function to convert a `stark_curve::Scalar` to a hexadecimal string.
pub fn scalar_to_hex(bytes:&[u8]) -> String {
    prefix_hex::encode(bytes.to_vec())
}

#[derive(Debug, Clone)]
pub struct SeverSignature {
    pub server_signature_r: String,
    pub server_signature_s: String,
}


#[derive(Debug, Clone)]
pub struct MockAccount {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub secp:Secp256k1<All>
}

impl MockAccount {
    pub fn new<R>(rng: &mut R) -> Self
    where
        R: RngCore + CryptoRng,
    {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    
        MockAccount {
            secret_key,
            public_key,
            secp
        }
    }

    pub fn sign_message(&self, message: &[u8]) -> (String, String){
        let digest = sha256::Hash::hash(message);
        let message = Message::from_digest(digest.to_byte_array());
        let sig = self.secp.sign_ecdsa(&message, &self.secret_key);
        let compact_signature  = sig.serialize_compact();
        let r = &compact_signature[0..32];  // First 32 bytes
        let s = &compact_signature[32..64]; // Second 32 bytes
        let server_signature_r = scalar_to_hex(r);
        let server_signature_s = scalar_to_hex(s);
        (server_signature_r,server_signature_s)
    }
}
