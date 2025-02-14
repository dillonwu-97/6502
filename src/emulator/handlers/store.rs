#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::Inst;

impl CPU {

    pub fn st(&mut self, inst: Inst) -> u8 {
        match inst {
            Inst::STA => {
                return self.ac;
            }
            Inst::STX => {
                return self.x;
            }
            Inst::STY => {
                return self.y;
            }
            _ => panic!("Should never reach this case"),
        }

    }
}
