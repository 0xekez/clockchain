#!/bin/bash

case $1 in
    build-local)
	(cd assembler/ ; cargo build)
	(cd local-evaluator/ ; cargo build --release)
	;;
    build-solana)
	(cd solana-evaluator/ ; ./run.sh build)
	;;
    run-solana)
	cat assembler/examples/fib.bc \
	    | ./assembler/target/debug/assembler \
	    | (cd solana-evaluator/ ; ./run.sh client)
	;;
    run-local)
	cat assembler/examples/fib.bc \
	    | ./assembler/target/debug/assembler \
	    | ./local-evaluator/target/release/local-evaluator
	;;
    *)
	echo "usage: $0 <build|run>"
	;;
esac
