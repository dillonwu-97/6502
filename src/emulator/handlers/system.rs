use crate::emulator::CPU;
use crate::emulator::Inst;
use crate::emulator::cpu::StatusRegister;

impl CPU {
    pub fn sys(&mut self, inst: Inst) {
        self.pc = self.pc.wrapping_add(1);
        println!("rust val: {}", self.pc);
        match inst {

// The BRK instruction forces the generation of an interrupt request. The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
            // load value from 0xfffe and 0xffff to pc as well instead of subtraction stuff

// how does this work?
// BRK instruction are fully executed before any interrupt. (On the NMOS version, if an interrupt occured while a BRK instruction was fetching the interupt vector, this would be overwritten and the interrupt executed, instead.) <-- what does "this" refer to?
            // push high byte first, then low byte since we READ low byte first
            Inst::BRK => {
                // set break flag to 1 before pushing
                
                // generate an interrupt request 
                self.memory[ 0x100 + self.sp as usize] = (self.pc >> 8) as u8; // 2 bytes
                self.sp = self.sp.wrapping_sub(1);

                self.memory[ 0x100 + self.sp as usize] = (self.pc & 0xff) as u8;
                self.sp = self.sp.wrapping_sub(1);

                let pushed : StatusRegister = self.sr.clone() | StatusRegister::B;
                self.memory[ 0x100 + self.sp as usize] = pushed.bits() as u8;
                self.set_status(StatusRegister::I);
                self.sp = self.sp.wrapping_sub(1);
                self.pc = ((self.memory[0xffff] as u16) << 8) | (self.memory[0xfffe] as u16);

            },
            Inst::NOP => {
            },
            _ => return
        }
    }
}
