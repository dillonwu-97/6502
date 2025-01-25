#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;

impl CPU {
    pub(crate) fn ld_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 { // check if 7th bit is set
            self.set_status(StatusRegister::N);
        }
    }

    // return found opcode or not
    pub(crate) fn ld_exec(&mut self, opcode: u8) -> bool {
        // halt after first one returns true
        return self.ld_acc(opcode) || self.ld_x(opcode) || self.ld_y(opcode);
    }


    pub(crate) fn ld_acc(&mut self, opcode: u8) -> bool {
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
            _ => {
                return false;
            }
        }
        self.ld_set_status(self.ac);
        true
    }

    pub(crate) fn ld_x(&mut self, opcode: u8) -> bool {
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
            Opcode::LDX_ABS => {
                self.x = self.memory[ self.fetch_two() as usize ];
            }
            Opcode::LDX_ABY => {
                let value = self.fetch_two() + (self.y as u16);
                self.x = self.memory[value as usize];
            }
            _ => {
                return false;
            }
        }
        self.ld_set_status(self.x);
        true
    }

    pub(crate) fn ld_y(&mut self, opcode: u8) -> bool {
        let op = Opcode::from(opcode);
        match op {
            Opcode::LDY_IMM => {
                self.y = self.fetch_byte();
            }
            Opcode::LDY_ZPG => {
                self.y = self.memory[self.fetch_byte() as usize]
            }
            Opcode::LDY_ZPX => {
                self.y = self.memory[ (self.fetch_byte() + self.x) as usize]
            }
            Opcode::LDY_ABS => {
                self.y = self.memory[ self.fetch_two() as usize ];
            }
            Opcode::LDY_ABX => {
                let value = self.fetch_two() + (self.x as u16);
                self.y = self.memory[value as usize];
            }
            _ => {
                return false; 
            }
        }
        self.ld_set_status(self.y);
        true
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_lda() {
        // why do I need use here when it's already declared at the top? v confused
        use super::CPU;
        use crate::emulator::Opcode;
        let mut cpu = CPU::new();
        cpu.reset();
        let op: u8 = Opcode::LDA_IMM as u8;
        cpu.memory[0x0] = op;
        cpu.memory[0x1] = 0x41;
        cpu.execute();
        assert_eq!(cpu.ac, 0x41);
    }
}
