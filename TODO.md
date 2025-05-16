# TODO:
- [x] Open file, and read the bytes from the rust file 
- [x] Build struct for the memory and cpu
- [ ] Add cycles to the code based on the documentation available
    <-- http://www.6502.org/users/obelisk/6502/reference.html#LDA

# 12/1
- are there macros that I can do in Rust to do an assignment for a specific bit in Rust?
- not sure if the instructions need to be in different files and whatnot
- first thing to do is to do the load instruction
- [x] implement load instruction
- [x] .h file in Rust to hold all the instructions 
- [x] try out the bitflags crate to use bitflags for the status registers

# 12/7
- [x] Implement load zero page
- [x] Implement load zero page x
- [x] separate the rust cpu structure from the main file
- [x] move the rust cpu structure into a module 

# 12/13
- [x] The load thing wont' work as intended because of the flags issue
- [ ] separate the load instructions from the cpu structure and inherit stuff
- [ ] begin developing rust test cases for load instructions
- [ ] move the opcodes into the correct handler files
- [ ] implement jmp 
- [x] Simplify the code in load.rs esp the redundant stuff
    - [x] single byte / double byte load would be a good change

# 1/23
- [ ] Review what I was working on previously and come up with a plan 
- [ ] Check if there are include files in rust
- [ ] move the fetch_operand functions into the main cpu file 
- [ ] create a wrapper function that checks the addressing mode of an opcode in operands.rs

# Future
- [ ] add cycles
- [ ] Figure out the warnings many warnings, much annoying
- [ ] Build the store crap
- [ ] load test cases
- [ ] store test cases
- [ ] turn some of the IMM/ZPG/ZPX stuff into macros instead? is that possible?
- [ ] Might be good to build a table instead for all of the cycle counts  

# Opcodes
- [ ] lda 
    - [ ] indirect_x
    - [ ] indirect_y
- [x] ldx
- [x] ldy
- [ ] sta
    - [ ] indirect_x
    - [ ] indirect_y
- [x] stx
- [x] sty
- [ ] logical ops
    - [ ] indirect x
    - [ ] indirect y 
    - [ ] bitset
- [x] stack 

What is the end goal of the project?
I want to be able to build this into an atari emulator and do some of the graphics stuff as well 
To that end, it would be useful to save the cycles 
Build out test cases
what is the best way to design this emulator?
oh yea i was thinking about creating a table for instruction / addressing mode instead of my current system as a linear list 
but how would we go from opcode to the instruction?
with the list we have, the addressing mode is already laid out correctly 

table of opcodes?
was i working on logical operators??
also, not sure what kind of test cases I want to build to see if it can run 
what was the design paradigm as well?
i should have taken more notes for this
i feel like i wanted to change load.rs to look more like arithmetic.rs file
i was also trying to get rid of the or statements so that we could group by the addressing mode operation as well I think with the idea of trying to make the code as compact as possible 
so group by addressing mode and then pass a function handler that works as a closure essentially 
what if instead, we grouped by addressing modes and then had a closure for each of the operations
test cases:
1. Need to build test cases that use each of the opcodes to make sure they work 

i think we should use inherent impls? 
interesting, we could use name to get the opcode?
I need to definitely build some simple test cases
