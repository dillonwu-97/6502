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

pub const C: u8 = 1 << 0;
pub const Z: u8 = 1 << 1;
pub const I: u8 = 1 << 2;
pub const D: u8 = 1 << 3;
pub const B: u8 = 1 << 4;
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
                    // unused 5th bit
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
    // TODO: add cycle check for this as well 
    
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
        return &mut self.memory[ self.fetch_byte().wrapping_add( self.x as u8) as usize ]; 
    }

    /* 
    * Zero Page, Y addressing mode
    */
    pub fn zpy(&mut self) -> &mut u8 {
        // TODO: check if page boundary has been crossed
        return &mut self.memory[ self.fetch_byte().wrapping_add( self.y as u8) as usize ]; 
    }

    /*
    * Abs addressing mode
    */
    pub fn abs(&mut self) -> &mut u8 {
        return &mut self.memory[ self.fetch_two() as usize ];
    }

    pub fn abx(&mut self) -> &mut u8 {
        // TODO: check if page boundary has been crossed
        let addend = self.fetch_two();
        let val = addend.wrapping_add(self.x as u16);
        if ((val >> 0x8) != (addend >> 0x8)) {
            self.boundary_flag = true;
        }
        return &mut self.memory[ val as usize ];
    }

    pub fn aby(&mut self) -> &mut u8 {
        let addend = self.fetch_two();
        let val = addend.wrapping_add(self.y as u16);
        if ((val >> 0x8) != (addend >> 0x8)) {
            self.boundary_flag = true;
        }
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
    // TODO: add page boundary checker
    pub fn idy(&mut self) -> &mut u8 {
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
            a if a == AddrMode::ACC => self.acc(),
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
        self.cycle_count += self.optable[op as usize].cycle as u64;
        match cur {
            // Load operations
            Inst::LDA | Inst::LDX | Inst::LDY => {
                let mem_ref: &mut u8 = self.addr_mode_handler(op);
                let mem_val = *mem_ref;
                self.ld(cur, mem_val);
                if (self.boundary_flag) {
                    self.cycle_count +=1;
                }
            }

            // Store operations
            Inst::STA | Inst::STX | Inst::STY => {
                let to_store = self.st(cur);
                let mem_ref: &mut u8 = self.addr_mode_handler(op);
                *mem_ref = to_store;
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
                let mem_ref: &mut u8 = self.addr_mode_handler(op);
                let mem_val: u8 = *mem_ref;
                self.log(cur, mem_val);
                if (self.boundary_flag) {
                    self.cycle_count += 1;
                }
            }

            // Arithmetic operations
            Inst::ADC | Inst::SBC | Inst::CMP | Inst::CPX | Inst::CPY => {
                
                let mem_ref: &mut u8 = self.addr_mode_handler(op);
                let mem_val: u8 = *mem_ref;
                self.ath(cur, mem_val);
                if (self.boundary_flag) { // set in the address mode handler
                    self.cycle_count += 1;
                }
            }

            // Shift operations
            Inst::ASL | Inst::LSR | Inst::ROL | Inst::ROR => {
                // TODO: add addressing mode for accumulator 
                // the return value for the handler might be different depending on if it's the
                // accumulator actually, so we might need a special case to handle this?
                // unless we can get a reference to self.ac itself
                // because it can be both accumulator or memory location, we need to return the
                // value from the function to use it here
                // - [ ] 
                let mem_ref: &mut u8 = self.addr_mode_handler(op);
                let mem_val: u8 = *mem_ref;
                self.sh(cur, mem_val); 
            }

            
            // Increment / Decrement operations
            Inst::INC | Inst::INX | Inst::INY | Inst::DEC | Inst::DEX | Inst::DEY => {
                // no boundary check needed
                // TODO: figure out a way to move this to its own separate file without pissing off
                // the borrow checker; kind of hard though
                let x: u8 = self.x;
                let y: u8 = self.y;
                let mem_ref: &mut u8 = self.addr_mode_handler(op);
                // let a = self.idc(1);
                // self.idc(1);
                // *mem_ref = 1;
                // *mem_ref = a;
                // it was ok because we didn't use the borrow i think 
                // in theory, we should be able to do the borrow inside self.idc to bypass this
                // issue?
                //
                match cur { 
                    Inst::INC => {
                        *mem_ref += 1;
                        let mem_val = *mem_ref;
                        self.idc_set_status(mem_val);
                    },
                    Inst::INX => {
                        self.x += 1; 
                        self.idc_set_status(self.x);
                    },
                    Inst::INY => {
                        self.y += 1;
                        self.idc_set_status(self.y);
                    },
                    Inst::DEC => {
                        *mem_ref -= 1;
                        let mem_val = *mem_ref;
                        self.idc_set_status(mem_val);
                    },
                    Inst::DEX => {
                        self.x -= 1;
                        self.idc_set_status(self.x);
                    },
                    Inst::DEY => {
                        self.y -= 1;
                        self.idc_set_status(self.y);
                    },
                    _ => {return; }
                }
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
