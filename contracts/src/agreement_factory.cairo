use starknet::ContractAddress;

#[derive(Drop, Serde)]
struct AgreementProposal {
    public_key: felt252,
    amount: u256,
}

#[derive(Drop, Serde)]
struct AgreementAcceptance {
    id: felt252,
    public_key: felt252,
    amount: u256,
}

#[derive(Drop, Serde, starknet::Store)]
struct ProposedAgreement {
    public_key: felt252,
    amount: u256,
}

#[derive(Drop, Serde, starknet::Store)]
struct AcceptedAgreement {
    client_public_key: felt252,
    server_public_key: felt252,
    client_amount: u256,
    total_amount: u256,
}

#[derive(Drop, Serde, starknet::Store)]
enum Agreement {
    NotFound: (),
    Proposed: ProposedAgreement,
    Accepted: AcceptedAgreement,
}

#[starknet::interface]
pub trait IAgreementFactory<TContractState> {
    fn propose(ref self: TContractState, proposal: AgreementProposal) -> felt252;
    fn accept(ref self: TContractState, acceptance: AgreementAcceptance) -> ContractAddress;
}

mod errors {
    pub const AGREEMENT_NOT_FOUND: felt252 = 'agreement not found';
    pub const AGREEMENT_ALREADY_ACCEPTED: felt252 = 'agreement already accepted';
}

#[starknet::contract]
mod AgreementFactory {
    use core::panic_with_felt252;
    use starknet::{ClassHash, ContractAddress, contract_address_const};

    use contracts::{
        agreement_factory::{
            Agreement, AgreementAcceptance, AgreementProposal, errors, ProposedAgreement
        },
        udc::{IUniversalDeployerDispatcher, IUniversalDeployerDispatcherTrait},
    };

    #[storage]
    struct Storage {
        udc_address: ContractAddress,
        collateral_token_address: ContractAddress,
        agreement_contract_class_hash: ClassHash,
        agreements_len: felt252,
        agreements: LegacyMap::<felt252, Agreement>,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        collateral_token_address: ContractAddress,
        agreement_contract_class_hash: ClassHash
    ) {
        self
            .udc_address
            .write(
                contract_address_const::<
                    0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf
                >()
            );
        self.collateral_token_address.write(collateral_token_address);
        self.agreement_contract_class_hash.write(agreement_contract_class_hash);
    }

    #[abi(embed_v0)]
    impl AgreementFactoryImpl of super::IAgreementFactory<ContractState> {
        fn propose(ref self: ContractState, proposal: AgreementProposal) -> felt252 {
            let agreement_id = self.agreements_len.read();
            self
                .agreements
                .write(
                    agreement_id,
                    Agreement::Proposed(
                        ProposedAgreement { public_key: proposal.public_key, amount: 0 }
                    )
                );
            self.agreements_len.write(agreement_id + 1);
            agreement_id
        }
        fn accept(ref self: ContractState, acceptance: AgreementAcceptance) -> ContractAddress {
            let agreement_proposal = self.agreements.read(acceptance.id);
            match agreement_proposal {
                Agreement::Proposed(proposal) => {
                    let udc = IUniversalDeployerDispatcher {
                        contract_address: self.udc_address.read()
                    };
                    udc
                        .deployContract(
                            self.agreement_contract_class_hash.read(),
                            0,
                            false,
                            array![proposal.public_key, acceptance.public_key].span()
                        )
                },
                Agreement::NotFound => panic_with_felt252(errors::AGREEMENT_NOT_FOUND),
                Agreement::Accepted(_) => panic_with_felt252(errors::AGREEMENT_ALREADY_ACCEPTED),
            }
        }
    }
}
