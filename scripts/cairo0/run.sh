#!/usr/bin/env bash

source .venv/bin/activate && \
cairo-run \
  --program target/cairo0/program.casm.json \
  --layout starknet \
  --print_output \
  --trace_file target/cairo0/trace.bin \
  --memory_file target/cairo0/memory.bin \
  --air_public_input target/cairo0/public_input.json \
  --air_private_input target/cairo0/private_input.json \
  --program_input target/generator_output/in.json \
  --proof_mode && \
deactivate
