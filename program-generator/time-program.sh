#!/bin/bash

PROGRAM="$1"

hyperfine "cat $PROGRAM | ../assembler/target/debug/assembler | (cd ../solana-evaluator ; ./run.sh client)" --export-json test.json > /dev/null

cat test.json | jq '.results[0].mean'

rm test.json
