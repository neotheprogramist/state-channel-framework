%builtins output pedersen range_check ecdsa bitwise ec_op poseidon

from starkware.cairo.common.cairo_builtins import HashBuiltin, PoseidonBuiltin, EcOpBuiltin, SignatureBuiltin
from starkware.cairo.common.builtin_poseidon.poseidon import poseidon_hash_many
from aggregate import aggregate
from input import (
    Input, get_agreements, Agreement
)


func main{
    output_ptr: felt*,
    pedersen_ptr: HashBuiltin*,
    range_check_ptr,
    ecdsa_ptr: SignatureBuiltin*,
    bitwise_ptr,
    ec_op_ptr: EcOpBuiltin*,
    poseidon_ptr: PoseidonBuiltin*,
}() -> () {
    alloc_locals;

    let (input: Input) = get_agreements();
    let (a: felt, b: felt) =aggregate(
        input.client_public_key,
        input.server_public_key,
        input.agreements_len,
        input.agreements,
        0,
        0
    );
    let result = a * input.settlement_price + b;

    assert output_ptr[0] = input.client_public_key;
    assert output_ptr[1] = input.server_public_key;
    assert output_ptr[2] = input.settlement_price;
    assert output_ptr[3] = a;
    assert output_ptr[4] = b;
    assert output_ptr[5] = result;
    let output_ptr = output_ptr + 6;

    return ();
}
