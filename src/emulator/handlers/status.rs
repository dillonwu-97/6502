use crate::emulator::CPU;
use crate::emulator::Inst;
use crate::emulator::cpu::StatusRegister;

impl CPU {

    pub fn stats(&mut self, inst: Inst) {
        match inst {
            Inst::CLC => self.clear_status(StatusRegister::C),
            Inst::CLD => self.clear_status(StatusRegister::D),
            Inst::CLI => self.clear_status(StatusRegister::I),
            Inst::CLV => self.clear_status(StatusRegister::V),
            Inst::SEC => self.set_status(StatusRegister::C),
            Inst::SED => self.set_status(StatusRegister::D),
            Inst::SEI => self.set_status(StatusRegister::I),
            _ => return
        } 
    }


}

