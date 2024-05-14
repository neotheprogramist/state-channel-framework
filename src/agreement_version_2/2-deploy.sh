#!/usr/bin/env bash

# Check if the arguments are provided
if [ $# -ne 7 ]; then
    echo "Usage: $0 <class_hash> <client_balance> <server_balance> <client_public_key> <server_public_key> <a> <b>"
    exit 1
fi

sncast --profile mateotest --wait deploy --class-hash $1 \
   -c "$2" "$3" "$4" "$5" "$6" "$7"