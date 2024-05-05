struct Input {
    client_public_key: felt,
    server_public_key: felt,
    agreements_len: felt,
    agreements: Agreement**,
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
                int(agreement["nonce"]),
                int(agreement["price"]),
                int(agreement["serverSignatureR"]),
                int(agreement["serverSignatureS"]),
                int(agreement["clientSignatureR"]),
                int(agreement["clientSignatureS"]),
            )
            for agreement in program_input_agreements
        ]
        ids.input.client_public_key = int(program_input["clientPublicKey"])
        ids.input.server_public_key = int(program_input["serverPublicKey"])
        ids.input.agreements_len = len(agreements)
        ids.input.agreements = segments.gen_arg(agreements)
    %}
    return (input=input);
}
