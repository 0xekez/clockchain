#!/bin/bash

PROGRAM="long.bc"

case $1 in
    build-local)
	(cd assembler/ ; cargo build)
	(cd local-evaluator/ ; cargo build --release)
	(cd program-generator/ ; cargo build)
	;;
    build-solana)
	(cd solana-evaluator/ ; ./run.sh build)
	;;
    run-solana)
	cat assembler/examples/long.bc \
	    | ./assembler/target/debug/assembler \
	    | (cd solana-evaluator/ ; ./run.sh client)
	;;
    generate-and-run-solana)
	./program-generator/target/debug/program-generator \
	    | ./assembler/target/debug/assembler \
	    | (cd solana-evaluator/ ; ./run.sh client)
	;;
    build-ethereum)
	(python3 ethereum-evaluator/deploy.py Interpreter)
	;;
    run-ethereum)
	cat assembler/examples/$PROGRAM \
	    | ./assembler/target/debug/assembler \
	    | python3 ethereum-evaluator/exec.py Interpreter execute
	;;
    run-local)
	cat assembler/examples/$PROGRAM \
	    | ./assembler/target/debug/assembler \
	    | ./local-evaluator/target/release/local-evaluator
	;;
    build-workers)
	echo "nothing to do. make sure the current workers deploy is up to date."
	;;
    run-workers)
	cat assembler/examples/$PROGRAM \
	    | ./assembler/target/debug/assembler \
	    | ./workers-evaluator/client.sh
	;;
    *)
	echo "usage: $0 <build|run>"
	;;
esac
