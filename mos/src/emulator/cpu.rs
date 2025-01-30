use bitflags::bitflags;
use crate::emulator::Opcode;
use crate::emulator::AddrMode;
use crate::emulator::Inst;
use crate::emulator::OpWrapper;

// TODO: is there a different way to using all these different functions in the same module?
use crate::emulator::inst_arr;
use crate::emulator::cycle_arr;
use crate::emulator::addrmode_arr;
use crate::emulator::opcode_arr;

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
    pub optable: Vec<OpWrapper>,

    // TODO: What should the size of this be?
    // TODO: Is each opcode equal to a cycle? I believe so 
    // I feel like I don't really need to keep track of this until later
    // cycles: u32,
}

impl CPU {
    pub fn new() -> Self {
        let mut op_vec: Vec<OpWrapper> = Vec::with_capacity(256);
        for i in 0..256 {
            let new_wrapper: OpWrapper = OpWrapper::new(
                opcode_arr[i],
                addrmode_arr[i],
                cycle_arr[i],
                inst_arr[i]
            );
            op_vec.push(new_wrapper);
        }
        Self {
            pc: 0, ac: 0, x: 0, y: 0, sr: StatusRegister::empty(), sp: 0xff, 
            memory: [0; MEMSIZE],
            optable: op_vec 
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

    // All addressing mode functions should be returning some memory address to be used
    // Branch instructions are all relative so REL is not handled
    
    /*
    * Addressing mode functions should be grabbing that which we are reading from / writing to
    */

    /*
    * Immediate addressing mode
    */
    pub fn imm(&mut self) -> &mut u8 {
        let ret = &mut self.memory[ self.pc as usize ];
        self.pc += 1;
        return ret;
    }

    /*
    * Operates directly on the accumulator 
    *
    * Four instructions that operate in this mode: ASL, ROL, LSR, ROR
    */
    pub fn acc(&mut self) -> &mut u8 {
        return &mut self.ac 
    }

    /*
    * Zero Page addressing mode
    *
    * Returns reference to memory 
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
        let val = self.fetch_two().wrapping_add(self.x as u16);
        return &mut self.memory[ val as usize ];
    }

    pub fn aby(&mut self) -> &mut u8 {
        let val = self.fetch_two().wrapping_add(self.y as u16);
        return &mut self.memory[ val as usize ];
    }

    // Absolute indirect
    //
    // This is only used for jumps instructions. The returned value is the lower
    // byte of the jump address
    // The next byte afterwards is the upper byte of the jump address
    // TODO: check the pc count for this
    pub fn ind(&mut self) -> &mut u8 {
        let lower = self.fetch_byte() as u16;   
        let upper = self.fetch_byte() as u16;
        let jmp_addr = lower + (upper << 8);
        return &mut self.memory[ jmp_addr as usize ];
    }

    // TODO: edge case: both memory locations (upper / lower) specifying the high and low order
    // bytes of the effective address must be in page zero 
    // e.g. zp_addr = 0xff
    // TODO: add a test case that handles the edge case
    // need to fix this
    pub fn idx(&mut self) -> &mut u8 {
        let zp_addr = self.fetch_byte().wrapping_add(self.x);  // zero page addr
        let lower = self.memory[ zp_addr as usize ] as u16;
        let upper = self.memory[ zp_addr.wrapping_add(0x1) as usize ] as u16;
        let addr = (upper << 0x8) + lower;
        return &mut self.memory[ addr as usize ];
    }
    
    /*
    * https://stackoverflow.com/questions/46262435/indirect-y-indexed-addressing-mode-in-mos-6502
    * TODO: if wrapping_add then cycle count goes up by 1 probs
    */
    pub fn idy(&mut self) -> &mut u8 {
        let zpg_addr = self.fetch_byte() as u16;
        let mut lower = self.memory[ zpg_addr as usize ] as u16;
        let mut upper = self.memory[ zpg_addr.wrapping_add(0x01) as usize ] as u16;
        lower += self.y as u16;
        if (lower > 0xff) {
            upper += 0x1;  
            upper &= 0xff; // in case there is an overflow
        }
        upper <<= 0x8;
        return &mut self.memory[((lower & 0xff) + upper) as usize];
    }

    // TODO: replace with None / Error out if the last case fails 
    // TODO: handle illegal opcodes, and the None case which implies that the opcode is not
    // illegal, and that we don't need to use it 
    pub fn addr_mode_handler(&mut self, op: u8) -> &mut u8 {
        
        let idx = op;
        let addr_mode = self.optable[op as usize].addr_mode;
        match addr_mode {
            a if a == AddrMode::IMM => self.imm(),
            a if a == AddrMode::ZPG => self.zpg(),
            a if a == AddrMode::ZPX => self.zpx(),
            a if a == AddrMode::ZPY => self.zpy(),
            a if a == AddrMode::ABS => self.abs(),
            a if a == AddrMode::ABX => self.abx(),
            a if a == AddrMode::ABY => self.aby(),
            a if a == AddrMode::IDX => self.idx(),
            a if a == AddrMode::IDY => self.idy(),
            a => &mut self.memory[0x0], // TODO: need to fix this
        }
    }

    pub fn handle_dispatch(&mut self, op: u8) {
        let cur: Inst = self.optable[op as usize].inst;
        match cur {
            Inst::LDA | Inst::LDX | Inst::LDY => {
                let mem_ref :&mut u8 = self.addr_mode_handler(op);
                let mem_val = *mem_ref;
                self.ld(cur, mem_val);
            }
            _ => {return; }
        }
    }

    // TODO: maybe move this code somewhere else instead?
    pub fn execute(&mut self) {
        // Need to return true / false for everything I think 
        // continue executing?
        
        let opcode = self.fetch_byte();
        self.handle_dispatch(opcode);
    }
}
