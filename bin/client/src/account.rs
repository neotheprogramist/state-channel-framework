use ed25519_dalek::ed25519::signature::SignerMut;
use rand::rngs::OsRng;
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;

pub struct MockAccount{ 
    signing_key: SigningKey,
}
impl MockAccount {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        MockAccount { signing_key }
    }

    pub fn sign_message(& mut self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
}