mod opcodes;
use opcodes::Opcode;
use bitflags::bitflags;

const MEMSIZE: usize = 2 << 16;

bitflags! {
    struct  StatusRegister: u8 {
        const C = 0b0000_0001; // carry
        const Z = 0b0000_0010; // zero
        const I = 0b0000_0100; // interrupt
        const D = 0b0000_1000; // decimal 
        const B = 0b0001_0000; // break
                    // unused 5th bit
        const V = 0b0100_0000; // overflow
        const N = 0b1000_0000; // negative
    }
}

struct CPU {

    memory: [u8; MEMSIZE],
    pc: u16,
    ac: u8, // accumulator
    x: u8,
    y: u8,
    sr: StatusRegister, // bitfield for the status bits
    // maybe define some constants and then do the | operation? 
    // not really sure lol 
    sp: u8,

    // TODO: What should the size of this be?
    // TODO: Is each opcode equal to a cycle? I believe so 
    // I feel like I don't really need to keep track of this until later
    // cycles: u32,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0, ac: 0, x: 0, y: 0, sr: StatusRegister::empty(), sp: 0xff   , 
            memory: [0; MEMSIZE],
        }
    }
    
    pub fn reset(&mut self) {
        // Reset the CPU
        self.pc = 0;
        self.ac = 0;
        self.x = 0;
        self.y = 0;
        self.sr = StatusRegister::empty();
        self.sp = 0xff;
        self.memory = [0; MEMSIZE];
    }

    // TODO: should these implementations be somewhere else?
    fn set_status(&mut self, flag: StatusRegister) {
        self.sr.insert(flag);
    }

    fn clear_status(&mut self, flag: StatusRegister) {
        self.sr.remove(flag);
    }

    fn get_status(&mut self, flag: StatusRegister) -> bool {
        self.sr.contains(flag)
    }

    // grab the bytecode?
    // no need to bee immutable i think 
    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc += 1;
        byte 
    }

    // TODO: maybe move this code somewhere else instead?
    fn lda_set_status(&mut self) {
        if self.ac == 0 {
            self.set_status(StatusRegister::Z);
        }
        if self.get_status(StatusRegister::V) {
            self.set_status(StatusRegister::N);
        }
    }


    pub fn execute(&mut self) {
        // Execute an instruction
        // TODO: separate this into load only? i.e. a section of code that only does load
        // A giant switch table does not feel right 
        // Very disorganized; better to separate it out into libraries specific for each type instruction
        let opcode = self.fetch_byte();
        match opcode {
            LDA => { // load immediate opcode
                let value = self.fetch_byte();
                self.ac = value;
                self.lda_set_status();
            }
            // load the accumulator with the value at the zero page?
            LDA_ZPG => {
                let value = self.fetch_byte() + self.x;
                let deref = self.memory[value as usize];
                self.ac = deref; 
                self.lda_set_status();
            }
            LDA_ZPX => {
                let value = self.fetch_byte();
                let deref = self.memory[value as usize];
                self.ac = deref;
                self.lda_set_status();
            }
            LDA_ABS => {

            }
            LDA_ABX => {

            }
            LDA_ABY => {

            }
            JSR => {

            }
            _   => println!("Instruction not found")
        }
    }
}

fn main() {
    println!("hello world");
    let mut cpu = CPU::new();
    cpu.reset();
    cpu.memory[0x0] = 0xA9;
    cpu.memory[0x1] = 0x41;
    cpu.execute();
    println!("Accumulator: {}", cpu.ac);
}




