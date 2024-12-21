#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;

impl CPU {
    
    // Returns found_opcode or not
    pub(crate) fn st_exec(&mut self, opcode: u8) -> bool {
        return self.st_acc(opcode) || self.st_x(opcode) || self.st_y(opcode);
    }

    pub(crate) fn st_acc(&mut self, opcode: u8) -> bool {
        let op = Opcode::from(opcode);
        match op {
            Opcode::STA_ZPG => {
                self.memory[ self.fetch_byte() as usize ] = self.ac;
            }
            Opcode::STA_ZPX => {
                let addr = self.fetch_byte() + self.x;
                self.memory[ addr as usize ] = self.ac;
            }
            Opcode::STA_ABS => {
                self.memory[ self.fetch_two() as usize ] = self.ac;
            }
            Opcode::STA_ABX => {
                let addr = self.fetch_two() + (self.x as u16);
                self.memory[ addr as usize ] = self.ac;
            }
            Opcode::STA_ABY => {
                let addr = self.fetch_two() + (self.y as u16);
                self.memory[ addr as usize ] = self.ac;
            }
            Opcode::STA_INX => {
            }
            Opcode::STA_INY => {
            }
            _ => {
                return false; 
            }
        }
        true
    }

    pub(crate) fn st_x(&mut self, opcode: u8) -> bool {
        let op = Opcode::from(opcode);
        match op {
            Opcode::STX_ZPG => {
                self.memory[ self.fetch_byte() as usize ] = self.x;
            }
            Opcode::STX_ZPY => {
                self.memory[ (self.fetch_byte() + self.y) as usize ] = self.x;
            }
            Opcode::STX_ABS => {
                self.memory[ self.fetch_two() as usize ] = self.x;
            }
            _ => {
                return false; 
            }
        }
        true
    }

    pub(crate) fn st_y(&mut self, opcode: u8) -> bool {
        let op = Opcode::from(opcode);
        match op {
            Opcode::STY_ZPG => {
                self.memory[ self.fetch_byte() as usize ] = self.y;
            }
            Opcode::STY_ZPX => {
                self.memory[ (self.fetch_byte() + self.x) as usize ] = self.y;
            }
            Opcode::STY_ABS => {
                self.memory[ self.fetch_two() as usize ] = self.y;
            }
            _ => {
                return false;
            }
        }
        true
    }
}
