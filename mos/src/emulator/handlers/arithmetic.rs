use crate::emulator::CPU;
use crate::emulator::cpu::StatusRegister;
use crate::emulator::Opcode;

impl CPU {

    fn arith_fetch_operand(&mut self, op: Opcode) -> Option<u8> {

        Some(match op {
            Opcode::ADC_IMM | Opcode::SBC_IMM  => 
                self.memory[self.fetch_byte()]
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

            _ => return None
        })


    }

    pub(crate) fn arith_handle_ops(&mut self) {

    }
 
}
