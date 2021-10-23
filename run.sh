#!/bin/bash

case $1 in
    build)
	(cd assembler ; cargo build)
	(cd local-evaluator/ ; cargo build)
	;;
    run)
	cat assembler/examples/fib.bc \
	    | ./assembler/target/debug/assembler \
	    | ./local-evaluator/target/release/local-evaluator
	;;
    *)
	echo "usage: $0 <build|run>"
	;;
esac
