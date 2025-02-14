use std::collections::HashMap;
use crate::emulator::CPU;
use crate::emulator::Inst;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::cpu::MEMSIZE;

impl CPU {

    fn check_boundary(&mut self, old_pc: u16, new_pc: u16) -> bool {
        return (old_pc >> 8) == (new_pc >> 8);
    }

    pub fn branch(&mut self, inst: Inst, orig_pc: u16, offset: i8) {
        // offset could be negative
        let mut i_pc: i32 = orig_pc as i32;
        let mut i_offset: i32 = offset as i32;
        let mut new_pc: u16 = ((i_pc + i_offset) % (MEMSIZE as i32)) as u16;
        let mut status_flag: bool = false;
        let mut table = HashMap::new();

        // format is key = instruction, value = boolean of how to set the new pc 
        table.insert(Inst::BCC, !self.get_status(StatusRegister::C));
        table.insert(Inst::BCS, self.get_status(StatusRegister::C));
        table.insert(Inst::BEQ, self.get_status(StatusRegister::Z));
        table.insert(Inst::BMI, self.get_status(StatusRegister::N));
        table.insert(Inst::BNE, !self.get_status(StatusRegister::Z));
        table.insert(Inst::BPL, !self.get_status(StatusRegister::N));
        table.insert(Inst::BVC, !self.get_status(StatusRegister::V));
        table.insert(Inst::BVS, self.get_status(StatusRegister::V));

        if table[&inst] {
            self.pc = new_pc; // the edge case is if we need to wrap back around
            // in the subtraction
            self.cycle_count += 1;
            status_flag = true;
        } 
        if status_flag && self.check_boundary(orig_pc, new_pc) {
            self.cycle_count += 1;
        }
    }
}
