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
        def parse_hex_or_int(value):
            if isinstance(value, str) and value.startswith('0x'):
                return int(value, 16)
            return int(value)

        program_input_agreements = program_input["agreements"]
        agreements = [
            (
                parse_hex_or_int(agreement["quantity"]),
                parse_hex_or_int(agreement["nonce"]),
                parse_hex_or_int(agreement["price"]),
                parse_hex_or_int(agreement["serverSignatureR"]),
                parse_hex_or_int(agreement["serverSignatureS"]),
                parse_hex_or_int(agreement["clientSignatureR"]),
                parse_hex_or_int(agreement["clientSignatureS"]),
            )
            for agreement in program_input_agreements
        ]
        ids.input.client_public_key = parse_hex_or_int(program_input["clientPublicKey"])
        ids.input.server_public_key = parse_hex_or_int(program_input["serverPublicKey"])
        ids.input.settlement_price = parse_hex_or_int(program_input["settlementPrice"])
        ids.input.agreements_len = len(agreements)
        ids.input.agreements = segments.gen_arg(agreements)
    %}
    return (input=input);
}
