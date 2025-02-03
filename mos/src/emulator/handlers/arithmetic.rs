#[cfg(test)]
#[path = "arithmetic.test.rs"]
mod tests;
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;
use crate::emulator::Inst;

use crate::emulator::cpu::N;
use crate::emulator::cpu::Z;
use crate::emulator::cpu::C;
use crate::emulator::cpu::V;

impl CPU {

    pub fn ath(&mut self, inst: Inst, mem_val: u8) {
        if (self.boundary_flag) {
            self.cycle_count += 1;
        }
        match inst {
            Inst::ADC => {
                // carry flag is set if the addition bleeds out
                let u_a: u16 = mem_val as u16;
                let u_b: u16 = self.ac as u16;
                let u_res: u16 = u_a + u_b + (self.get_status(StatusRegister::C) as u16);
                if ((u_res >> 8) == 1) {
                    self.set_status(StatusRegister::C);
                } else {
                    self.clear_status(StatusRegister::C);
                }

                let i_a: i8 = mem_val as i8; // cut off the top byte
                let i_b: i8 = self.ac as i8;
                let i_res: i8 = u_res as i8;
                if (i_a > 0 && i_b > 0) { // both positive
                    if (i_res < i_a) {
                        self.set_status(StatusRegister::V);
                    }
                } else if (i_a < 0 && i_b < 0) { // both negative
                    if (i_res > i_a) {
                        self.set_status(StatusRegister::V);
                    }
                }

                self.ac = i_res as u8;
                
                // zero handle 
                if (self.ac == 0) {
                    self.set_status(StatusRegister::Z);
                }

                // negative handle
                if (self.ac >> 7 == 1) {
                    self.set_status(StatusRegister::N);
                }
    

            }
            Inst::SBC => {
            }
            Inst::CMP => {

            }
            Inst::CPX => {

            }
            Inst::CPY => {

            }
            _=>{return;}
        }
    }

}

// TODO: unit tests
// 1. carry and over flow check for each of these 
// 1+1 -> c = 0, v = 0
// 1 + -1 -> c = 1, v = 0
// 127 + 1 -> c = 0, v = 1
// -128 + -1 -> c = 1, v = 1
// 127 + -1 -> c = 1, v = 0 i think 
// -128 + 1 -> c = 0, v = 0 
// 127 + 127 -> c = 0, v = 1
