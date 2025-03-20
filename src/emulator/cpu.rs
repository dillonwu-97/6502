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
use crate::emulator::flag_arr;
// use crate::emulator::handlers::incdec::idc;


// TODO: should I find some way to handle the invalid bits?
pub const C: u8 = 1 << 0;
pub const Z: u8 = 1 << 1;
pub const I: u8 = 1 << 2;
pub const D: u8 = 1 << 3;
pub const B: u8 = 1 << 4;
pub const U: u8 = 1 << 5; 
pub const V: u8 = 1 << 6;
pub const N: u8 = 1 << 7;

// I wonder if there is a better way to do this or nah 
pub const MEMSIZE: usize = 2 << 16;
bitflags! {
    pub struct StatusRegister: u8 {
        const C = C; // carry
        const Z = Z; // zero
        const I = I; // interrupt
        const D = D; // decimal 
        const B = B; // break
        const U = U; // unused 5th bit
        const V = V; // overflow
        const N = N; // negative
    }
}

pub struct CPU {
    pub memory: [u8; MEMSIZE],
    pub pc: u16,
    pub ac: u8, // accumulator
    pub x: u8,
    pub y: u8,
    pub sr: StatusRegister, // bitfield for the status bits
    pub sp: u8,
    pub optable: Vec<OpWrapper>,
    pub cycle_count: u64,
    pub boundary_flag: bool, // page boundary bit that gets set when we cross a page boundary
}

impl CPU {
    pub fn new() -> Self {
        let mut op_vec: Vec<OpWrapper> = Vec::with_capacity(256);
        for i in 0..256 {
            let new_wrapper: OpWrapper = OpWrapper::new(
                opcode_arr[i],
                addrmode_arr[i],
                cycle_arr[i],
                inst_arr[i],
                flag_arr[i]
            );
            op_vec.push(new_wrapper);
        }
        Self {
            pc: 0, ac: 0, x: 0, y: 0, sr: StatusRegister::empty(), sp: 0xff, 
            memory: [0; MEMSIZE],
            optable: op_vec,
            cycle_count: 0,
            boundary_flag: false
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

    pub fn update(&mut self, pc: u16, ac: u8, x: u8, y: u8, sr: u8, sp: u8) {
        self.pc = pc;
        self.ac = ac;
        self.x = x;
        self.y = y;
        // println!("value as u8: {}", sr);
        self.sr = StatusRegister::from_bits_truncate(sr); // ignore the invalid bits, but not sure
        // if this is the right way
        self.sp = sp;
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

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
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
    pub fn imm(&mut self) -> usize {
        let ret = self.pc as usize;
        self.pc += 1;
        return ret;
    }

    /*
    * Operates directly on the accumulator 
    *
    * Four instructions that operate in this mode: ASL, ROL, LSR, ROR
    * TODO: special case to separate this out / THIS IS WRONG
    */
    pub fn acc(&mut self) -> usize {
        return usize::MAX;
    }

    /*
    * Zero Page addressing mode
    *
    * Returns reference to memory 
    */
    pub fn zpg(&mut self) -> usize {
        return self.fetch_byte() as usize;
    }

    /* 
    * Zero Page, X addressing mode
    */
    pub fn zpx(&mut self) -> usize {
        return self.fetch_byte().wrapping_add( self.x as u8) as usize;
    }

    /* 
    * Zero Page, Y addressing mode
    */
    pub fn zpy(&mut self) -> usize {
        // TODO: check if page boundary has been crossed
        return self.fetch_byte().wrapping_add( self.y as u8) as usize;
    }

    /*
    * Relative addressing mode
    *
    * Returns a value from -128 -> 127 as usize
    */
    pub fn rel(&mut self) -> usize {
        return self.fetch_byte() as usize;
    }

    /*
    * Abs addressing mode
    */
    pub fn abs(&mut self) -> usize {
        return self.fetch_two() as usize;
    }

    pub fn abx(&mut self) -> usize {
        let addend = self.fetch_two();
        let val = addend.wrapping_add(self.x as u16);
        if ((val >> 0x8) != (addend >> 0x8)) {
            self.boundary_flag = true;
        }
        return val as usize;
    }

    pub fn aby(&mut self) -> usize {
        let addend = self.fetch_two();
        let val = addend.wrapping_add(self.y as u16);
        if ((val >> 0x8) != (addend >> 0x8)) {
            self.boundary_flag = true;
        }
        return val as usize;
    }

    // Absolute indirect
    //
    // This is only used for jumps instructions. The returned value is the lower
    // byte of the jump address
    // The next byte afterwards is the upper byte of the jump address
    // TODO: check the pc count for this
    pub fn ind(&mut self) -> usize {
        let lower = self.fetch_byte() as u16;   
        let upper = self.fetch_byte() as u16;
        let jmp_addr = lower + (upper << 8);
        return jmp_addr as usize;
    }

    // TODO: edge case: both memory locations (upper / lower) specifying the high and low order
    // bytes of the effective address must be in page zero 
    // e.g. zp_addr = 0xff
    // TODO: add a test case that handles the edge case
    // need to fix this
    pub fn idx(&mut self) -> usize {
        let zp_addr = self.fetch_byte().wrapping_add(self.x);  // zero page addr
        let lower = self.memory[ zp_addr as usize ] as u16;
        let upper = self.memory[ zp_addr.wrapping_add(0x1) as usize ] as u16;
        let addr = (upper << 0x8) + lower;
        return addr as usize;
    }
    
    /*
    * https://stackoverflow.com/questions/46262435/indirect-y-indexed-addressing-mode-in-mos-6502
    */
    pub fn idy(&mut self) -> usize {
        let zpg_addr = self.fetch_byte();
        let mut lower = self.memory[ zpg_addr as usize ] as u16;
        let mut upper = self.memory[ zpg_addr.wrapping_add(0x01) as usize ] as u16; // wrapping add
        // for the next mem location to read the high byte
        lower += self.y as u16;
        if (lower > 0xff) {
            upper += 0x1;  
            upper &= 0xff; // in case there is an overflow
            self.boundary_flag = true;
        }
        upper <<= 0x8;
        return ((lower & 0xff) + upper) as usize;
    }

    // TODO: replace with None / Error out if the last case fails 
    // TODO: handle illegal opcodes, and the None case which implies that the opcode is not
    // illegal, and that we don't need to use it 
    pub fn addr_mode_handler(&mut self, op: u8) -> usize {
        let idx = op;
        let addr_mode = self.optable[op as usize].addr_mode;
        match addr_mode {
            a if a == AddrMode::IMM => self.imm(),
            a if a == AddrMode::ACC => self.acc(),
            a if a == AddrMode::ZPG => self.zpg(),
            a if a == AddrMode::ZPX => self.zpx(),
            a if a == AddrMode::ZPY => self.zpy(),
            a if a == AddrMode::REL => self.rel(),
            a if a == AddrMode::ABS => self.abs(),
            a if a == AddrMode::ABX => self.abx(),
            a if a == AddrMode::ABY => self.aby(),
            a if a == AddrMode::IDX => self.idx(),
            a if a == AddrMode::IDY => self.idy(),
            a => usize::MAX, // TODO: need to fix this
        }
    }

    pub fn handle_dispatch(&mut self, op: u8) {
        let cur: Inst = self.optable[op as usize].inst;
        self.cycle_count += self.optable[op as usize].cycle as u64;
        match cur {
            // Load operations
            Inst::LDA | Inst::LDX | Inst::LDY => {
                let idx: usize = self.addr_mode_handler(op);
                let mem_val = self.memory[idx];
                self.ld(cur, mem_val);
                if (self.boundary_flag) {
                    self.cycle_count +=1;
                }
            }

            // Store operations
            Inst::STA | Inst::STX | Inst::STY => {
                let to_store = self.st(cur);
                let idx: usize = self.addr_mode_handler(op);
                self.memory[idx] = to_store;

            }

            // Register Transfers
            // These isntructions are implied addressing mode instructions so we dont need to use
            // the addressing mode information
            Inst::TAX | Inst::TAY | Inst::TXA | Inst::TYA => {
                self.tx(cur);
            }

            // Stack operations
            // Also implied addressing mode operations
            Inst::TSX | Inst::TXS | Inst::PHA | Inst::PHP | Inst::PLA | Inst::PLP => {
                self.stack(cur); 
            }

            // Logical operations
            // We need to check the boundary flags for these
            Inst::AND | Inst::EOR | Inst::ORA | Inst::BIT => {
                let idx: usize = self.addr_mode_handler(op);
                let mem_val = self.memory[idx];
                self.log(cur, mem_val);
                if (self.boundary_flag) {
                    self.cycle_count += 1;
                }
            }

            // Arithmetic operations
            Inst::ADC | Inst::SBC | Inst::CMP | Inst::CPX | Inst::CPY => {
                
                let idx: usize = self.addr_mode_handler(op);
                let mem_val: u8 = self.memory[idx];
                self.ath(cur, mem_val);
                if (self.boundary_flag) { // set in the address mode handler
                    self.cycle_count += 1;
                }
            }
            
            // Increment / Decrement operations
            Inst::INC | Inst::INX | Inst::INY | Inst::DEC | Inst::DEX | Inst::DEY => {
                // no boundary check needed
                // TODO: figure out a way to move this to its own separate file without pissing off
                // the borrow checker; kind of hard though
                let idx: usize = self.addr_mode_handler(op);
                self.idc(cur, idx);
            }

            // Shift operations
            // only shift operations contains accumulator mode instructions
            Inst::ASL | Inst::LSR | Inst::ROL | Inst::ROR => {
                let idx: usize = self.addr_mode_handler(op);
                // let mem_val: u8 = self.memory[idx];
                if (idx == usize::MAX) {
                    self.sh(cur, idx, true); 
                } else {
                    self.sh(cur, idx, false);
                }
            }

            // TODO: not sure if this is the best way to handle RTS actually
            Inst::JMP | Inst::JSR => {
                // jmp = absolute / indirect, jsr = absolute, rts = implied
                // absolute jmps means the idx value is not actually and idx, it's the value we
                // want to jump to 
                let jump_addr: usize = self.addr_mode_handler(op); 
                self.jmp(cur, jump_addr);
            }

            Inst:: RTS => {
                self.jmp(cur, usize::MAX);
            }

            Inst::BCC | Inst::BCS | Inst::BEQ | Inst::BMI | Inst::BNE | Inst::BPL | Inst::BVC | Inst::BVS => {
                let offset: i8 = self.addr_mode_handler(op) as i8;
                let orig_pc: u16 = self.pc - 2;
                assert!(offset >= -128);
                assert!(offset <= 127);
                self.branch(cur, orig_pc, offset);
            }

            Inst::CLC | Inst::CLD | Inst::CLI | Inst::CLV | Inst::SEC | Inst::SED | Inst::SEI => {
                self.stats(cur);
            }

            Inst::BRK | Inst::NOP | Inst::RTI => {
                self.sys(cur);
            }

            _ => {return; }
        }
        self.boundary_flag = false;
    }

    /*
    * Execute instructions
    */
    pub fn execute(&mut self) {
        // Need to return true / false for everything I think 
        // continue executing?
        let opcode = self.fetch_byte();
        self.handle_dispatch(opcode);
    }
}
