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

    pub fn idc(&mut self, mem_val: u8) {
        self.x += 1;
        // return self.x; 
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

