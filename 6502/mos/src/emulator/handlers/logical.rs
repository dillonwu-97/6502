#![allow(non_snake_case)]
use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;


impl CPU {

    pub(crate) fn log_set_status(&mut self, register: u8) {
        if register == 0 {
            self.set_status(StatusRegister::Z);
        }     
        if register >> 7 == 1 {
            self.set_status(StatusRegister::N);
        }
    }

    // Switching between operands
    pub(crate) fn fetch_operand(&mut self, op: Opcode) -> Option<u8> {
        Some(match op {
            Opcode::AND_IMM | Opcode::EOR_IMM | Opcode::ORA_IMM => 
                self.fetch_byte(),
            Opcode::AND_ZPG | Opcode::EOR_ZPG | Opcode::ORA_ZPG => 
                self.memory[ self.fetch_byte() as usize ], 
            Opcode::AND_ZPX | Opcode::EOR_ZPX | Opcode::ORA_ZPX =>
                self.memory[ self.fetch_byte().wrapping_add(self.x) as usize ],
            Opcode::AND_ABS | Opcode::EOR_ABS | Opcode::ORA_ABS =>
                self.memory[ self.fetch_two() as usize ],
            Opcode::AND_ABX | Opcode::EOR_ABX | Opcode::ORA_ABX =>
                self.memory[ self.fetch_two().wrapping_add(self.x as u16) as usize ],
            Opcode::AND_ABY | Opcode::EOR_ABY | Opcode::ORA_ABY =>
                self.memory[ self.fetch_two().wrapping_add(self.y as u16) as usize ],
            Opcode::AND_INX | Opcode::EOR_INX | Opcode::ORA_INX => {
                unimplemented!()
            }
            Opcode::AND_INY | Opcode::EOR_INY | Opcode::ORA_INY => {
                unimplemented!()
            }
            _ => return None,
        })
    }

    pub(crate) fn handle_op<F>(&mut self, opcode: u8, operation: F) -> bool 
    where 
        F: Fn(u8, u8) -> u8
    {
        let op = Opcode::from(opcode);    
        if let Some(operand) = self.fetch_operand(op) {
            self.ac = operation(self.ac, operand); 
            self.log_set_status(self.ac);
            true
        } else {
            false
        }
    }

    pub(crate) fn and_exec(&mut self, opcode: u8) -> bool {
        self.handle_op(opcode, | a,b | a & b )
    }

    pub(crate) fn eor_exec(&mut self, opcode: u8) -> bool {
        self.handle_op(opcode, | a,b | a ^ b )
    }

    pub(crate) fn ora_exec(&mut self, opcode: u8) -> bool {
        self.handle_op(opcode, | a,b | a | b )
    }

    pub(crate) fn bit_test(&mut self, opcode: u8) -> bool {
        let op = Opcode::from(opcode);
        match op {
            Opcode::BIT_ZPG => {

            }
            Opcode::BIT_ABS => {

            }
            _ => {
                return false;
            }
        }
        true
    }

    pub(crate) fn log_exec(&mut self, opcode: u8) -> bool {
        self.and_exec(opcode) | self.eor_exec(opcode) | self.ora_exec(opcode)
    }
}
