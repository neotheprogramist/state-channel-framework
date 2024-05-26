%builtins output pedersen range_check bitwise
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.hash import hash2
from starkware.cairo.common.signature import (
    verify_ecdsa_signature,
)
from aggregate import aggregate
from input import (
    Input, get_agreements
)

func main{output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr: felt, bitwise_ptr: felt*}() -> () {
    alloc_locals;

    let (input: Input) = get_agreements();
    let (a: felt, b: felt) = aggregate(input.agreements_len, input.agreements, 0, 0);
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
