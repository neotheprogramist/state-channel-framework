#!/usr/bin/env bash

source .venv/bin/activate && \
mkdir -p resources/$1 && \
cairo-hash-program \
  --program resources/$1/compiled.json && \
deactivate
