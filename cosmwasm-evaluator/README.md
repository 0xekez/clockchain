# cosmwasm executor

A cosmwasm smart contract which evaluates bytecode.

## build and deploy

You'll need to have wasmd and Rust installed. See the directions
[here](https://docs.cosmwasm.com/docs/0.16/getting-started/installation). You'll
also need to create a wallet named `wallet` and airdrop some testnet
monies into it by following the instructions
[here](https://docs.cosmwasm.com/docs/0.16/getting-started/setting-env#setup-go-cli).

With those installed you can build and deploy the contract by running:

```
./run.sh small-build
./run.sh deploy
```

## execute

Given the address of the smart contract (as outputed by `./run.sh
deploy`) run:

```
cat bytecode | ./run.sh execute
```

`bytecode` above ought to be a file containing the bytecode that you
would like to execute. See `../assembler` for information about
compiling bytecode.
