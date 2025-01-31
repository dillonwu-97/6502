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

    fn ld_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 { // check if 7th bit is set
            self.set_status(StatusRegister::N);
        }
    }

    pub fn ld(&mut self, op: u8, val: u8) {
        self.cycle_count += self.optable[op as usize].cycle as u64;
        if (self.boundary_flag) {
            self.cycle_count +=1;
        }
        let inst = self.optable[op as usize].inst;
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
