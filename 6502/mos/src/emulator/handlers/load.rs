#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;

impl CPU {
    pub(crate) fn ld_set_status(&mut self, register: u8) {
        // set status is different based on the register i think 
        // so we should actually have to pass the register here?
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if self.get_status(StatusRegister::V) {
            self.set_status(StatusRegister::N);
        }
    }

    pub(crate) fn ld_execute(&mut self, opcode: u8) {
        self.ld_acc(opcode); 
    }

    pub(crate) fn ld_acc(&mut self, opcode: u8) {
        //println!("ld execute {}", LDA_IMM);
        let op = Opcode::from(opcode);
        match op {
            // load accumulator opcodes
            Opcode::LDA_IMM => {
                self.ac = self.fetch_byte();
            }
            Opcode::LDA_ZPG => {
                self.ac = self.memory[ self.fetch_byte() as usize ];
            }
            Opcode::LDA_ZPX => {
                self.ac = self.memory[ (self.fetch_byte() + self.x) as usize ]; 
            }
            Opcode::LDA_ABS => {
                self.ac = self.memory[ self.fetch_two() as usize ];
            }
            Opcode::LDA_ABX => {
                let value = self.fetch_two() + (self.x as u16);
                self.ac = self.memory[value as usize];
            }
            Opcode::LDA_ABY => {
                let value = self.fetch_two() + (self.y as u16);
                self.ac = self.memory[value as usize];
            }
            Opcode::LDA_INX => {
                // not yet sure how these work 
                // too tired to figure it out atm 
            }
            Opcode::LDA_INY => {

            }
            _ => {}
        }
        self.ld_set_status(self.ac);
    }

    pub(crate) fn ld_x(&mut self, opcode: u8) {
        let op = Opcode::from(opcode);
        match op {
            // X register opcodes
            Opcode::LDX_IMM => {
                self.x = self.fetch_byte();
            }
            Opcode::LDX_ZPG => {
                self.x = self.memory[self.fetch_byte() as usize]
            }
            Opcode::LDX_ZPY => {
                self.x = self.memory[ (self.fetch_byte() + self.y) as usize]
            }
            _ => {}
        }
        self.ld_set_status(self.ac);
    }

}
