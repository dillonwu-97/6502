https://skilldrick.github.io/easy6502/#snake

For the instruction set, what is the size of each instruction?
How might we go about checking each instruction size?


There is a better way to deal with this, that is to create an abstraction opcode 
Then create implementations for specific addressing modes for the abstracted opcodes 
then for an input opcode, check which implementation should be used 
also can all of the 
# TODO: need to double check how the opcodes affect memory; load is easy but what about arithmetic operations? do they just modify the accumulator?
Will our system work for stores? 
if we store a value, what happens to the stored value? written back into mem?
yea some operations do a grab, and some can do a store in memory (not just sta family of opcodes)
so this means that we should technically be doing some operations in memory 
what if we further categorize into read / write operations
so a rewrite would be something like an abstraction for an opcode 
for each opcode, it is of type read / write / misc <-- is there actually "misc"? like break / interrupts
for each opcode, it has an addressing mode 


Need to break this down into smaller steps 


Maybe the best way to do this is to have a table of cycles 
1) table of cycles 
2) table of load / store / etc. 
3) table of addressing mode 
then for each int value, go from int -> Opcode, and the opcode has specific traits ?
but what does an arithmetic inheritance even entail 
e.g., for an add operation
if opcode inherits from arithmetic, then check if it does add
if it does add, then take two values and add it 
then pass it back to the opcode and check the addressing mode inheritance 
if addressing mode is immediate, then do something 
so should the opcodes have access to the cpu, or should the inheritance have access to the cpu?
yea i need to think more about this and redesign it i think 

each independent impl extension should have a list of opcodes that it manages 
when an opcode is checked, it should defer to a snippet of code that exists in the cpu.rs file 
    <-- where is the opcode checked?
this snippet of code checks to see what addressing mode to use 
the responsibility to decide whether to do a fetch or an immediate store is dependent on the individual rs file because some operations do both a load and a store 
within each category, we can also keep track of the cycle count as well 

- [ ] Build test cases for load 
    <-- what kind of test cases?
    - [ ] can search online for a bunch of test cases, 
    - [ ] test register setting
    - [ ] try to port exisiting tests
    - [ ] creating basic test cases for each instruction 
        <-- what is the basic functionality of a single instruction 
        <-- then branch out to handle memory
- [ ] Move load opcodes into the load files 
- [ ] Move opcodes into 
- [ ] rewrite for sta and lda with new model, then push
- [ ] build test cases for these specifically afterwards 
    - [ ] unit tests
    - [ ] integration tests 

Call grabbing memory fetch, and storing memory save to avoid confusion with the ld / st opcodes 
i think i also forgot to increment the pc after executing the first instruction ( pretty sure )

create enum in opcodes for addressing mode 
or instead, return a pointer to the memory and let the specific category of instruction decide how to deal with it so load will load a pointer to the memory 
depending on the addressing mode, switch statement to handle fetch_zpx / store_zpx?  

Not sure what borrowing is also in Rust 
so in our case, the folder is emulator, so that is our module 
trying to figure out why the test cannot find our rust emulator module 
maybe i have to modify the lib.rs file or something?
yea needed a lib.rs file in order for this to work 

if we have a lib.rs file, then use the library file as our crate it seems
it seems like when we create a lib.rs file, we have now indicated that the folder containing the src directory is a module that we can import

https://codegolf.stackexchange.com/questions/12844/emulate-a-mos-6502-cpu

# 1/26
Given an instruction, return its addressing mode
based on the addressing mode, return the pointer in memory
worst case complexity is we search all the way to the end, but most common instructions can get placed in the beginning so that's fine 
could also do two layers of pruning 
so all the LD instructions in a vector, and all the addressing mode instructions in a vector, with test case to make sure that they are all accounted for 
first find addressing mode, then check if it's load but this also takes up more memory 
addressing mode gives the memory, then a dispatch function to handle loading? 
then do a second search to see which handler is responsible for the dispatch
ok dispatch table for each instruction is better, instead of vector but keep the returning of the addressing mode?
a giant dispatch table reduces readability of the code 
a search for the correct handler to use doesnt do that 
this means u pass in the memory to the dispatch handlers 
yea for testing purposes at least, having each of them separately is probably better
and then we can store the vectors of opcodes in each of the files, so all load opcodes belong in the load file 



TODO:
- [x] vector of instructions, and if vector contains the instructions, return the corresponding addressing mode  
- [x] get_addr_mode fn 
- [x] put load opcodes in load file
- [x] fn to dispatch the opcodes within the file 
- x_handle_dispatch function feels a little bit redundant
- [ ] unit tests for load 

- [x] Add the addrmode code for abs abx and aby
- [ ] Create some unit tests to test out each load instruction 
- [ ] Opcode has a struct and in it is the addressing mode, and instruction class 
- then create a list of opcodes based on the category instead of manually coding it 
- so like build_load_list
- build_zpx_list, etc. 
- the problem is that I would have to manually add the trait for each instruction 
- so maybe there is some metaprogramming trick i can do here
- need 
addr_mode
opcode_type
cycle
for each from i -> 256:
    check if there is a corresponding Opcode
    if there is, build a struct on the opcode and place it into a table of 256 but the question is how to initialize the fields without doing it manually if that's even possible 
    yea will need to write a python script to build the table 
    for each li
    ok but do i wnat a struct for the opcode?
    

- [ ] Build a struct for a single opcode 
- [ ] Separate opcode into smaller lists 




exec -> 
    handle_dispatch ->
        addr_mode_handler
        switch case to match the opcode 
        where should the cycles be calculated?
        update_cycles function?

Idiosyncracies of Rust is something I should write about
A programming journal so to speak 


Future:
- [ ] Branch prediction
- [ ] Optimizations
- [ ] Need tooling to measure optimizations as well 

mutable borrowing is a concept in Rust that involves accessing data without ownership 
what is rust's borrow checker and how does it work?
ok there has to be another way, i.e. grab the memory position instead of the pointer?
or set mutability after 

if it's a read from memory, we need to read first and then pass the value to the thing doing the operation 
if it's a store to memory, we need to get the value then use it in cpu afterwards but the addressing mode can still stay the same i think 

what is copy vs clone?
every instruction as an addressing mode
maybe i should add a handler for faulting instructions on illegal opcodes 
maybe ILL_OP or something? but then going from ill_op -> a u8 value would be tricky 

might be better to make an ILL_OP for all the indices that dont have an instruction
then after creating the faulting instruction, we can try to 

Observation that the architecture of chromium looks very rust-like 
memory safety is heavily emphasized, and ownership is enforced

array indexing in rust is also weird 
Rust enforces move semantics
what is derive?
what is copy / clone?
what are copy vs move semantics?

where to handle each addressing mode?
still inside the cpu i think 
unless... we attach a function pointer to the addressing mode?
i need more practice working on designing intricate systems 

TODO:
- [ ] idx / inx addressing modes
- [ ] Run
- [ ] Paint
- [ ] Code, make a plan lol figure out how to make more money
- [ ] Do research, etc, blog post


Future: build compiler in PL for the 6502 architecture?
https://rust-unofficial.github.io/patterns/translations.html
probs should use this to look at common design patterns in rust
program counter increments fetch_byte()
problem with current design is that we are not sure which register to operate on based on the opcode
so we need to figure that out 
so redesign again??
need to make it into based on the name?
and then there are certain where the instruction only has a single addressing mode so we don't need to do anything there i think
ok yea need to modify the table once more
should've thought about the design a bit more before coding zzz 
instead of OpType, we should call it Inst
and then handle dispatch will dispatch based on instruction?
and the instruction + addressing mode belongs in each unique file
so organization is:

filename = laod/store operations
actually is there even a point to having separate files
cuz the code is just something like
handle_lda(op, mem_val)
    -> self.x = *mem_val
        self.set_status()

so the instruction type dictates how to use the memory
the addressing mode dictates where the memory is 
yea might not actually need the extra files tbh
but building unit tests would be much better in each file as opposed to more broadly and organizationally, it makes more sense
ok yea steps are:
- [x] modify the op enums again
should i just create a vector an initialize it in cpu instead of doing all this workaround?
Need to double check how the borrowing stuff works again at some point
interesting, should give it a read
https://www.thecodedmessage.com/tags/rust-vs-c++/
https://web.archive.org/web/20211204234443if_/http://archive.6502.org/datasheets/mos_6501-6505_mpu_preliminary_aug_1975.pdf

- [x] test cases next 
- [ ] clean up notes and write programming journals / reflections for the past month
- [ ] add cycles


i think there is something to be said about the cycles that we use 
fetch, decode, execute repeat cycle

how important is keeping track of cycles?
https://www.reddit.com/r/EmuDev/comments/1ibybip/feedback_on_my_6502_emulator/
this talks about cycles a little bit / their importance
how to keep track of cycles?
would probably be worth to buy a mos 6502 / old atari system online to take it apart and play with / mess around with 
which instructions have variable cycles?
how to handle these?
Trying to design the emulator for cycle accuracy is important because the next step is to integrate the emulator into a full NES emulator 
PLA = programmable logic array
better to have a state machine 
for the branch instruction, maybe we can keep track upon execute of whether or not a branch was taken and if it was, add an extra cycle, etc. instead of having separate tables for everything
https://archive.org/details/mos_microcomputers_programming_manual
https://github.com/SingleStepTests/65x02
converting these tests into rust code
future goal is to eventually pitch this as a class 
need to check the addressing modes for the branch instructions
crossed page <-- what does this mean 
what is a page crossing?
page crossing is when the high byte gets modified

the cycle count is based on the instruction and then if a boundary is crossed, the cycle count gets incremented 
so count should be incremented based on the opcode
need to pass as an argument i think
why is there no page boundary issue for some of the instructions, but a page boundary issue for other instructions?
know if it's crossed based on the abs + x
https://retrocomputing.stackexchange.com/questions/15621/why-dont-all-absolute-x-instructions-take-an-extra-cycle-to-cross-page-boundary
so there is a bit of parallelism in the operation
1) fetch opcode
2) fetch low byte 
3) fetch high byte, add x to low byte
4) read data
1) fetch low byte
2) add low byte to x, fetch high byte
3) check for carry?
4) if no carry, read high byte + done
4) if carry, dummy read bad high byte, increment high byte
5) read high byte

for write:

should i make this a cycle based design??


the "simultaneous operations" is not strictly the case; what actually happens is that it can get a instruction while finishing the current instruction
it's because for some given clock cycle, the ALU is responsible for doing the math while the CPU is responsible for doing the reads / writes

modern cpu's have the ability to handle more nuanced operations right

cannot do a dummy write

ok properly handling 

another option is to do something like increment op for each atomic operation involving a cycle
if we used a second table, then what?
if we used a second table, then we could do something like a check to see which to use, but the check could also just increment the op in that case
ok so cycles are attached to the instruction, not the addressing mode 
this means we need to check on a per instruction basis / instruction classification e.g. load instruction can use up an extra cycle for specific address modes
so if there is 

what is the problem that blockchain technologies solve exactly?
ok i need to find more problems to solve 

We can have a list of opcodes where we need to check for a page boundary, and if it is in the list then we pass in an optional argument, which we can do a check for before returning
OR
we set a page boundary bit when a page boundary has been crossed for an addressing mode operation
then depending on the instruciton, we check the bit and reset it and increment the cycle then and there
<-- this probably cleaner

Following addrmodes check the page boundary:
1) abx, aby, iny, rel
we can use overflowing add

There should be a free and effective voting system. How do we design something like that?
What are the traits that a voting system should have? 
Cryptographic security to make sure that you are who you claim to be. 
There needs to be a way to verify your identity within a certain country
e2e encryption 
ranked choice or some method to handle this 
the records should be anonymous. 
there should be a way to test this in nyc for local elections at the very least. 

push notes i should or something


there should be one set_status and we pass in the flags we need  to set for some operation instead of having the same code everywhere
how does the flag setting work again 
i dont want to have 8 parameters so maybe another enum for the different status bits
then do a check for each bit and manage the status flag accordingly?
so 0b1111 1111 means check all the registers
and we can have a shift by 1 for each or something like this?
do i need the status register object thing at all actually?
what I'm currently doing is manually writing in the code which flags need to be set but maybe I should do that as another precompute 
Then I can do the set flags stuff using the precomputed value and the register

this would require creating another table for all the flags that need to be checked
then, get rid of all the *_set_status functions
after getting rid of the *_set_status functions, deref the index of an op to see which flags need to be set
this can happen in the handle_dispatch function as well i believe 
in the all_set_status function, we do a check to see if for a given instruction, which flags are relevant AND then check if that flag is set and if it is set, then set the register
but this needs to be within the instruction so there's that 
this means we need the opcode again lol 

TODO:
- [!] Replace all the set_status functions with set_status_all 
    <-- doesnt work the way we want it to actually because although certain groups of opcodes share the same methods of setting status register flags, specific opcodes in instruction groups don't, i.e. there are enough edge cases in my opinion that writing a general function doesn't make as much sense 
- [x] change inst -> op instead





it might not be the case that all the flags are set because of some register
they could just be set because of an op so register should be an optional argument i think 
no optional arguments, but maybe we could use an optional type
generalized flag setting function might not have been the right way to go 
e.g. bit_test / brk are pretty different from the load sets
it seems like some opcodes are unique but some other ones share a common pattern  
this means for certain categories e.g. logical category, the bit setting will be unique to specific opcodes

- [ ] Test cases needed for:
    - [ ] store
    - [ ] transfer
    - [ ] stack 
    - [ ] logical

- [ ] Implementations needed for:
    - [ ] arithmetic
    - [ ] inc/dec

overflow vs carry flag?
http://stackoverflow.com/questions/69124873/understanding-the-difference-between-overflow-and-carry-flags

overflow is used to handle 

definitely test cases for this as a sidenote

adding / subtracting is harder than one might think 
https://www.masswerk.at/6502/6502_instruction_set.html#ADC
distinction b/t overflow and carry flag:
carry = add one to the next operation
overflow = related to the sign of a number 
0x0111 1111 = 127 

do i have to handle both signed and unsigned adds / subtracts?
When the addition result is 0 to 255, the carry is cleared.
When the addition result is greater than 255, the carry is set.
When the subtraction result is 0 to 255, the carry is set.
When the subtraction result is less than 0, the carry is cleared.

V flag used to indicate whether result is outside -128 -> 127
ok, how to actually check this in software?
need to keep track of the 7 bits before the first bit 
becase we also need knowledge of the two operands as well?
for example, if both numbers are the same signs 

case: opposite signs
+ - -1 + 127 -> 126 carry flag is set, but overflow flag shouldn't be set int theory 
- + -128 + 1 -> -127


a = self.ac 
b = mem 
c = i8 a + b
addition should just be in i8 
if both are negative, make sure the result is also negative
if both are positive, make sure both are also positive 
if one is positive, and the other is negative, cannot overflow 
we will definitely need test cases for this; very tricky stuff 

i am getting the second mutable borrow error 
i need to better understand when a borrow is occuring and when it can be dropped
it seems like it gets dropped after we exit the current scope

ok what is the reason it isn't working?
we have a mutable reference, and the borrow occurs when we call self.idc?
but then what's the distinction between this and other borrows?
what we are trying to do is not permissible because we have a mutable memory reference.
can i convert to a mutable reference borrowing??
we're doing block scoping.
this is annoying because it means I might have to modify fetch_byte() so that the program counter gets modified outside of it but i dont want to do that 
ok i cannot use self then 
maybe I can make idc() do a read first before using addr_mode_handler or something?
maybe self.pc should only change opcodes instead?
ok instead of modifying the pc inside of the code responsible for retreiving a memory address, we can modify the pc outside of it 
there is a byte table 
the bytes are consistent, and based on the addressing mode 
so the biggest problem is where to handle the program counter 
we have to call the handler multiple times which is the issue 
increment the program counter in the handle_dispatch function?
options:
ok the solution might be to have a table of bytes based on the address modes and then increment the pc based on the table 
the problem is that fetch_byte uses the program counter, so fetch_two() would be getting the same byte twice in the new model 
i think self.pc should be left as is
maybe there is a different way. 
so we need to borrow again to do an assignment though


1) have another table for the addr modes <-- not the biggest fan of this
2) a very very shitty way of dealing with this is to re-decrement after fetching 
or somehow separate the fetch logic from the addrmode logic, actually that might be better
but the problem is we need self.x too so we need the self.x value because it is used somewhere

do not wnat to pass in self.x separately
in theory, we could instead of getting the hard pointer, get an index into the stack
lmao not even the ai can understand rust's borrow checker
explaining borrowing in rust feels like trying to explain the offside rule; you can't explain it but you know it when u see it 

shift operations
- [ ] handle case of accumulator vs memory 
    <-- accumulator mode is very specific to shift operations it seems
- [ ] set the flags inside of the sh function should be ok i think, but the assignment will have to be done outside 
- [ ] TODO: write test cases to make sure self.ac vs self.mem is being handled properly 

okay, potential solution to my problems:
instead of returning a reference to the memory, return the index instead
if we return the index, then we can pass in and do an assignment without having to worry about the pointer stuff
so instead of returning self.memory, just return val / self.fetch_byte() , etc. 
but then for accumulator mode, i guess we could have special cases that don't use the addrmode?
since most of the time we are working with raw memory 

- [x] modify the addressing modes to return the index to memory instead of the pointer
- [x] move inc / dec code into separate file
- [ ] work on shifts 
- [ ] move files one layer up I think, not sure how to do this though 

https://forums.nesdev.org/
interesting forum for dev 

okay what's the logic of the byte? i can image this being something of a test question if i were to teach a class 
Why does the 6502 JSR instruction only increment the return address by 2 bytes?

Simply because the PC is already pushed before the second address byte is read. That way, the CPU need only buffer the lower target address byte and later read the higher one directly into the PC.

what are indirect subroutine calls?
how many registers are in the 6502 cpu? is there an internal operand buffer?
might be worth rereading this as well at some point
https://retrocomputing.stackexchange.com/questions/8022/why-does-this-6502-code-push-a-function-address-onto-the-stack-before-calling

might be because we are trying to save an operation or something like that?
we are not truly decrementing, we just don't do the increment and instead increment the pc afterwards?
read high byte, but do not increment the program counter b/c there is not point, instead save the existing pc and then increment upon restoring instead

                // https://retrocomputing.stackexchange.com/questions/19543/why-does-the-6502-jsr-instruction-only-increment-the-return-address-by-2-bytes

- [ ] Add relative addressing mode
    - [ ] save the current program counter 
    - [ ] need to check if we are at a new page as well 
        <-- check boundary_flag 
        <-- increment cycle count based on the if case
    - [ ] need to check if the branch succeeded
    - [ ] fetch the bytes
    - [ ] if branch conditional is true: 
        add the offset to the original pc 
    else continue

NES system 

So i want to parse the JSON files in the repo into rust so these should maybe be in test cases folder?
- [ ] is test folder in rust project special?
- [ ] read json file 
- [ ] probs better to build struct for faster parsing

# Questions
- [ ] what happens if we run out of stack space on the 6502 cpu?
 
# FUN idiosyncracies of 6502 architecture
- the only two instructions with accumulator modes are ROR and ROL 
- PLA / STA for rapid data transfer?
- is AES 256 bytes?
- what programming language were old NES games written in? was it just assembly?
page boundary idiosyncracy with indirect addressing mode 


Testing
Need to move all of the code into common.rs maybe?
or just a separate function in the test file
    <-- separate function in the test file probs better

pass in cpu, pass in v, do cpu.reset() each time and then init with new values

And then for the files in the directory, we should try to do research on creating separate test sections(?) for all of the files, but within one function if that's possible so that we can loop through all of the files in the directory and print out at the top descriptions of the corresponding opcode using our opcode table, so we'll need to import that as well
    <-- test wrapper is permissible 



Result::unwrap consumes the result object, moving the entry out of the Result and into the return value.

Either unwrap once and store the resulting object in a variable (let entry = entry.unwrap();) or use Result::as_ref to borrow the object inside of the Result (entry.as_ref().unwrap().path()...)
    <-- what does it mean to move the entry out of Result and into the return value?
    <-- 

rstest for test parametrization
https://lobste.rs/s/wrq7iv/guide_test_parametrization_rust
https://unterwaditzer.net/2023/rust-test-parametrization.html
what is building a testing harness in rust exactly?

Another problem I encountered in the project involves building test cases in a modular way. There is a testing suite that I found for each of the 256 opcodes, and there is a json file for each opcode. In pytest, we can have parameterized testing but this seems to be a little bit more difficult in Rust. In Rust, each test case must be a top level function. There were a couple of solutions that I played around with. 
1) A quick fix would be to loop over all the test files, and handle them in a single top level test function, but I don't like this method because it's not modular. 
2) Use rstest. 
    <-- Not sure if I can generate the test cases based on a filepath or if I have to manually write out the filenames
    <-- seems like the rstest provides documentation that this is supported so I opt to use this method 
    <-- testing individual / specific json files / opcodes is a bit tricky though
    <-- batch testing is fine but fine grained specification of which file to test is harder
    <-- for fine grained testing, I could try to add a command line flag or something like that
3) Use macros. 
4) Don't use Rust's default testing library and create a separate testing tool for this task 
While working on this, I also learned about testing harnesses which is pretty interesting. 

I ended up using macros instead of the other methods. 

what is continuous integration?


cargo test --test integration_test test_single -- --nocapture --quiet --test=00
something like this

# TODO: 
cargo test --test integration_test test_single -- --nocapture --quiet
TESTCASE=00 cargo test --test integration_test test_single -- --nocapture --quiet
this is even better for specifying env variables i think 




# Assignments for the class:

Project: Building MOS 6502 emulator
hw1: ctf challenge
