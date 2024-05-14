#!/usr/bin/env bash

# # Check if the arguments are provided
# if [ $# -ne 8 ]; then
#     echo "Usage: $0 <profile> <class_hash> <client_balance> <server_balance> <client_public_key> <server_public_key> <a> <b>"
#     exit 1
# fi
Check if the arguments are provided
if [ $# -ne 4 ]; then
    echo "Usage: $0 <profile> <class_hash> <client_public_key> <server_public_key>"
    exit 1
fi

sncast --profile "$1" --wait deploy --class-hash "$2" \
   -c "1000000" "1000000" "$3" "$4"  "0" "0"