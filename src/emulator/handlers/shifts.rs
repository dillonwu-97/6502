use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Inst;

const LEFT: bool = false;
const RIGHT: bool = true;

impl CPU {
    // TODO: also, whose responsibility is it to clear the status registers?
    // do we assume that the registers will always be 0, or can there be stale data? 
    // assuming for now that it is NOT our responsibility to clear the status bits for a register
    // because there are specific instructions responsible for handling that so presumably, if a
    // register is set from an add instruction, and there is another add that does a carry, before
    // the second add takes place, an instruction clears out the carry flag 
    fn sh_set_status(&mut self, cur_val: u8, prev_val: u8, dir: bool) {
        
        if (dir == LEFT) {
            if (prev_val >> 7 == 1) { // if we shifted left 
                self.set_status(StatusRegister::C);
            } else {
                self.clear_status(StatusRegister::C);
            }
        } else { // if we shifted right
            if (prev_val & 1 == 1) {
                self.set_status(StatusRegister::C);
            } else {
                self.clear_status(StatusRegister::C);
            }
        }
        
        if (cur_val == 0) {
            self.set_status(StatusRegister::Z);
        } else {
            self.clear_status(StatusRegister::Z);
        }

        if (cur_val >> 7 == 1) {
            self.set_status(StatusRegister::N);
        } else {
            self.clear_status(StatusRegister::N);
        }

    }

    // pass in the idx because we have to emulate the rotation in memory
    // should be different depending on the modes i think 
    // yea assignment is in the cpu.rs file 
    // passing in accumulator, or memory so we cannot index into the accumulator 
    // The documentation at 6502.org seems to imply that the responsibility is on the instruction
    // to set / clear the carry flag 
    pub fn sh(&mut self, inst: Inst, idx: usize, is_acc: bool) {

        let prev_val: u8;
        let cur_val: u8;
        let dir: bool;

        if is_acc {
            prev_val = self.ac;
        } else {
            prev_val = self.memory[idx];
        }

        match inst {
            Inst::ASL => {
                if is_acc {
                    self.ac <<= 1;
                } else {
                    self.memory[idx] <<= 1;
                }
                dir = LEFT;
            },
            Inst::LSR => {
                if is_acc {
                    self.ac >>= 1;
                } else {
                    self.memory[idx] >>= 1;
                }
                dir = RIGHT;
            },
            Inst::ROL => {
                
                if is_acc {
                    self.ac <<= 1;
                    self.ac |= self.get_status(StatusRegister::C) as u8;
                } else {
                    self.memory[idx] <<= 1;
                    self.memory[idx] |= self.get_status(StatusRegister::C) as u8;
                }
                dir = LEFT;
            },
            Inst::ROR => {
                if is_acc {
                    self.ac >>= 1;
                    self.ac |= ((self.get_status(StatusRegister::C) as u8) << 7);
                } else {
                    self.memory[idx] >>= 1;
                    self.memory[idx] |= ((self.get_status(StatusRegister::C) as u8) << 7);
                }
                dir = RIGHT;
            },
            _ => {return;}
        }
        if (is_acc) {
            cur_val = self.ac;
        } else {
            cur_val = self.memory[idx];
        }
        self.sh_set_status(cur_val, prev_val, dir);
    }
}

