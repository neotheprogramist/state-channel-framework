from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.builtin_poseidon.poseidon import poseidon_hash_many
from starkware.cairo.common.cairo_builtins import HashBuiltin, PoseidonBuiltin, EcOpBuiltin, SignatureBuiltin
from starkware.cairo.common.signature import (
    verify_ecdsa_signature,
)
from input import Agreement

func aggregate{
    range_check_ptr,
    ecdsa_ptr: SignatureBuiltin*,
    bitwise_ptr,
    ec_op_ptr: EcOpBuiltin*,
    poseidon_ptr: PoseidonBuiltin*,
}(
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

    tempvar elements: felt* = new (agreements[0].price, agreements[0].nonce, agreements[0].quantity, client_public_key);
    let (hash) = poseidon_hash_many(n=4, elements=elements);
    // verify_ecdsa_signature(
    //     hash,
    //     server_public_key,
    //     agreements[0].server_signature_r,
    //     agreements[0].server_signature_s,
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
