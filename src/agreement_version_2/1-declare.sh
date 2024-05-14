#!/usr/bin/env bash
# Check if the arguments are provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <profile>"
    exit 1
fi

profile=$1

sncast --profile "$profile" --wait declare --contract-name AgreementVersion2