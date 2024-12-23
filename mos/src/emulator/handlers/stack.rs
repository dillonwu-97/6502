// Handles stack operations
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;

impl CPU {

    pub(crate) fn sp_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 {
            self.set_status(StatusRegister::N);
        }
    }

    pub(crate) fn stack_exec(&mut self, opcode: u8) -> bool {
        let op = Opcode::from(opcode);         
        match op {
            Opcode::PHA => {
                self.memory[self.sp as usize] = self.ac;
                self.sp-=1;
            }
            Opcode::PHP => {
                self.memory[self.sp as usize] = self.sr.bits();
                self.sp-=1;
            }
            Opcode::PLA => {
                self.ac = self.memory[self.sp as usize];
                self.sp+=1;
                self.sp_set_status(self.ac);
            }
            Opcode::PLP => {
                self.sr = StatusRegister::from_bits_truncate(self.memory[self.sp as usize]);
                self.sp += 1;
            }
            _ => {
                return false;
            }
        }
        true
    }
}

