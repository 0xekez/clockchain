#!/bin/bash

set -e
set -o pipefail

function measure() {
    local wallet=$1

    local start=$(date -u | awk '{split($0, spaces, " "); print spaces[4]}')
    local tx=$(junod tx wasm execute juno1sr06m8yqg0wzqqyqvzvp5t07dj4nevx9u8qc7j4qa72qu8e3ct8qm0adwt '{"bytecode":"AQoAAAAOAgAAABEAAAAAAgAAAAABAwAAAAsKAAAAAgAAAAABAQAAAAQAAAAADvr///8NAQAAAAECAAAABAAAAAAO9v///wMAAAAADwAAAAA="}' --from $wallet --output json --yes | jq -r '.txhash')

    sleep 60

    local end=$(junod query tx $tx --output json | jq -r .timestamp \
		    | awk '{split($0, d, "T"); print substr(d[2], 1, length(d[2]) - 1)}')

    datediff $start $end
}

if [ $1 == "" ]; then
    echo "usage: $0 <wallet>"
    echo "computes the 10 fibonacci number using the clockchain juno evaluator and reports the time taken."
fi

measure $1
