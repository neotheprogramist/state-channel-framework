from starkware.cairo.common.builtin_poseidon.poseidon import poseidon_hash_many
from starkware.cairo.stark_verifier.core.serialize_utils import append_felt, append_felts
from starkware.cairo.common.signature import (
    verify_ecdsa_signature,
)
from input import Agreement

func aggregate(
    client_public_key: felt,
    server_public_key: felt,
    agreements_len: felt,
    agreements: Agreement**,
    a: felt,
    b: felt
) -> (a: felt, b: felt) {
    if (agreements_len == 0) {
        return (a=a, b=b);
    }

    // let (data: felt*) = alloc();
    // append_felt{data=data}(agreements[0].price);
    // append_felt{data=data}(agreements[0].nonce);
    // append_felt{data=data}(agreements[0].quantity);
    // append_felt{data=data}(client_public_key);
    // let data_start = data;
    // let (hash) = poseidon_hash_many(n=4, elements=data_start);
    // verify_ecdsa_signature(
    //     message = agreement_hash,
    //     public_key = server_public_key,
    //     signature_r = agreements[0].server_signature_r,
    //     signature_s = agreements[0].server_signature_s,
    // );

    return aggregate(
        client_public_key,
        server_public_key,
        agreements_len - 1,
        agreements + 1,
        a + agreements[0].quantity,
        b - agreements[0].quantity * agreements[0].price
    );
}
