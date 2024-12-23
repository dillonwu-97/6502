use bitflags::bitflags;
pub(crate) const MEMSIZE: usize = 2 << 16;
bitflags! {
    pub(crate) struct StatusRegister: u8 {
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

pub struct CPU {

    pub memory: [u8; MEMSIZE],
    pub(crate) pc: u16,
    pub(crate) ac: u8, // accumulator
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) sr: StatusRegister, // bitfield for the status bits
    // maybe define some constants and then do the | operation? 
    // not really sure lol 
    pub(crate) sp: u8,

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
    pub(crate) fn set_status(&mut self, flag: StatusRegister) {
        self.sr.insert(flag);
    }

    pub(crate) fn clear_status(&mut self, flag: StatusRegister) {
        self.sr.remove(flag);
    }

    pub(crate) fn get_status(&mut self, flag: StatusRegister) -> bool {
        self.sr.contains(flag)
    }

    // grab the bytecode?
    // no need to bee immutable i think 
    pub(crate) fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc += 1;
        byte 
    }

    pub(crate) fn fetch_two(&mut self) -> u16 {
        let lower = self.fetch_byte() as u16;
        let upper = self.fetch_byte() as u16;
        let value = lower + (upper << 8);
        return value;
    }

    // TODO: maybe move this code somewhere else instead?
    pub fn execute(&mut self) {
        // Need to return true / false for everything I think 
        let opcode = self.fetch_byte();
        self.ld_exec(opcode) || self.st_exec(opcode) || self.tx_exec(opcode) || self.stack_exec(opcode);
    }
}
