use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;
use crate::emulator::Inst;


// TODO: should write some test cases for this to make sure that the renaming conventions are ok in
// the opcodes file 
impl CPU {
    pub fn idc_set_status(&mut self, val: u8) {
        if val == 0 {
            self.set_status(StatusRegister::Z);
        }
        if val >> 7 == 1 {
            self.set_status(StatusRegister::N);
        }
    }

    pub fn idc(&mut self, inst: Inst, idx: usize) {
        match inst {
            Inst::INC => {
                self.memory[idx] += 1;
                self.idc_set_status(self.memory[idx]);
            },
            Inst::INX => {
                self.x += 1; 
                self.idc_set_status(self.x);
            },
            Inst::INY => {
                self.y += 1;
                self.idc_set_status(self.y);
            },
            Inst::DEC => {
                self.memory[idx] += 1;
                self.idc_set_status(self.memory[idx]);
            },
            Inst::DEX => {
                self.x -= 1;
                self.idc_set_status(self.x);
            },
            Inst::DEY => {
                self.y -= 1;
                self.idc_set_status(self.y);
            },
            _ => {return; }

        }
    }

    // pub fn idc(&mut self, inst: Inst, mem_val: u8) -> u8 {
    //     match inst {
    //         Inst::INC => {
    //             let temp: mut u8 = mem_val;
    //             temp += 1;
    //             return temp;
    //         }
    //         _=>{return 0;}
    //     } 
    //
    // }
}

