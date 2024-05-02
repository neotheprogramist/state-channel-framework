%builtins output pedersen range_check bitwise
from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.hash import hash2
from starkware.cairo.common.signature import (
    verify_ecdsa_signature,
)
from aggregate import aggregate
from input import (
    Agreement, get_agreements
)

func main{output_ptr: felt*, pedersen_ptr: HashBuiltin*, range_check_ptr: felt, bitwise_ptr: felt*}() -> () {
    alloc_locals;

    let (agreements_len: felt, agreements: Agreement**) = get_agreements();
    let (a: felt, b: felt) = aggregate(agreements_len, agreements, 0, 0);

    assert output_ptr[0] = a;
    assert output_ptr[1] = b;
    let output_ptr = output_ptr + 2;

    return ();
}
