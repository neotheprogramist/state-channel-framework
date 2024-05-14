#[starknet::interface]
pub trait IEVMFactsRegistry<TContractState> {
    // @notice Returns a proven storage slot value
    // @param account: The account to query
    // @param block: The block number
    // @param slot: The slot to query
    // @return The value of the slot, if the slot is not proven, returns None
    fn get_slot_value(
        self: @TContractState, account: felt252, block: u256, slot: u256
    ) -> Option<u256>;
}
