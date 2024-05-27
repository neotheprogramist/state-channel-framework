use ed25519_dalek::{Signature, VerifyingKey};
/// Verifies a signature given a nonce and a public key using ed25519_dalek.
///
/// - `signature`: The signature object.
/// - `nonce`: The message that was signed, as a string.
/// - `public_key_hex`: The hexadecimal string of the public key.
///
/// Returns `true` if the signature is valid; `false` otherwise.
pub fn verify_signature(signature: &Signature, nonce: &str, public_key_hex: &str) -> bool {
    let public_key_bytes = match hex::decode(public_key_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Check if the decoded bytes are of the expected length
    if public_key_bytes.len() != 32 {
        return false;
    }

    // Convert the Vec<u8> to &[u8; 32]
    let public_key_array: &[u8; 32] = match public_key_bytes.as_slice().try_into() {
        Ok(arr) => arr,
        Err(_) => return false,
    };

    let public_key = match VerifyingKey::from_bytes(public_key_array) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    public_key
        .verify_strict(nonce.as_bytes(), signature)
        .is_ok()
}
