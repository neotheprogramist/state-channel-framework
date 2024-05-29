#!/usr/bin/env bash

# Check if the arguments are provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <contract_address> <fact_hash>"
    exit 1
fi

contract_address=$1
fact_hash=$2

sncast --profile testingnet \
    --wait \
    call \
    --contract-address  "$contract_address" \ 
    --function get_agreement_by_id --calldata "$fact_hash"