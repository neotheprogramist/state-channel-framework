from input import Agreement
from starkware.cairo.common.signature import (
    verify_ecdsa_signature,
)

from starkware.cairo.common.cairo_builtins import SignatureBuiltin 
from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.hash import hash2
from starkware.cairo.common.hash_chain import hash_chain

from starkware.cairo.common.cairo_builtins import HashBuiltin

func aggregate{output_ptr: felt*, pedersen_ptr: HashBuiltin*,ecdsa_ptr: SignatureBuiltin*}(agreements_len: felt, agreements: Agreement**, a: felt, b: felt, server_public_key: felt, client_public_key: felt) -> (a: felt, b: felt) {
    if (agreements_len == 0) {
        return (a=a, b=b);
    }

    let agreement_quantity:felt = agreements[0].quantity;
    let agreement_nonce:felt = agreements[0].nonce;
    let agreement_price:felt = agreements[0].price;
    let agreement_server_signature_r: felt = agreements[0].server_signature_r;
    let agreement_server_signature_s :felt= agreements[0].server_signature_s;
        //667047919106426844415409795208142446036318900740055715479849966787227123817.

    assert agreement_quantity=1;
    assert agreement_nonce=0x7c219f1adcf0131760e6ec9cbb78e5ca30b76209983e45883ebf5741571a848;
    assert agreement_price=1715;
    let address:felt  = 0x4b3f4ba8c00a02b66142a4b1dd41a4dfab4f92650922a3280977b0f03c75ee1;
    assert output_ptr[0] = agreement_quantity;
    let  output_ptr = output_ptr+1;
    let (arr: felt*) = alloc();

    // Computes a hash chain of a sequence whose length is given at [data_ptr] and the data starts at
    // data_ptr + 1. The hash is calculated backwards (from the highest memory address to the lowest).
    // For example, for the 3-element sequence [x, y, z] the hash is:
    //   h(3, h(x, h(y, z)))
    assert arr[0] = 4;
    assert arr[1] = address;
    assert arr[2] = agreement_quantity;
    assert arr[3] = agreement_nonce;
    assert arr[4] = agreement_price;
    let (agreement_hash:felt)= hash_chain{hash_ptr=pedersen_ptr}(arr);
    verify_ecdsa_signature(
        message = agreement_hash,
        public_key = server_public_key,
        signature_r = agreement_server_signature_r,
        signature_s = agreement_server_signature_s,
    );    
    
    return aggregate(
        agreements_len - 1,
        agreements + 1,
        a + agreement_quantity,
        b - agreement_quantity * agreement_price,
        server_public_key,
        client_public_key
    );
}