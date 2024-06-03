use starknet::core::types::FieldElement;
/// Helper function to convert a `stark_curve::Scalar` to a hexadecimal string.
pub fn scalar_to_hex(bytes: &[u8]) -> String {
    prefix_hex::encode(bytes.to_vec())
}

#[derive(Debug, Clone)]
pub struct SeverSignature {
    pub server_signature_r: FieldElement,
    pub server_signature_s: FieldElement,
}
