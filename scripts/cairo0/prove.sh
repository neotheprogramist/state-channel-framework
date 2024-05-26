#!/usr/bin/env bash

cairo-prove -k $1 -u http://prover.visoft.dev:3618 -c 0 < target/cairo0/compiled_with_input.json > target/cairo0/proof.json
