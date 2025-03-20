// Handles stack operations
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Inst;
use bitflags::bitflags;

impl CPU {
    // Set N/Z status for TSX and PLA instructions
    pub fn nz_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }
        if register >> 7 == 1 {
            self.set_status(StatusRegister::N);
        }
    }

    pub fn stack(&mut self, inst: Inst) {

        match inst {
            Inst::TSX => {
                self.x = self.sp;
                self.nz_status(self.x);
            }
            Inst::TXS => {
                self.sp = self.x;
            }
            Inst::PHA => { // push accumulator
                self.memory[ 0x100 + self.sp as usize ] = self.ac;
                self.sp.wrapping_sub(1);
            }
            Inst::PHP => { // push register status
                self.set_status(StatusRegister::U);
                self.set_status(StatusRegister::B);
                self.memory[ 0x100 + self.sp as usize ] = self.sr.bits();
                self.sp.wrapping_sub(1);
            }
            Inst::PLA => { // Pull accumulator from stack 
                self.ac = self.memory[ 0x100 + self.sp as usize ];
                self.nz_status(self.ac);
                self.sp.wrapping_add(1); 
            }
            Inst::PLP => { // pull processor status 
                let status: u8 = self.memory[ 0x100 + self.sp as usize ];
                // First option
                // let flags = StatusRegister::from_bits(status).unwrap();
                // self.sr = flags;
                // not sure what this entails though
                // TODO: need to ignore break flag and bit 5

                // Second option
                let flags = StatusRegister::from_bits(status);
                match flags {
                    Some(flags) => {StatusRegister::from_bits(status);},
                    None => {println!("Bad bit found");}
                }
                self.sp += 1;
            }
            _ => panic!("Should not reach this instruction"),
        }

    }
}

