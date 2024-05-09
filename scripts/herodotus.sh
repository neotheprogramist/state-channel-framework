#!/usr/bin/env bash

sncast \
    --url https://free-rpc.nethermind.io/sepolia-juno/v0_7 \
    call \
    --contract-address 0x008ec19768587f3d7362d83b644253de560c9d46eda096619df5942d56079abb \
    --function get_slot_value \
    --calldata 0xb116f0f7383600C87af9aFDC4FA33D525Cfd19AA 5853310 0 8 0
