#!/bin/bash

case $1 in
    time-program-solana)
	PROGRAM="$2"
	hyperfine "cat $PROGRAM | ../assembler/target/debug/assembler | (cd ../solana-evaluator ; ./run.sh client)" --export-json test.json > /dev/null
	cat test.json | jq '.results[0].mean'
	rm test.json
	;;
    slow-search)
	rm -rf slow-search-programs
	mkdir slow-search-programs
	REPS="$2"
	for i in {1..$REPS}
	do
	    cargo run > generated.bc
	    ./run.sh time-program generated.bc > time
	    mkdir slow-search-programs/$i
	    mv generated.bc slow-search-programs/$i
	    mv time slow-search-programs$i
	done
	;;
    *)
	echo "usage: $0 <slow-search | time-program-solana> <reps | program>"
	;;
esac
