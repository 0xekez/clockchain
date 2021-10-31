#!/bin/bash

set -e
set -o pipefail

case $1 in
    small-build)
	# Need to cargo wasm build before sending to docker in case a
	# dependency has changed and we need to redownload. Docker
	# image is run with --locked so it is not allowed to download
	# dependencies itself.
	cargo wasm
	docker run --rm -v "$(pwd)":/code \
	       --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
	       --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	       cosmwasm/rust-optimizer:0.11.5
	echo "=> your wasm binary is in artifacts/"
        ls -lh artifacts | rg "\.wasm"
    ;;
    deploy)
	echo "=> using pebblenet testnet"
	echo "=> assuming that you have created pebblenet wallets as described here: https://docs.cosmwasm.com/docs/0.16/getting-started/setting-env#setup-go-cli"
	curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/pebblenet-1/defaults.env > setup.env
	source setup.env
	rm setup.env
        NODE="--node $RPC"
	TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 0.001upebble --gas auto --gas-adjustment 1.3"

	SC=$(ls artifacts | rg "\.wasm")
	echo "=> deploying smart contract: ($SC)"

	RES=$(wasmd tx wasm store artifacts/$SC --from wallet $TXFLAG -y)
	CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
	echo "=> sucessfully uploaded contract with code id ($CODE_ID)"

	INIT='{}'
	wasmd tx wasm instantiate $CODE_ID "$INIT" \
	      --from wallet --label "hello world contract" $TXFLAG -y

	CONTRACT=$(wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
	echo "=> contract deployed with id:"
	echo "$CONTRACT"
    ;;
    execute)
	CONTRACT=$2
	curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/pebblenet-1/defaults.env > setup.env
	source setup.env
	rm setup.env
        NODE="--node $RPC"
	TXFLAG="${NODE} --chain-id ${CHAIN_ID} --gas-prices 0.001upebble --gas auto --gas-adjustment 1.3"
	BYTECODE=$(cat - | base64)


        # wasmd tx wasm execute $CONTRACT "{\"bytecode\":\"$BYTECODE\"}" --from wallet $TXFLAG -y 2>/dev/null | jq '.logs[0].events[2].attributes[1].value | tonumber'
	wasmd tx wasm execute $CONTRACT "{\"bytecode\":\"$BYTECODE\"}" --from wallet $TXFLAG -y
    ;;
    *)
	echo "usage: none do not use at all costs"
    ;;
esac
