#!/usr/bin/env bash

# Check if the arguments are provided
if [ $# -ne 9 ]; then
    echo "Usage: $0 <contract_address> <quantity> <nonce> <price> <server_signature_r> <server_signature_s> <client_signature_r> <client_signature_s>"
    exit 1
fi

profile=$1
contract_address=$2

sncast --profile "$profile" \
    --wait call \
    --contract-address "$contract_address" \
    --function apply  \
    --calldata "$3" "$4" "$5" "$6" "$7" "$8" "$9" 