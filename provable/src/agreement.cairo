#[derive(Drop, Serde)]
struct Input {
    pub client_public_key: felt252,
    pub server_public_key: felt252,
    pub agreements: Array<Agreement>,
    pub settlement_price: felt252,
}

#[derive(Drop, Serde)]
struct Agreement {
    pub quantity: felt252,
    pub nonce: felt252,
    pub price: felt252,
    pub server_signature_r: felt252,
    pub server_signature_s: felt252,
    pub client_signature_r: felt252,
    pub client_signature_s: felt252,
}
