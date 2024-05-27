use starknet::ContractAddress;

#[derive(Drop, Serde)]
struct ProgramOutput {
    client_public_key: felt252,
    server_public_key: felt252,
    a: felt252,
    b: felt252,
}

#[starknet::interface]
pub trait IVerifierApplier<TContractState> {
    fn get_program_hash(self: @TContractState) -> felt252;
    fn get_facts_registry_address(self: @TContractState) -> ContractAddress;
    fn get_herodotus_facts_registry_address(self: @TContractState) -> ContractAddress;
    fn get_client_public_key(self: @TContractState) -> felt252;
    fn get_server_public_key(self: @TContractState) -> felt252;
    fn get_collateral_token_address(self: @TContractState) -> ContractAddress;
    fn get_client_amount(self: @TContractState) -> u256;
    fn settle(ref self: TContractState, program_output: ProgramOutput) -> felt252;
}

mod errors {
    pub const CLIENT_PUBLIC_KEY_NOT_MATCH: felt252 = 'client public key not match';
    pub const SERVER_PUBLIC_KEY_NOT_MATCH: felt252 = 'server public key not match';
    pub const INVALID_PROOF: felt252 = 'invalid proof';
}

#[starknet::contract]
mod VerifierApplier {
    use verifier_applier::{
        verifier_applier::{ProgramOutput, errors},
        facts_registry::{IFactsRegistryDispatcher, IFactsRegistryDispatcherTrait},
    };
    use starknet::{ContractAddress, contract_address_const};
    use core::poseidon::poseidon_hash_span;

    #[storage]
    struct Storage {
        program_hash: felt252,
        fact_registry_address: ContractAddress,
        herodotus_fact_registry_address: ContractAddress,
        client_public_key: felt252,
        server_public_key: felt252,
        collateral_token_address: ContractAddress,
        client_amount: u256,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        client_public_key: felt252,
        server_public_key: felt252,
        collateral_token_address: ContractAddress,
        client_amount: u256
    ) {
        self.program_hash.write(0x1e0a9aedb642a67097df9114a992054d577d7759f558d2499c55f35beebf390);
        self
            .fact_registry_address
            .write(
                contract_address_const::<
                    0x679bd7ba29abf0c708f2ddcc321aab97e26f70ccb85a7ce92c289d9dfedac0c
                >()
            );
        self
            .herodotus_fact_registry_address
            .write(
                contract_address_const::<
                    0x07d3550237ecf2d6ddef9b78e59b38647ee511467fe000ce276f245a006b40bc
                >()
            );
        self.client_public_key.write(client_public_key);
        self.server_public_key.write(server_public_key);
        self.collateral_token_address.write(collateral_token_address);
        self.client_amount.write(client_amount);
    }

    #[abi(embed_v0)]
    impl VerifierApplierImpl of super::IVerifierApplier<ContractState> {
        fn get_program_hash(self: @ContractState) -> felt252 {
            self.program_hash.read()
        }
        fn get_facts_registry_address(self: @ContractState) -> ContractAddress {
            self.fact_registry_address.read()
        }
        fn get_herodotus_facts_registry_address(self: @ContractState) -> ContractAddress {
            self.herodotus_fact_registry_address.read()
        }
        fn get_client_public_key(self: @ContractState) -> felt252 {
            self.client_public_key.read()
        }
        fn get_server_public_key(self: @ContractState) -> felt252 {
            self.server_public_key.read()
        }
        fn get_collateral_token_address(self: @ContractState) -> ContractAddress {
            self.collateral_token_address.read()
        }
        fn get_client_amount(self: @ContractState) -> u256 {
            self.client_amount.read()
        }
        fn settle(ref self: ContractState, program_output: ProgramOutput) -> felt252 {
            assert(
                program_output.client_public_key == self.client_public_key.read(),
                errors::CLIENT_PUBLIC_KEY_NOT_MATCH
            );
            assert(
                program_output.server_public_key == self.server_public_key.read(),
                errors::SERVER_PUBLIC_KEY_NOT_MATCH
            );

            let mut program_output_array = array![];
            program_output.serialize(ref program_output_array);
            let program_output_hash = poseidon_hash_span(program_output_array.span());

            let fact = poseidon_hash_span(
                array![self.program_hash.read(), program_output_hash].span()
            );
            let fact_registry = IFactsRegistryDispatcher {
                contract_address: self.fact_registry_address.read()
            };
            assert(fact_registry.is_valid(fact), errors::INVALID_PROOF);

            0
        }
    }
}
