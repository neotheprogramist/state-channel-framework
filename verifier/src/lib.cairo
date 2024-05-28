mod component;

use cairo_verifier::StarkProofWithSerde;
use starknet::ContractAddress;

#[starknet::interface]
trait IFactRegistry<TContractState> {
    fn verify_and_register_fact(ref self: TContractState, stark_proof: StarkProofWithSerde);
    fn is_valid(self: @TContractState, fact: felt252) -> bool;
}

#[starknet::interface]
trait ISmartProof<TContractState> {
    fn get_proof(self: @TContractState) -> Array<felt252>;
}

#[starknet::contract]
mod FactRegistry {
    use cairo_verifier::StarkProofWithSerde;
    use starknet::ContractAddress;
    use core::{
        poseidon::{Poseidon, PoseidonImpl, HashStateImpl},
        starknet::event::EventEmitter
    };
    use verifier::{component::{CairoVerifier, ICairoVerifier, StarkProof}, IFactRegistry,};
    use super::{ISmartProofDispatcher, ISmartProofDispatcherTrait};

    component!(path: CairoVerifier, storage: cairo_verifier, event: CairoVerifierEvent);

    #[storage]
    struct Storage {
        #[substorage(v0)]
        cairo_verifier: CairoVerifier::Storage,
        facts: LegacyMap<felt252, bool>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        CairoVerifierEvent: CairoVerifier::Event,
        FactRegistered: FactRegistered,
    }

    #[derive(Drop, starknet::Event)]
    struct FactRegistered {
        #[key]
        fact: felt252,
    }

    #[abi(embed_v0)]
    impl FactRegistryImpl of IFactRegistry<ContractState> {
        fn verify_and_register_fact(ref self: ContractState, stark_proof: StarkProofWithSerde) {
            let (program_hash, output_hash) = self.cairo_verifier.verify_proof(stark_proof.into());
            self._register_fact(program_hash, output_hash);
        }

        fn is_valid(self: @ContractState, fact: felt252) -> bool {
            self.facts.read(fact)
        }
    }

    #[generate_trait]
    impl InternalFactRegistry of InternalFactRegistryTrait {
        fn _register_fact(ref self: ContractState, program_hash: felt252, output_hash: felt252,) {
            let fact = PoseidonImpl::new().update(program_hash).update(output_hash).finalize();
            self.emit(Event::FactRegistered(FactRegistered { fact }));
            self.facts.write(fact, true);
        }
    }
}
