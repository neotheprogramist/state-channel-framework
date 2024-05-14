use starknet::ContractAddress;

#[derive(Drop, Serde, starknet::Store, PartialEq)]
pub struct Agreement {
    pub quantity: felt252,
    pub nonce: felt252,
    pub price: felt252,
    pub server_signature_r: felt252,
    pub server_signature_s: felt252,
    pub client_signature_r: felt252,
    pub client_signature_s: felt252,
}

#[starknet::interface]
pub trait IAgreementVersion2<TContractState> {
    fn apply(ref self: TContractState, agreement: Agreement,) -> Result<felt252, felt252>;
    fn result(self: @TContractState, x: u256) -> u256;
    fn get_client_balance(self: @TContractState) -> u256;
    fn get_server_balance(self: @TContractState) -> u256;
    fn get_agreement_by_id(self: @TContractState, id: felt252) -> Agreement;
    fn get_client_public_key(self: @TContractState) -> felt252;
    fn get_server_public_key(self: @TContractState) -> felt252;
}

#[starknet::contract]
mod AgreementVersion2 {
    use core::traits::Into;
    use core::ecdsa::check_ecdsa_signature;
    use core::poseidon::{PoseidonImpl, PoseidonTrait};
    use core::hash::HashStateTrait;
    use core::result::ResultTrait;

    use agreement_version_2::agreement_version_2::Agreement;

    #[storage]
    struct Storage {
        client_public_key: felt252,
        server_public_key: felt252,
        client_balance: u256,
        server_balance: u256,
        agreements_len: felt252,
        agreements: LegacyMap::<felt252, Agreement>,
        a: u256,
        b: u256
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        client_balance: felt252,
        server_balance: felt252,
        client_public_key: felt252,
        server_public_key: felt252,
        a: felt252,
        b: felt252
    ) {
        self.client_public_key.write(client_public_key);
        self.server_public_key.write(server_public_key);
        self.client_balance.write(client_balance.into());
        self.server_balance.write(server_balance.into());
        self.a.write(a.into());
        self.b.write(b.into());
    }

    #[abi(embed_v0)]
    impl AgreementVersion2Impl of super::IAgreementVersion2<ContractState> {
        fn apply(ref self: ContractState, agreement: Agreement) -> Result<felt252, felt252> {
            let agreement_hash = PoseidonImpl::new()
                .update(self.client_public_key.read())
                .update(agreement.quantity)
                .update(agreement.nonce)
                .update(agreement.price)
                .finalize();

            let valid_server_signature = check_ecdsa_signature(
                agreement_hash,
                self.server_public_key.read(),
                agreement.server_signature_r,
                agreement.server_signature_s
            );
            if !valid_server_signature {
                return Result::Err('Invalid server signature');
            }

            let curr_a = self.a.read() + agreement.quantity.into();
            self.a.write(curr_a);

            let curr_b = self.b.read() + agreement.quantity.into() * agreement.price.into();
            self.b.write(curr_b);

            let agreement_id = self.agreements_len.read();
            self.agreements.write(agreement_id, agreement);
            self.agreements_len.write(agreement_id + 1);

            Result::Ok(agreement_id)
        }

        fn result(self: @ContractState, x: u256) -> u256 {
            self.a.read() * x + self.b.read()
        }
        fn get_client_public_key(self: @ContractState) -> felt252 {
            self.client_public_key.read()
        }
        fn get_server_public_key(self: @ContractState) -> felt252 {
            self.server_public_key.read()
        }
        fn get_client_balance(self: @ContractState) -> u256 {
            self.client_balance.read()
        }
        fn get_server_balance(self: @ContractState) -> u256 {
            self.server_balance.read()
        }
        fn get_agreement_by_id(self: @ContractState, id: felt252) -> Agreement {
            self.agreements.read(id)
        }
    }
}
