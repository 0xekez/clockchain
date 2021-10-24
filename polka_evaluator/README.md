# Solana Evaluator

- `Cargo.toml/` contains package imports for the smart contract.
- `lib.rs/` contains the actual interpretor smart contract code, written in [Ink!](https://github.com/paritytech/ink).

To run the programs locally you'll need the Substrate and Rust toolchains
installed. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Substrate/Ink can
be found [here](https://github.com/paritytech/ink). We recommend going through Ink's Flipper tutorial's node and Rust set up.

## Local Development

Once you've completed the Substrate/Ink! installation, compile the Ink! smart contract:

```
cargo +nightly contract build
```

And to test:
```
cargo +nightly test
```

In order to run this smart contract, we need to spin up a local development node:

```
substrate-contracts-node --dev --tmp
```

This will run in the background. from within [Canvas.ui](https://paritytech.github.io/canvas-ui/#/upload) / [this thing](https://cloudflare-ipfs.com/ipns/dotapps.io/), upload the smart contract and run! 
