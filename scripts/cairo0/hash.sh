#!/usr/bin/env bash

source .venv/bin/activate && \
mkdir -p target/cairo0 && \
cairo-hash-program \
  --program target/cairo0/program.casm.json && \
deactivate
