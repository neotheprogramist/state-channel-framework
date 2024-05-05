use starknet::{ContractAddress, contract_address_const};

use snforge_std::{declare, ContractClassTrait};

use state_channel_framework::udc::{IUniversalDeployerDispatcher, IUniversalDeployerDispatcherTrait};

pub const DEFAULT_ACCOUNT: felt252 = selector!("DEFAULT_ACCOUNT");

pub fn get_udc_address() -> ContractAddress {
    contract_address_const::<0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf>()
}

pub fn declare_and_deploy_via_udc(name: ByteArray, calldata: Array<felt252>) -> ContractAddress {
    let contract = declare(name).unwrap();
    let udc = IUniversalDeployerDispatcher { contract_address: get_udc_address() };
    udc.deployContract(contract.class_hash, 0, false, calldata.span())
}

pub fn deploy_contract(name: ByteArray, calldata: Array<felt252>) -> ContractAddress {
    let contract = declare(name).unwrap();
    let (contract_address, _) = contract.deploy(@calldata).unwrap();
    contract_address
}
