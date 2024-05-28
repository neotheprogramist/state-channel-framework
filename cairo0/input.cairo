struct Input {
    client_public_key: felt,
    server_public_key: felt,
    agreements_len: felt,
    agreements: Agreement**,
    settlement_price: felt,
}

struct Agreement {
    quantity: felt,
    nonce: felt,
    price: felt,
    server_signature_r: felt,
    server_signature_s: felt,
    client_signature_r: felt,
    client_signature_s: felt,
}

func get_agreements() -> (input: Input) {
    alloc_locals;
    local input: Input;
    local agreements_len: felt;
    local agreements: Agreement**;
    %{
        program_input_agreements = program_input["agreements"]
        agreements = [
            (
                int(agreement["quantity"]),
                int(agreement["nonce"], 16),
                int(agreement["price"]),
                int(agreement["serverSignatureR"], 16),
                int(agreement["serverSignatureS"], 16),
                int(agreement["clientSignatureR"], 16),
                int(agreement["clientSignatureS"], 16),
            )
            for agreement in program_input_agreements
        ]
        ids.input.client_public_key = int(program_input["clientPublicKey"], 16)
        ids.input.server_public_key = int(program_input["serverPublicKey"], 16)
        ids.input.settlement_price = int(program_input["settlementPrice"])
        ids.input.agreements_len = len(agreements)
        ids.input.agreements = segments.gen_arg(agreements)
    %}
    return (input=input);
}
