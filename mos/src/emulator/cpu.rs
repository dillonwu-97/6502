use bitflags::bitflags;
use crate::emulator::Opcode;
use crate::emulator::AddrMode;

// I wonder if there is a better way to do this or nah 
pub const MEMSIZE: usize = 2 << 16;
bitflags! {
    pub struct StatusRegister: u8 {
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
    pub pc: u16,
    pub ac: u8, // accumulator
    pub x: u8,
    pub y: u8,
    pub sr: StatusRegister, // bitfield for the status bits
    // maybe define some constants and then do the | operation? 
    // not really sure lol 
    pub sp: u8,

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
    pub fn set_status(&mut self, flag: StatusRegister) {
        self.sr.insert(flag);
    }

    pub fn clear_status(&mut self, flag: StatusRegister) {
        self.sr.remove(flag);
    }

    pub fn get_status(&mut self, flag: StatusRegister) -> bool {
        self.sr.contains(flag)
    }

    // grab the bytecode?
    // no need to bee immutable i think 
    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc += 1;
        byte 
    }

    pub fn fetch_two(&mut self) -> u16 {
        let lower = self.fetch_byte() as u16;
        let upper = self.fetch_byte() as u16;
        let value = lower + (upper << 8);
        return value;
    }

    /*
    * TODO: Accumulator addressing mode
    */

    /*
    * Immediate addressing mode
    */
    pub fn imm(&mut self) -> u8 {
        return self.fetch_byte();
    }

    /*
    * Zero Page addressing mode
    */
    pub fn zpg(&mut self) -> &mut u8 {
        return &mut self.memory[ self.fetch_byte() as usize ];
    }

    /* 
    * Zero Page, X addressing mode
    */
    pub fn zpx(&mut self) -> &mut u8 {
        return &mut self.memory[ self.fetch_byte().wrapping_add(self.x as u8) as usize ];
    }

    pub fn zpy(&mut self) -> &mut u8 {
        return &mut self.memory[ self.fetch_byte().wrapping_add( self.y as u8) as usize ]; 
    }

    pub fn abs(&mut self) -> &mut u8 {
        return &mut self.memory[ self.fetch_two() as usize ];
    }

    pub fn abx(&mut self) -> &mut u8 {
        let val = self.fetch_two() + (self.x as u16);
        return &mut self.memory[ val as usize ];
    }

    pub fn aby(&mut self) -> &mut u8 {
        let val = self.fetch_two() + (self.y as u16);
        return &mut self.memory[ val as usize ];
    }

    // TODO: replace with None / Error out if the last case fails 
    pub fn addr_mode_handler(&mut self, op: &Opcode) -> &mut u8 {
        let addr_mode = Opcode::get_addr_mode(op);
        match addr_mode {
            x if x == Some(AddrMode::ZPG) => self.zpg(),
            x if x == Some(AddrMode::ZPX) => self.zpx(),
            x if x == Some(AddrMode::ZPY) => self.zpy(),
            x if x == Some(AddrMode::ABS) => self.abs(),
            x if x == Some(AddrMode::ABX) => self.abx(),
            x if x == Some(AddrMode::ABY) => self.aby(),
            _ => &mut self.memory[0x0],
        }
    }

    pub fn handle_dispatch(&mut self, op: &Opcode) {
        match op {
            x if self.is_ld_opcode(x) => {
                let mem_ref: &mut u8 = self.addr_mode_handler(op); 
                let mem_val = *mem_ref;
                self.ld_handle_dispatch(op, mem_val);
            }
            _ => {return; }
        }
    }


    // TODO: maybe move this code somewhere else instead?
    pub fn execute(&mut self) {
        // Need to return true / false for everything I think 
        let opcode = self.fetch_byte();
        // self.ld_exec(opcode) || self.st_exec(opcode) || self.tx_exec(opcode) || self.stack_exec(opcode);
        // self.ld_exec(opcode);
    }
}
