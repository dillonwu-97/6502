#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;
use crate::emulator::AddrMode;
use crate::emulator::Inst;

impl CPU {

    // TODO: get rid of this to reduce redundancy
    // TODO add LDA / LDX / LDY / etc. 
    
    pub fn is_ld_opcode(&mut self, op: Opcode) {

    }

    fn ld_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 { // check if 7th bit is set
            self.set_status(StatusRegister::N);
        }
    }

    pub fn ld(&mut self, inst: Inst, val: u8) {
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
                self.x = val;
                self.ld_set_status(self.y);
            }
            _ => {return;}
        }
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
