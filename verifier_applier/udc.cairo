use starknet::{ClassHash, ContractAddress};

#[starknet::interface]
pub trait IUniversalDeployer<TContractState> {
    fn deployContract(
        ref self: TContractState,
        class_hash: ClassHash,
        salt: felt252,
        unique: bool,
        calldata: Span<felt252>
    ) -> ContractAddress;
}
