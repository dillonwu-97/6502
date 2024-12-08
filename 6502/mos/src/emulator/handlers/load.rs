use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;

impl CPU {
    pub(crate) fn lda_set_status(&mut self) {
        if self.ac == 0 {
            self.set_status(StatusRegister::Z);
        }
        if self.get_status(StatusRegister::V) {
            self.set_status(StatusRegister::N);
        }
    }
}
