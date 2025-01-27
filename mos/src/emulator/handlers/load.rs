#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;
use crate::emulator::AddrMode;

impl CPU {

    // TODO: get rid of this to reduce redundancy
    // TODO add LDA / LDX / LDY / etc. 
    const LDA_OP:[Opcode; 8] = [
        Opcode::LDA_IMM,
        Opcode::LDA_ZPG,
        Opcode::LDA_ZPX,
        Opcode::LDA_ABS,
        Opcode::LDA_ABX,
        Opcode::LDA_ABY,
        Opcode::LDA_INX,
        Opcode::LDA_INY,
    ];

    const LDX_OP:[Opcode; 5] = [
        Opcode::LDX_IMM,
        Opcode::LDX_ZPG,
        Opcode::LDX_ZPY,
        Opcode::LDX_ABS,
        Opcode::LDX_ABY,
    ];

    const LDY_OP:[Opcode; 5] = [
        Opcode::LDY_IMM,
        Opcode::LDY_ZPG,
        Opcode::LDY_ZPX,
        Opcode::LDY_ABS,
        Opcode::LDY_ABX,
    ];

    pub fn is_ld_opcode(&mut self, op: &Opcode) -> bool {
        return Self::LDA_OP.contains(op) || Self::LDX_OP.contains(op) || Self::LDY_OP.contains(op);
    }

    fn ld_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 { // check if 7th bit is set
            self.set_status(StatusRegister::N);
        }
    }

    /*
    * Receive dispatch from the cpu.rs file 
    */
    pub fn ld_handle_dispatch(&mut self, op: &Opcode, val: u8) {
        match op {
            x if Self::LDA_OP.contains(op) => {
                self.ac = val;
                self.ld_set_status(self.ac);
            }
            x if Self::LDX_OP.contains(op) => {
                self.x = val;
                self.ld_set_status(self.x);
            }
            x if Self::LDY_OP.contains(x) => {
                self.y = val;
                self.ld_set_status(self.y);
            }
            _ => { return; }
        }
    }

    // return found opcode or not
    // pub fn ld_exec(&mut self, opcode: u8) -> bool {
    //     // halt after first one returns true
    //     return self.ld_acc(opcode) || self.ld_x(opcode) || self.ld_y(opcode);
    // }

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
