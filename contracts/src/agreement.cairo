use starknet::ContractAddress;

#[derive(Drop, Serde)]
struct ProgramOutput {
    client_public_key: felt252,
    server_public_key: felt252,
    a: felt252,
    b: felt252,
}

#[starknet::interface]
pub trait IAgreement<TContractState> {
    fn get_program_hash(self: @TContractState) -> felt252;
    fn get_facts_registry_address(self: @TContractState) -> ContractAddress;
    fn get_herodotus_facts_registry_address(self: @TContractState) -> ContractAddress;
    fn get_client_public_key(self: @TContractState) -> felt252;
    fn get_server_public_key(self: @TContractState) -> felt252;
    fn get_collateral_token_address(self: @TContractState) -> ContractAddress;
    fn get_client_amount(self: @TContractState) -> u256;
    fn get_account(self: @TContractState) -> felt252;
    fn get_block(self: @TContractState) -> u256;
    fn get_slot(self: @TContractState) -> u256;
    fn settle(ref self: TContractState, program_output: ProgramOutput) -> u256;
}

mod errors {
    pub const CLIENT_PUBLIC_KEY_NOT_MATCH: felt252 = 'client public key not match';
    pub const SERVER_PUBLIC_KEY_NOT_MATCH: felt252 = 'server public key not match';
    pub const INVALID_PROOF: felt252 = 'invalid proof';
    pub const NO_STORAGE_PROOF: felt252 = 'no storage proof';
}

#[starknet::contract]
mod Agreement {
    use core::option::OptionTrait;
    use core::traits::Into;
    use contracts::{
        agreement::{ProgramOutput, errors}, erc20::{IErc20Dispatcher, IErc20DispatcherTrait},
        facts_registry::{IFactsRegistryDispatcher, IFactsRegistryDispatcherTrait},
        herodotus::{IEVMFactsRegistryDispatcher, IEVMFactsRegistryDispatcherTrait},
    };
    use starknet::{ContractAddress, contract_address_const, get_contract_address};
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
        account: felt252,
        block: u256,
        slot: u256,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        client_public_key: felt252,
        server_public_key: felt252,
        collateral_token_address: ContractAddress,
        client_amount: u256,
        account: felt252,
        block: u256,
        slot: u256,
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
                    0x008ec19768587f3d7362d83b644253de560c9d46eda096619df5942d56079abb
                >()
            );
        self.client_public_key.write(client_public_key);
        self.server_public_key.write(server_public_key);
        self.collateral_token_address.write(collateral_token_address);
        self.client_amount.write(client_amount);
        self.account.write(account);
        self.block.write(block);
        self.slot.write(slot);
    }

    #[abi(embed_v0)]
    impl AgreementImpl of super::IAgreement<ContractState> {
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
        fn get_account(self: @ContractState) -> felt252 {
            self.account.read()
        }
        fn get_block(self: @ContractState) -> u256 {
            self.block.read()
        }
        fn get_slot(self: @ContractState) -> u256 {
            self.slot.read()
        }
        fn settle(ref self: ContractState, program_output: ProgramOutput) -> u256 {
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
            let facts_registry = IFactsRegistryDispatcher {
                contract_address: self.fact_registry_address.read()
            };
            assert(facts_registry.is_valid(fact), errors::INVALID_PROOF);

            let herodotus_facts_registry = IEVMFactsRegistryDispatcher {
                contract_address: self.herodotus_fact_registry_address.read()
            };
            let value = herodotus_facts_registry
                .get_slot_value(self.account.read(), self.block.read(), self.slot.read());
            assert(value.is_some(), errors::NO_STORAGE_PROOF);
            let value = value.unwrap();

            let reserve0 = value % 5192296858534827628530496329220096;
            let reserve1 = (value
                / 5192296858534827628530496329220096) % 5192296858534827628530496329220096;
            let result = reserve0 * program_output.a.into() / reserve1 + program_output.b.into();

            let token = IErc20Dispatcher { contract_address: self.collateral_token_address.read() };
            token.transfer(self.collateral_token_address.read(), result);
            token
                .transfer(
                    self.collateral_token_address.read(), token.balance_of(get_contract_address())
                );
            result
        }
    }
}
