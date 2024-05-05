#!/usr/bin/env bash

source .venv/bin/activate && \
mkdir -p resources/$1 && \
cairo-compile \
  cairo0/$1.cairo \
  --cairo_path cairo0 \
  --output resources/$1/compiled.json \
  --proof_mode && \
deactivate
