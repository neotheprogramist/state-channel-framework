#[starknet::interface]
pub trait IFactsRegistry<TContractState> {
    fn is_valid(self: @TContractState, fact: felt252) -> bool;
}
