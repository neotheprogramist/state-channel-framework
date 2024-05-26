#!/usr/bin/env zsh

mkdir -p target/provable && \
cairo1-compile compile provable/src/lib.cairo > target/provable/compiled.sierra.json && \
cairo1-compile merge --output target/provable/compiled_with_input.json target/provable/compiled.sierra.json provable/input.json
