// Handles transfer operations

#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;


impl CPU {

    pub(crate) fn tx_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 { // check if 7th bit is set
            self.set_status(StatusRegister::N);
        }
    }

    pub(crate) fn tx_exec(&mut self, opcode: u8) -> bool {
        let op = Opcode::from(opcode);
        match op {
            Opcode::TAX => {
                self.x = self.ac;
                self.tx_set_status(self.x);
            }
            Opcode::TAY => {
                self.y = self.ac;
                self.tx_set_status(self.y);
            }
            Opcode::TSX => {
                self.x = self.sp;
                self.tx_set_status(self.x);
            }
            Opcode::TXA => {
                self.ac = self.x;
                self.tx_set_status(self.ac);
            }
            Opcode::TXS => {
                self.sp = self.x;
            }
            Opcode::TYA => {
                self.ac = self.y;
                self.tx_set_status(self.ac);
            }
            _ => {
                return false;
            }
        }
        true
    }
}
