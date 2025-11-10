use crate::emulator::CPU;
use crate::emulator::Inst;
use crate::emulator::cpu::StatusRegister;

impl CPU {
    pub fn sys(&mut self, inst: Inst) {
        let mut pc_to_push: u16 = self.pc + 2;
        match inst {

// The BRK instruction forces the generation of an interrupt request. The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
            // what is a break mark?

// how does this work?
// BRK instruction are fully executed before any interrupt. (On the NMOS version, if an interrupt occured while a BRK instruction was fetching the interupt vector, this would be overwritten and the interrupt executed, instead.) <-- what does "this" refer to?
            // push high byte first, then low byte since we READ low byte first
            Inst::BRK => {
                // set break flag to 1 before pushing
                self.set_status(StatusRegister::B);
                
                // generate an interrupt request 
                self.memory[ 0x100 + self.sp as usize] = (self.pc >> 8) as u8; // 2 bytes
                self.sp = self.sp.wrapping_sub(1);

                self.memory[ 0x100 + self.sp as usize] = (self.pc & 0xff) as u8;
                self.sp = self.sp.wrapping_sub(1);

                self.memory[ 0x100 + self.sp as usize] = self.sr.bits() as u8;
                self.sp = self.sp.wrapping_sub(1);

            },
            Inst::NOP => {
            },
            _ => return
        }
    }
}
