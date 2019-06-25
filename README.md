# The ICE-9 virtual machine specification


## Instructions set

ICE-9 is a simple stack-based virtual machine, containing the following instructions.

`c/n` -- push constant at index n to the stack

`e` -- pop and echo the top item on the stack

`a` -- pop the top two elements on the stack, and push their sum

`r` -- return, or exit if at bottom of the call stack (REQUIRED AT END OF PROGRAM)

`q` -- push 1 if the top two elements are equal, 0 otherwise

`j/n` -- set PC to n if top of stack (popped) equals 1

(temporary)
`dup` -- duplicate the top of the stack (I just want some quick turing completeness for now,
         will eventually be replaced with local variables + stack frames)

## Implementations

*Hydro* is the primary implementation of the ICE-9 virtual machine, written
in the Liquid temlating language.

*Freon* is an implementation of the ICE-9 virtual machine written in Rust for easier
debuging.

Any difference in behaviour between Hydro and Freon is an implementation error.
