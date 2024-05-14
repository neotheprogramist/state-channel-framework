#!/usr/bin/env bash

# Check if the arguments are provided
if [ $# -ne 8 ]; then
    echo "Usage: $0 <contract_address> <quantity> <nonce> <price> <server_signature_r> <server_signature_s> <client_signature_r> <client_signature_s>"
    exit 1
fi

sncast --profile mateotest \
    --wait call \
    --contract-address "$1" \
    --function apply  \
    --calldata "$2" "$3" "$4" "$5" "$6" "$7" "$8" 
