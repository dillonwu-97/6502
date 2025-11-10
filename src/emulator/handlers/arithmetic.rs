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

    fn cmp_status(&mut self, reg: u8, mem_val: u8) {
        if (reg >= mem_val) {
            self.set_status(StatusRegister::C);
        } else {
            self.clear_status(StatusRegister::C);
        }         
        if (reg == mem_val) {
            self.set_status(StatusRegister::Z);
        } else {
            self.clear_status(StatusRegister::Z);
        }
        // TODO: not sure what the expected behavior for this would be 
        // reg = 0x0, mem_val = 255 since 0 - 255 = 1 in the u8 space
        // a comparison like 0 < 255 -> set the flag, but if we do the subtraction and get the
        // result, we DON'T set the flag 
        // will probs need to revisit this when i build up the test cases
        let res: u8 = reg.wrapping_sub(mem_val);
        if (res >> 7 == 1) {
            self.set_status(StatusRegister::N);
        }

    }

    pub fn ath(&mut self, inst: Inst, mem_val: u8) {
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

                let i_a: i8 = mem_val as i8; 
                let i_b: i8 = self.ac as i8;
                let i_res: i8 = u_res as i8; // cut off the top byte
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

            // in the role of subtraction, the role of carry is inverted
            // A = A - M - (Cnot) where Cnot is "not carry", so take the carry and invert it 
            Inst::SBC => {
                let u_a: u16 = self.ac as u16;
                let u_b: u16 = mem_val as u16;
                // TODO: need to double check that this does what we think it does (the inverse)
                let u_res: u16 = u_a - u_b - ((!self.get_status(StatusRegister::C)) as u16);
                if (u_res >> 8 == 1) {
                    self.set_status(StatusRegister::C);
                } else {
                    self.clear_status(StatusRegister::C);
                }

                let i_a: i8 = self.ac as i8;
                let i_b: i8 = mem_val as i8;
                let i_res: i8 = u_res as i8;
                // negative minus positive number -> negative
                // positive minus negative number -> positive
                // n - n = + or neg (shouldn't overflow)
                if (i_a < 0 && i_b > 0) {
                    if (i_res > i_a) {
                        self.set_status(StatusRegister::V);
                    }
                } else if (i_a > 0 && i_b < 0) {
                    if (i_res < i_a) {
                        self.set_status(StatusRegister::V);
                    }
                }

                self.ac = i_res as u8;
                if (self.ac == 0) {
                    self.set_status(StatusRegister::Z);
                }

                if (self.ac >> 7 == 1) {
                    self.set_status(StatusRegister::N);
                }

                // negative negative
                // positive positive
                // is there a way to use the addition code for the subtraction case?
                // is there a way to do addition with just bit flips instead of the method shown above?
                // technically, the & operation is addition but i dont want to go back and think through the optimization at the moment 
                // maybe not 
                // ok for this at least, carry is set when we have something like -128 + -128 ->
                // -256 so if the result is > a 

            }
            Inst::CMP => {
                // should the comparison be u8 or i8?
                // i kind of just assumed all the register were just u8 values
                // do self.ac - mem so the comparison should be made arithmetically 
                self.cmp_status(self.ac, mem_val);
            }
            Inst::CPX => {
                self.cmp_status(self.x, mem_val);
            }
            Inst::CPY => {
                self.cmp_status(self.y, mem_val);
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
