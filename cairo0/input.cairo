struct Agreement {
    client_public_key: felt,
    server_public_key: felt,
    quantity: felt,
    nonce: felt,
    price: felt,
    server_signature_r: felt,
    server_signature_s: felt,
    client_signature_r: felt,
    client_signature_s: felt,
}

func get_agreements() -> (agreements_len: felt, agreements: Agreement**) {
    alloc_locals;
    local agreements_len: felt;
    local agreements: Agreement**;
    %{
        program_input_agreements = program_input

        agreements = [
            (
                int(agreement["clientPublicKey"]),
                int(agreement["serverPublicKey"]),
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
        ids.agreements_len = len(agreements)
        ids.agreements = segments.gen_arg(agreements)
    %}
    return (agreements_len=agreements_len, agreements=agreements);
}
