#![allow(non_snake_case)]
#[cfg(test)]
#[path = "load.test.rs"]
mod tests;
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;
use crate::emulator::AddrMode;
use crate::emulator::Inst;

impl CPU {

    // TODO: get rid of this to reduce redundancy
    // TODO add LDA / LDX / LDY / etc. 
    
    pub fn is_ld_opcode(&mut self, op: Opcode) {

    }

    fn ld_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 { // check if 7th bit is set
            self.set_status(StatusRegister::N);
        }
    }

    pub fn ld(&mut self, inst: Inst, val: u8) {
        match inst {
            Inst::LDA => {
                self.ac = val; 
                self.ld_set_status(self.ac);
            }
            Inst::LDX => {
                self.x = val;
                self.ld_set_status(self.x);
            }
            Inst::LDY => {
                self.y = val;
                self.ld_set_status(self.y);
            }
            _ => {return;}
        }
    }
}
