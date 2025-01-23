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

# Future
- [ ] add cycles
- [ ] Figure out the warnings many warnings, much annoying
- [ ] Build the store crap
- [ ] load test cases
- [ ] store test cases
- [ ] turn some of the IMM/ZPG/ZPX stuff into macros instead? is that possible?
- [ ] Might be good to build a table instead for all of the cycle counts  
- [ ] Table for the 

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



