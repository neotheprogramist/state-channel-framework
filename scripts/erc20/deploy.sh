#!/usr/bin/env bash

sncast \
    --url https://free-rpc.nethermind.io/sepolia-juno/v0_7 \
    deploy \
    --class-hash 0x3f2315af640989f095f41d138eac443d3aaa1281f6b8f7557d4095e36be0fd9 \
    --constructor-calldata \
    0xcee714eaf27390e630c62aa4b51319f9eda813d6ddd12da0ae8ce00453cb4b \
    0xcee714eaf27390e630c62aa4b51319f9eda813d6ddd12da0ae8ce00453cb4b \
    0xcee714eaf27390e630c62aa4b51319f9eda813d6ddd12da0ae8ce00453cb4b \
    0xcee714eaf27390e630c62aa4b51319f9eda813d6ddd12da0ae8ce00453cb4b
