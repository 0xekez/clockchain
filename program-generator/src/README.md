# Program Generator

Generates programs. Generated programs do not have jump or call instructions.

A valid program:

1. Ends with an exit instruction.
2. Does not attempt to pop from an empty stack.
3. Does not attempt to rotate in position zero or off the stack.
