

#[cfg(test)]
mod tests {
    use starknet::ContractAddress;

    use snforge_std::{declare, ContractClassTrait};
    
    use agreement_version_2::agreement_version_2::IAgreementVersion2Dispatcher;
    use agreement_version_2::agreement_version_2::IAgreementVersion2DispatcherTrait;
    use agreement_version_2::agreement_version_2::Agreement;
    
    // #[test]
    // fn test_balance(){
    //     let contract = declare("AgreementVersion2").unwrap();
        
    //     let client_public_key = (0x2bd287bb6b61ff3d8239ecedb9fc6e62 , 0xe774d3acaf4b3ca16bca06d33b241160);

    //     let client_public_key:felt252 =0x02c2b8abcc6df93eeaeabbc94b522d465128d457c9e4ad06b4f5bc1b5a1001e30b;
    //     let server_public_key:felt252 =0x0361e47966f6bd8be422f194bea135a95089cbb35a7b6f6d267455cb391705a474;

    //     let (contract_address, _) = contract.deploy(@array![10000000,10000000,client_public_key,server_public_key,0,0]).unwrap();

    //     let dispatcher = IAgreementVersion2Dispatcher{contract_address};

    //     let client_balance = dispatcher.get_client_balance();
    //     let expected_balance :u256 = 10000000;

    //     assert(client_balance == expected_balance, 'Invalid balance');
    //     let server_balance = dispatcher.get_server_balance();
    //     assert(server_balance == expected_balance, 'Invalid balance');
    // }   

    // #[test]
    // fn test_apply_agreement(){
    //     let contract = declare("AgreementVersion2").unwrap();
        
    //     let (contract_address, _) = contract.deploy(@array![10000000,10000000]).unwrap();

    //     let dispatcher = IAgreementVersion2Dispatcher{contract_address};
    //     let quantity:felt252 = 1;
    //     let nonce:felt252 = 0x2bd287bb6b61ff3d8239ecedb9fc6e62e774d3acaf4b3ca16bca06d33b241160;
    //     let price:felt252 = 1604;
    //     let server_signature_r:felt252=0x9413e24b39e26e5ceda98cc31cb731a718ac1c0d541b654f9f33142115c39003;
    //     let server_signature_s:felt252=0x505cf6311743d21af706f3a641b0449feb9a8a9b959e577bd0b35a2023f900c6;
    //     let client_signature_r:felt252=0x46f2f636d13cc4fc449cdff97467f6e35a30a84129c2e6c6b5344dcac0b52185;
    //     let client_signature_s:felt252=0x20ebf5f27a45b00d9715a43725fdde3ca33647194d215cc5b1231c5edb1b9d58;
        
    //     let agreement = Agreement{
    //         quantity:quantity,
    //         nonce:nonce,
    //         price:price,
    //         server_signature_r:server_signature_r,
    //         server_signature_s:server_signature_s ,
    //         client_signature_r:client_signature_r,
    //         client_signature_s:client_signature_s ,
    //     };

    //     dispatcher.apply(agreement);

    //     let agreement = dispatcher.get_agreement_by_id(0);
    //     assert(agreement == agreement, 'Invalid agreements');
    // }   
}