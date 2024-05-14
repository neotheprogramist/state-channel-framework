#!/usr/bin/env bash

# Check if the arguments are provided
if [ $# -ne 1 ]; then
    echo "Usage: $0 <class-hash>"
    exit 1
fi

# Assign argument to variable
class_hash=$1

sncast --profile testingnet2 --wait deploy --class-hash "$class_hash"