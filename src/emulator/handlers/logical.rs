#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;
use crate::emulator::Inst;
use crate::emulator::cpu::N;
use crate::emulator::cpu::V;

impl CPU {

    fn log_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }     
        if register >> 7 == 1 {
            self.set_status(StatusRegister::N);
        }
    }

    // val is val held in memory
    pub fn log(&mut self, inst: Inst, val: u8) {
        match inst {
            Inst::AND => {
                self.ac &= val;     
                self.log_set_status(self.ac);
            } ,
            Inst::EOR => {
                self.ac ^= val;
                self.log_set_status(self.ac);
            }
            Inst::ORA => {
                self.ac |= val;
                self.log_set_status(self.ac);
            }
            Inst::BIT => {
                if (self.ac & val == 0) {
                    self.set_status(StatusRegister::Z);
                }

                if (val & N == 1) { self.set_status(StatusRegister::N) } else { self.clear_status(StatusRegister::N) };
                if (val & V == 1) { self.set_status(StatusRegister::V) } else { self.clear_status(StatusRegister::V) };
            }
            _ => {return;}

        }
    }

}
