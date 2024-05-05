#!/usr/bin/env bash

source .venv/bin/activate && \
cairo-run \
  --program resources/$1/compiled.json \
  --layout recursive \
  --print_output \
  --trace_file resources/$1/trace.bin \
  --memory_file resources/$1/memory.bin \
  --air_public_input resources/$1/public_input.json \
  --air_private_input resources/$1/private_input.json \
  --program_input $2 \
  --proof_mode && \
deactivate
