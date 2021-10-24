# Clockchain

Clockchain is a system for benchmarking smart contract execution times
across blockchains using Arcesco-- a block-chain agnostic instruction set.

It consists of three parts:

1. A simple instruction set composed of 17 instructions.
2. A bytecode specification and assembler for that instruction set.
3. A collection of runtimes to execute Arcesco bytecode.

These work together to form a benchmarking system as follows:

1. A set of example programs to test various performance
   characteristics are assembled.
2. For each blockchain of interest a smart contract is deployed which
   will take Arcesco bytecode, interpretet it, and report the result.
3. For each smart contract a runtime is created which, given some
   local bytecode, sends that to its corresponding contract and
   reports information about execution time.

All three components have a unified interface which uses Unix pipes. As
such all invocations are in the following form:

```
cat fib.bc | assembler | executer
```

In this way Clockchain presents a unified, extenable interface for
benchmarking smart contract execution times. This repo also contains
implementations of evaluators in the three most popular smart-contract
platforms: Ethereum, Solana, and Polkadot.

## An Example Program

Here is an example program computes the 10th fibonacci number:

```
	pi 10
	call fib
	exit
fib:
	copy
	pi 2
	jlt done
	copy
	pi 1
	sub
	call fib
	rot 1
	pi 2
	sub
	call fib
	add
done:
	ret

```

To run this program locally first install Rust and then build the
assembler and local-evaluator programs:

```bash
(cd assembler ; cargo build)
(cd local-evaluator/ ; cargo build)
```

Then, to run the fib program place it in a file called `fib.bc` (here
we use the example one in `assembler/examples/fib.bc`) and run:

```bash
cat assembler/examples/fib.bc \
    | ./assembler/target/debug/assembler \
    | ./local-evaluator/target/release/local-evaluator
```

## The Instruction Set

The Arcesco instruction set is composed of 17 instructions which
operate on 32 bit signed integers. There are no other types in Arcesco
other than the 32 bit signed integer.

The Arcesco runtime consists of a stack machine and in addition to the
bytecode must maintain the following state in order to execute Arcesco
programs.

```rust
struct InterpreterState {
    pc: u32,
    call_stack: Vec<u32>,
    stack: Vec<i32>,
}
```

`pc` here refers to program counter and increments by one after each
instruction is executed unless a jump or call instruction otherwise
moves execution flow.

```
opcode | instruction  | explanation
-----------------------------------
1      | pi   <value> | push immediate - pushes VALUE to the stack
2      | copy         | duplicates the value on top of the stack
3      | add          | pops two values off the stack and adds them pushing
                        the result back onto the stack.
4      | sub          | like add but subtracts.
5      | mul          | like add but multiplies.
6      | div          | like add but divides.
7      | mod          | like add but modulus.
8      | jump <label> | moves program execution to LABEL
9      | jeq  <label> | moves program execution to LABEL if the two two
                        stack values are equal. Pops those values from the
						stack.
10     | jneq <label> | like jeq but not equal.
11     | jlt  <label> | like jeq but less than.
12     | jgt  <label> | like jeq but greater than.
13     | rot  <value> | swaps stack item VALUE items from the top with the
                        stack item VALUE-1 items from the top. VALUE must
						be >= 1.
14     | call <label> | moves program execution to LABEL and places the
                        current PC on the runtime's call stack
15     | ret          | sets PC to the value on top of the call stack and
                        pops that value.
16     | pop          | pops the value on top of the stack.
17     | exit         | terminates program execution. The value at the top
                        of the stack is the program's return value.
```

For operations where the oder of operands matter the topmost value on
the stack is considered to eb the right side of the equation and the
one below that the left. For example the following program returns 1:

```
pi 2
pi 1
sub
exit
```

Labels appear like regular instructions with no immediates except that
they end with a `:`. For example, in the following program `foo` is a
label:

```
foo:
	pi 1
	add
	ret
```

## Assembly

During assembly to bytecode all labels are removed from the program
and jump and call instructions have their immediates replaced by
relative jumps. Here is an example of what that looks like using our
earlier fibonacci code:

Before assembly the code has labels and jumps to those labels.

```
  pi 4
  call fib
  exit

fib:
  copy
  pi 2
  jlt done
  copy
  pi 1
  sub
  call fib
  rot 1
  pi 2
  sub
  call fib
  add
done:
  ret
```

The bytecode after having its labels removed and jumps filled with
their relative versions:

```
pi 4
call 2
exit
copy
pi 2
jlt 10
copy
pi 1
sub
call -6
rot 1
pi 2
sub
call -10
add
ret
```

## The Bytecode

Every Arcesco instruction is encoded into a 40 bit instruction the
layout of which is as follows:

```
0        8                              40
+--------+-------------------------------+
| opcode |    immediate                  |
+--------+-------------------------------+
```

8 bits are reserved for the opcode and another 32 are reserved for the
instruction's immediate. Instructions without immediates are still
this size but ignore the value in the immediate.

Immediate values are encoded in little endian format. For further
documentation of the Arcesco instruction set, check out `assembler`.

## Reference Implementation

I've put together a reference Arcesco runtime in
`local-evaluator/src/main.rs`.

## Ethereum Impletmentation

- Dependencies: solc, geth
- Build: ./run.sh build-ethereum
- Run: ./run.sh run-ethereum

## Solana Impletmentation

- Dependencies: see `solana-evaluator/README.md`
- Build: ./run.sh build-solana
- Run: ./run.sh run-solana

## Polkadot Impletmentation

- Dependencies: see `polka-evaluator/README.md`
- We couldn't figure out how to run this outside of the polkadot UI,
  so this requires a independent solution.