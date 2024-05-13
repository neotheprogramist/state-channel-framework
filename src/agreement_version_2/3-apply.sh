#!/usr/bin/env bash

# Check if the arguments are provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <contract_address> <fact_hash>"
    exit 1
fi

# Assign arguments to variables
contract_address=$1
fact_hash=$2

sncast --profile testingnet  \ 
    --wait call \
    --contract-address "$contract_address" \ 
    --function apply  \
    --calldata "$fact_hash"
# Pass the calldata to the sncast command
# sncast --profile testnet \
#   --wait \
#   call \
#   --contract-address "$contract_address" \
#   --function "is_valid" \
#   --calldata 1 2 3 4 5 6 7