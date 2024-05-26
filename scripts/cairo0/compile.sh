#!/usr/bin/env bash

source .venv/bin/activate && \
mkdir -p target/cairo0 && \
cairo-compile \
  cairo0/main.cairo \
  --cairo_path cairo0 \
  --output target/cairo0/program.casm.json \
  --proof_mode && \
deactivate
