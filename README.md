# The ICE-9 virtual machine specification


## Instruction set

ICE-9 is a simple stack-based virtual machine, containing the following instructions.

`c/n` -- push constant at index n to the stack

`e` -- pop and echo the top item on the stack

`+` -- pop the top two elements on the stack, and push their sum

`r` -- return, or exit if at bottom of the call stack (REQUIRED AT END OF PROGRAM)

`=` -- push 1 if the top two elements are equal, 0 otherwise

`j/n` -- set PC to n if top of stack (popped) equals 1

`g/n` -- goto n

`x/n` -- call subroutine at n, pushing a new stack frame

`s/n` -- set variable n to the value at the top of the stack

`v/n` -- push the value of variable n

`*` -- multiple the top two elements, pushing their product

`-` -- subtract the top two elements, pushing their difference

`>` -- greater than (2nd greater than first)

`<` -- less than (2nd less than first)

`|` -- or

`&` -- and

`!` -- not

`%` -- modulo

## Implementations

*Hydro* is the primary implementation of the ICE-9 virtual machine, written
in the Liquid temlating language.

*Freon* is an implementation of the ICE-9 virtual machine written in Rust for easier
debuging.

Any difference in behaviour between Hydro and Freon is an implementation error.


## Plasma

Plasma is a langage that compiles to ICE-9 assembly, which can then be
interpreted by liquid.

Here is some sample code. Note that the syntax is highly unstable.
This code has been tested and runs successfully when compiled / interpreted.

```
candidate = 2;

while candidate < 1000 {
  prime? = 1;
  divisor = 2;

  while divisor < bound {
    if (candidate % divisor) == 0 {
      prime? = 0;
    }
    divisor = divisor + 1;
  }

  if prime? {
    echo candidate;
  }

  candidate = candidate + 1;
}
```
