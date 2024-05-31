use serde::Deserialize;
use serde::Serialize;
use starknet::core::types::FieldElement;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub address: FieldElement,
    pub quantity: FieldElement,
    pub nonce: FieldElement,
    pub price: FieldElement,
}
impl std::fmt::Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Address: {}, Quantity: {}, Nonce: {:?}, Price: {}",
            self.address, self.quantity, self.nonce, self.price
        )
    }
}
