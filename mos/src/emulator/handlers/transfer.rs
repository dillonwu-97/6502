// Handles transfer operations

#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Inst;


impl CPU {

    pub fn tx_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 { // check if 7th bit is set
            self.set_status(StatusRegister::N);
        }
    }

    pub fn tx(&mut self, inst: Inst) {
        match inst {
            Inst::TAX => {
                self.x = self.ac;
                self.tx_set_status(self.x);
            }
            Inst::TAY => {
                self.y = self.ac;
                self.tx_set_status(self.y);
            }
            Inst::TXA => {
                self.ac = self.x;
                self.tx_set_status(self.ac);
            }
            Inst::TYA => {
                self.ac = self.y;
                self.tx_set_status(self.ac);
            }
            _ => panic!("Should not reach this instruction")
        } 
    } 
}
