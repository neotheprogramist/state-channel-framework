use elliptic_curve::point::AffineCoordinates;
use elliptic_curve::Field;
use elliptic_curve::Group;
use elliptic_curve::PrimeField;
use rand_core::{CryptoRng, RngCore};
use sha2::{Digest, Sha256};
use std::ops::Mul;

impl Signature {
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        let r_hex = scalar_to_hex(&self.r);
        let s_hex = scalar_to_hex(&self.s);
        let serialized = serde_json::json!({ "r": r_hex, "s": s_hex }).to_string();
        Ok(serialized)
    }
}
pub struct SigningKey {
    secret_scalar: stark_curve::Scalar,
}

// Verifying key structure
pub struct VerifyingKey {
    public_point: stark_curve::ProjectivePoint,
}

#[derive(Debug)]
pub struct Signature {
    pub r: stark_curve::Scalar,
    pub s: stark_curve::Scalar,
}
/// Helper function to convert a `stark_curve::Scalar` to a hexadecimal string.
pub fn scalar_to_hex(scalar: &stark_curve::Scalar) -> String {
    let bytes = scalar.to_repr();
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}
impl SigningKey {
    pub fn new(scalar: stark_curve::Scalar) -> Self {
        SigningKey {
            secret_scalar: scalar,
        }
    }

    pub fn to_verifying_key(&self) -> VerifyingKey {
        let public_point = stark_curve::ProjectivePoint::generator().mul(self.secret_scalar);
        VerifyingKey { public_point }
    }

    pub fn sign_message<R: RngCore + CryptoRng>(
        &self,
        message: &[u8],
        rng: &mut R,
    ) -> Result<Signature, &'static str> {
        // Step 1: Hash the message
        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash_result = hasher.finalize();
        let e = stark_curve::Scalar::from_be_bytes_mod_order(&hash_result);

        // Step 2 and 3: Generate k
        let k = stark_curve::Scalar::random(rng);

        // Step 4: Calculate the curve point (x1, y1) = k * G
        let k_g = stark_curve::ProjectivePoint::generator().mul(k);
        let affine = k_g.to_affine();
        let x = affine.x();
        let r = stark_curve::Scalar::from_be_bytes_mod_order(&x);

        // Step 5: Calculate r = x1 mod n
        if r.is_zero().into() {
            return Err("r is zero");
        }
        // Step 6: Calculate s = k^-1 (z + r*d) mod n
        let rd = r * self.secret_scalar;
        let z_plus_rd = e + rd;
        let k_inv = k.invert();
        if k_inv.is_none().unwrap_u8() == 1 {
            return Err("Failed to compute k^-1, possibly division by zero");
        }
        let k_inv = k_inv.unwrap();
        let s_not_mod = k_inv * z_plus_rd;

        //?????????????????????????????????????????????????????????????????????????
        let s = stark_curve::Scalar::new(*s_not_mod);
        if s.is_zero().into() {
            return Err("s is zero");
        }

        // Step 7: Return signature
        Ok(Signature { r, s })
    }
}

pub fn generate_keys<R>(rng: &mut R) -> (SigningKey, VerifyingKey)
where
    R: CryptoRng + RngCore,
{
    let scalar = stark_curve::Scalar::random(rng);
    let signing_key = SigningKey::new(scalar);
    let verifying_key = signing_key.to_verifying_key();
    (signing_key, verifying_key)
}

pub struct MockAccount {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl MockAccount {
    pub fn new<R>(rng: &mut R) -> Self
    where
        R: RngCore + CryptoRng,
    {
        let (signing_key, verifying_key) = generate_keys(rng);
        MockAccount {
            signing_key,
            verifying_key,
        }
    }

    pub fn sign_message<R: RngCore + CryptoRng>(
        &self,
        message: &[u8],
        rng: &mut R,
    ) -> Result<Signature, &'static str> {
        self.signing_key.sign_message(message, rng)
    }
}
