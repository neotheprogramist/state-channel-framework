use starknet::contract_address_const;
use contracts::{
    agreement::{IAgreementDispatcher, IAgreementDispatcherTrait},
    erc20::{IErc20Dispatcher, IErc20DispatcherTrait},
};
use tests::utils::{DEFAULT_ACCOUNT, deploy_contract};

#[test]
fn test_udc_deploy() {
    let token_contract = IErc20Dispatcher {
        contract_address: deploy_contract(
            "Erc20", array![DEFAULT_ACCOUNT, DEFAULT_ACCOUNT, DEFAULT_ACCOUNT, DEFAULT_ACCOUNT]
        )
    };
    let agreement_contract = IAgreementDispatcher {
        contract_address: deploy_contract(
            "Agreement", array![37, 42, token_contract.contract_address.into(), 0, 0, 0, 0, 0, 0, 0]
        )
    };

    assert(
        agreement_contract
            .get_program_hash() == 0x1e0a9aedb642a67097df9114a992054d577d7759f558d2499c55f35beebf390,
        'invalid program hash'
    );
    assert(
        agreement_contract
            .get_facts_registry_address() == contract_address_const::<
                0x679bd7ba29abf0c708f2ddcc321aab97e26f70ccb85a7ce92c289d9dfedac0c
            >(),
        'invalid fact registry address'
    );
    assert(agreement_contract.get_client_public_key() == 37, 'invalid client public key');
    assert(agreement_contract.get_server_public_key() == 42, 'invalid server public key');
    assert(
        agreement_contract.get_collateral_token_address() == token_contract.contract_address,
        'invalid token address'
    );
    assert(agreement_contract.get_client_amount() == 0, 'invalid client amount');
    assert(agreement_contract.get_account() == 0, 'invalid account');
    assert(agreement_contract.get_block() == 0, 'invalid block');
    assert(agreement_contract.get_slot() == 0, 'invalid slot');
}
