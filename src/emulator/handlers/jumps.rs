// handle jumps and calls
use crate::emulator::CPU;
use crate::emulator::Inst;


impl CPU {

    // jmp has a special property  according to this documentation
    // http://www.6502.org/users/obelisk/6502/reference.html#JMP
    // 6502 does not correctly fetch the target address if the indirect vector falls on a page
    //      boundary 
    //      TODO: should it behave correctly, or incorrectly (true to the nature of 6502)
    // jump to absolute is literally jumping to the absolute value, not the value at that addr 
    // TODO: need to review how absolute works 
    pub fn jmp(&mut self, inst: Inst, ja: usize) {
        let mut jump_addr: u16 = ja as u16;
        match inst {
            Inst::JMP => {
                self.pc = jump_addr
            },
            Inst::JSR => {
                // might need to break this down into individual ops actually 
                self.memory[ 0x100 + self.sp as usize] = (self.pc.wrapping_sub(1) >> 0x08) as u8;
                self.sp = self.sp.wrapping_sub(1);
                self.memory[ 0x100 + self.sp as usize] = ((self.pc.wrapping_sub(1) ) & 0xff) as u8; 
                // edge case when the lower byte is 0x00 
                // we will need to modify the pc here actually
                self.sp = self.sp.wrapping_sub(1);
                self.pc = jump_addr;
                println!("{} {}", self.pc, self.sp);
            },
            Inst::RTS => {
                jump_addr = self.memory[ 0x100 + self.sp as usize ] as u16; 
                self.sp.wrapping_add(1);
                jump_addr += (self.memory[ 0x100 + self.sp as usize ] as u16) << 8;
                self.sp.wrapping_add(1);
                jump_addr += 1;
            }

            _ => {return;}
        }
    }
}


