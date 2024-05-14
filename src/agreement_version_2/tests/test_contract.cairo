#[cfg(test)]
mod tests {
    use starknet::ContractAddress;

    use snforge_std::{declare, ContractClassTrait};

    use agreement_version_2::agreement_version_2::IAgreementVersion2Dispatcher;
    use agreement_version_2::agreement_version_2::IAgreementVersion2DispatcherTrait;
    use agreement_version_2::agreement_version_2::Agreement;

    #[test]
    fn test_balance() {
        let contract = declare("AgreementVersion2").unwrap();

        let client_public_key: felt252 =
            0xe5f5c0f64f7d753a3094d012a62d714f0fe3ca320df466cee03bf393d352f;
        let server_public_key: felt252 =
            0x70bf7cc40c6ea06a861742fa98c2a22e077672a1dd9ed2aa025ec2f8258a2e5;
    }

    #[test]
    fn test_apply_agreement() {
        let contract = declare("AgreementVersion2").unwrap();

        let (contract_address, _) = contract.deploy(@array![10000000, 10000000]).unwrap();

        let dispatcher = IAgreementVersion2Dispatcher { contract_address };
        let quantity: felt252 = 1;
        let nonce: felt252 = 0x01b868544003173ba975b7de48fadcb48065bd6aab2582d9e08714e8e42edc41;
        let price: felt252 = 1604;
        let server_signature_r: felt252 =
            0x5b52138ac22bc3ff92860dc3cc24247ed405724c243c7c9d3b2db72aa159cc5;
        let server_signature_s: felt252 =
            0x687f73155b96e2373d432bf3dc245eea1daeb1edce8a2c65f8af2f2a3f2a604;
        let client_signature_r: felt252 =
            0x6fdc481b6a9aa77bea31afd26bd104bf25fc6824ff04f0bc12cc0cba6dd292;
        let client_signature_s: felt252 =
        0x40c7a9291e466b2342eba3f18e917bab860f4fa060e9a7c6cc50933b9684331;

        let agreement = Agreement {
            quantity: quantity,
            nonce: nonce,
            price: price,
            server_signature_r: server_signature_r,
            server_signature_s: server_signature_s,
            client_signature_r: client_signature_r,
            client_signature_s: client_signature_s,
        };

        dispatcher.apply(agreement);

        let agreement = dispatcher.get_agreement_by_id(0);
        assert(agreement == agreement, 'Invalid agreements');
    }
}
