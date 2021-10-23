# Solana Evaluator

- `client/` contains logic for running a deployed solana bytecode
  evaluator program.
- `program/` contains a Solana smart contract capiable of executing
  bytecode.

To run the programs locally you'll need the Solana and Rust toolchains
installed. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Solana can
be found [here](https://docs.solana.com/cli/install-solana-cli-tools).

## Local Development

Once you've completed the Solana installation run the following
commands to configure you machine for local development:

```
solana config set --url localhost
solana-keygen new
```

These two commands create Solana config files in `~/.config/solana/`
which solana command line tools will read in to determine what cluster
to connect to and what keypair to use.

Having done that run a local Solana validator by running:

```
solana-test-validator
```

This program must be left running in the background.

## Devnet Devlopment

If you would like to run on the Solana dev net run

```
solana config set --url https://api.devnet.solana.com
```

You may need to airdrop yourself some money to deploy the program:

```
solana airdrop 5
```

## Deploying the program

```
./run.sh deploy
```

## Running the program

```
./run.sh client
```

This will hang waiting for input from stdin. The input is expected to
be bytecode. See the `run-solana` target `../run.sh` for an example of
using this command.
