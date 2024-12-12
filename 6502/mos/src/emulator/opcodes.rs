#![allow(non_snake_case)]
pub enum Opcode {
    // load accumulator
    LDA_IMM = 0xA9,  // immediate load accumulator
    LDA_ZPG = 0xA5, // load zero page
    LDA_ZPX = 0xB5, // load value at zero page offset + x into accumulator
    LDA_ABS = 0xAD,
    LDA_ABX = 0xBD,
    LDA_ABY = 0xB9,
    LDA_INX = 0xA1,
    LDA_INY = 0xB1,

    // load x
    LDX_IMM = 0xA2,
    LDX_ZPG = 0xA6,
    LDX_ZPY = 0xB6,
    LDX_ABS = 0xAE,
    LDX_ABY = 0xBE,

    // load y
    LDY_IMM = 0xA0,
    LDY_ZPG = 0xA4,
    LDY_ZPX = 0xB4,
    LDY_ABS = 0xAC,
    LDY_ABX = 0xBC,
}


impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0xA9 => Opcode::LDA_IMM,
            0xA5 => Opcode::LDA_ZPG,
            0xB5 => Opcode::LDA_ZPX,
            0xAD => Opcode::LDA_ABS,
            0xBD => Opcode::LDA_ABX,
            0xB9 => Opcode::LDA_ABY,
            0xA1 => Opcode::LDA_INX,
            0xB1 => Opcode::LDA_INY,

            0xA2 => Opcode::LDX_IMM,
            0xA6 => Opcode::LDX_ZPG,
            0xB6 => Opcode::LDX_ZPY,
            0xAE => Opcode::LDX_ABS,
            0xBE => Opcode::LDX_ABY,

            0xA0 => Opcode::LDY_IMM,
            0xA4 => Opcode::LDY_ZPG,
            0xB4 => Opcode::LDY_ZPX,
            0xAC => Opcode::LDY_ABS,
            0xBC => Opcode::LDY_ABX,

            _ => panic!("Value not found")
        }
    } 
}

