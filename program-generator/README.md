Generates valid programs for execution by smart contracts. The
programs sizes are limited by the maximum program size within Solana
compute limits. Generated programs do not have jump or div / mul
instructions to reduce the complexity of jumping to valid labels and
not dividing by zero.

`./run.sh slow-search N` will generate `N` programs and find the
slowest one with hyperfine when executed by Solana. Running that
requires that programs are built on your computer.
