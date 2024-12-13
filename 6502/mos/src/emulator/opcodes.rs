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

    // store accumulator
    STA_ZPG = 0x85,
    STA_ZPX = 0x95,
    STA_ABS = 0x8D,
    STA_ABX = 0x9D,
    STA_ABY = 0x99,
    STA_INX = 0x81,
    STA_INY = 0x91,

    // store x 
    STX_ZPG = 0x86,
    STX_ZPY = 0x96,
    STX_ABS = 0x8E,

    // store y 
    STY_ZPG = 0x84,
    STY_ZPX = 0x94,
    STY_ABS = 0x8C,

    // Transfer opcodes
    TAX = 0xAA,
    TAY = 0xA8,
    TSX = 0xBA,
    TXA = 0x8A,
    TXS = 0x9A,
    TYA = 0x98,
}


impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            // load accumulator 
            0xA9 => Opcode::LDA_IMM,
            0xA5 => Opcode::LDA_ZPG,
            0xB5 => Opcode::LDA_ZPX,
            0xAD => Opcode::LDA_ABS,
            0xBD => Opcode::LDA_ABX,
            0xB9 => Opcode::LDA_ABY,
            0xA1 => Opcode::LDA_INX,
            0xB1 => Opcode::LDA_INY,

            // load x
            0xA2 => Opcode::LDX_IMM,
            0xA6 => Opcode::LDX_ZPG,
            0xB6 => Opcode::LDX_ZPY,
            0xAE => Opcode::LDX_ABS,
            0xBE => Opcode::LDX_ABY,

            // load y
            0xA0 => Opcode::LDY_IMM,
            0xA4 => Opcode::LDY_ZPG,
            0xB4 => Opcode::LDY_ZPX,
            0xAC => Opcode::LDY_ABS,
            0xBC => Opcode::LDY_ABX,

            // store accumulator
            0x85 => Opcode::STA_ZPX,  
            0x95 => Opcode::STA_ABS,  
            0x8D => Opcode::STA_ABX,  
            0x9D => Opcode::STA_ABY,  
            0x81 => Opcode::STA_INX,  
            0x91 => Opcode::STA_INY,  
                 
            // store x
            0x86 => Opcode::STX_ZPG,  
            0x96 => Opcode::STX_ZPY,  
            0x8E => Opcode::STX_ABS,  
                 
            // store y
            0x84 => Opcode::STY_ZPG,  
            0x94 => Opcode::STY_ZPX,  
            0x8C => Opcode::STY_ABS,  

            // transfer 
            0xAA => Opcode::TAX,
            0xA8 => Opcode::TAY,
            0xBA => Opcode::TSX,
            0x8A => Opcode::TXA,
            0x9A => Opcode::TXS,
            0x98 => Opcode::TYA,

            _ => panic!("Opcode not found")
        }
    } 
}

