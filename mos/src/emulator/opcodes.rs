// TODO: maybe there is a better way of organizing / using opcodes
// TODO this file is A LOT of redundancy; there might be a better way of dealing with this 
// Might be better instead of enumerating all the opcodes to use an array and store the opcodes in
// the index so we can index into the array to get the opcode 
// then afterwards maybe do something like 

#![allow(non_snake_case)]


#[derive(PartialEq, Eq)]
pub enum AddrMode {
    IMM = 0x00,
    ZPG = 0x01,
    ZPX = 0x02,
    ZPY = 0x03,
    ABS = 0x04,
    ABX = 0x05,
    ABY = 0x06,
    INX = 0x07,
    INY = 0x08,
}


#[derive(PartialEq, Eq)]
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

    // Stack operations 
    PHA = 0x48,
    PHP = 0x08,
    PLA = 0x68,
    PLP = 0x28,

    // Logical operations
    AND_IMM = 0x29,
    AND_ZPG = 0x25,
    AND_ZPX = 0x35,
    AND_ABS = 0x2D,
    AND_ABX = 0x3D,
    AND_ABY = 0x39,
    AND_INX = 0x21,
    AND_INY = 0x31,

    EOR_IMM = 0x49,
    EOR_ZPG = 0x45,
    EOR_ZPX = 0x55,
    EOR_ABS = 0x4D,
    EOR_ABX = 0x5D,
    EOR_ABY = 0x59,
    EOR_INX = 0x41,
    EOR_INY = 0x51,

    ORA_IMM = 0x09,
    ORA_ZPG = 0x05,
    ORA_ZPX = 0x15,
    ORA_ABS = 0x0D,
    ORA_ABX = 0x1D,
    ORA_ABY = 0x19,
    ORA_INX = 0x01,
    ORA_INY = 0x11,

    BIT_ZPG = 0x24,
    BIT_ABS = 0x2C,

    // Arithmetic operations
    // Add with carry
    ADC_IMM = 0x69,
    ADC_ZPG = 0x65,
    ADC_ZPX = 0x75,
    ADC_ABS = 0x6D,
    ADC_ABX = 0x7D,
    ADC_ABY = 0x79,
    ADC_INX = 0x61,
    ADC_INY = 0x71,

    // subtract with carry
    SBC_IMM = 0xE9,
    SBC_ZPG = 0xE5,
    SBC_ZPX = 0xF5,
    SBC_ABS = 0xED,
    SBC_ABX = 0xFD,
    SBC_ABY = 0xF9,
    SBC_INX = 0xE1,
    SBC_INY = 0xF1,

    // cmp 
    CMP_IMM = 0xC9,
    CMP_ZPG = 0xC5,
    CMP_ZPX = 0xD5,
    CMP_ABS = 0xCD,
    CMP_ABX = 0xDD,
    CMP_ABY = 0xD9,
    CMP_INX = 0xC1,
    CMP_INY = 0xD1,

    // compare x 
    CPX_IMM = 0xE0,
    CPX_ZPG = 0xE4,
    CPX_ABS = 0xEC,

    // compare y
    CPY_IMM = 0xC0,
    CPY_ZPG = 0xC4,
    CPY_ABS = 0xCC,

    // Increments and Decrements
    INC_ZPG = 0xE6,
    INC_ZPX = 0xF6,
    INC_ABS = 0xEE,
    INC_ABX = 0xFE,

    INX = 0xE8,
    INY = 0xC8,

    DEC_ZPG = 0xC6,
    DEC_ZPX = 0xD6,
    DEC_ABS = 0xCE,
    DEC_ABX = 0xDE,

    DEX = 0xCA,
    DEY = 0x88,

    // Shifts
    ASL_ACC = 0x0A,
    ASL_ZPG = 0x06,
    ASL_ZPX = 0x16,
    ASL_ABS = 0x0E,
    ASL_ABX = 0x1E,

    LSR_ACC = 0x4A,
    LSR_ZPG = 0x46,
    LSR_ZPX = 0x56,
    LSR_ABS = 0x4E,
    LSR_ABX = 0x5E,

    ROL_ACC = 0x2A,
    ROL_ZPG = 0x26,
    ROL_ZPX = 0x36,
    ROL_ABS = 0x2E,
    ROL_ABX = 0x3E,

    ROR_ACC = 0x6A,
    ROR_ZPG = 0x66,
    ROR_ZPX = 0x76,
    ROR_ABS = 0x6E,
    ROR_ABX = 0x7E,

    // Jumps and calls
    JMP_ABS = 0x4C,
    JMP_IND = 0x6C,
    JSR_ABS = 0x20,
    RTS = 0x60,

    // Branches
    BCC = 0x90,
    BCS = 0xB0,
    BEQ = 0xF0,
    BMI = 0x30,
    BNE = 0xD0,
    BPL = 0x10,
    BVC = 0x50,
    BVS = 0x70,

    // Status Flag Changes 
    CLC = 0x18,
    CLD = 0xD8,
    CLI = 0x58,
    CLV = 0xB8,
    SEC = 0x38,
    SED = 0xF8,
    SEI = 0x78,
    
    // System functions
    BRK = 0x00,
    NOP = 0xEA,
    RTI = 0x40,

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

            0x48 => Opcode::PHA,
            0x08 => Opcode::PHP,
            0x68 => Opcode::PLA,
            0x28 => Opcode::PLP,

            // Logical operations
            0x29 => Opcode::AND_IMM,
            0x25 => Opcode::AND_ZPG,
            0x35 => Opcode::AND_ZPX,
            0x2D => Opcode::AND_ABS,
            0x3D => Opcode::AND_ABX,
            0x39 => Opcode::AND_ABY,
            0x21 => Opcode::AND_INX,
            0x31 => Opcode::AND_INY,

            0x49 => Opcode::EOR_IMM,
            0x45 => Opcode::EOR_ZPG,
            0x55 => Opcode::EOR_ZPX,
            0x4D => Opcode::EOR_ABS,
            0x5D => Opcode::EOR_ABX,
            0x59 => Opcode::EOR_ABY,
            0x41 => Opcode::EOR_INX,
            0x51 => Opcode::EOR_INY,

            0x09 => Opcode::ORA_IMM,
            0x05 => Opcode::ORA_ZPG,
            0x15 => Opcode::ORA_ZPX,
            0x0D => Opcode::ORA_ABS,
            0x1D => Opcode::ORA_ABX,
            0x19 => Opcode::ORA_ABY,
            0x01 => Opcode::ORA_INX,
            0x11 => Opcode::ORA_INY,

            0x24 => Opcode::BIT_ZPG,
            0x2C => Opcode::BIT_ABS,

            0x69 => Opcode::ADC_IMM,
            0x65 => Opcode::ADC_ZPG,
            0x75 => Opcode::ADC_ZPX,
            0x6D => Opcode::ADC_ABS,
            0x7D => Opcode::ADC_ABX,
            0x79 => Opcode::ADC_ABY,
            0x61 => Opcode::ADC_INX,
            0x71 => Opcode::ADC_INY,

            // subtract with carry
            0xE9 => Opcode::SBC_IMM,
            0xE5 => Opcode::SBC_ZPG,
            0xF5 => Opcode::SBC_ZPX,
            0xED => Opcode::SBC_ABS,
            0xFD => Opcode::SBC_ABX,
            0xF9 => Opcode::SBC_ABY,
            0xE1 => Opcode::SBC_INX,
            0xF1 => Opcode::SBC_INY,

            // cmp 
            0xC9 => Opcode::CMP_IMM,
            0xC5 => Opcode::CMP_ZPG,
            0xD5 => Opcode::CMP_ZPX,
            0xCD => Opcode::CMP_ABS,
            0xDD => Opcode::CMP_ABX,
            0xD9 => Opcode::CMP_ABY,
            0xC1 => Opcode::CMP_INX,
            0xD1 => Opcode::CMP_INY,

            // compare x 
            0xE0 => Opcode::CPX_IMM,
            0xE4 => Opcode::CPX_ZPG,
            0xEC => Opcode::CPX_ABS,

            // compare y
            0xC0 => Opcode::CPY_IMM,
            0xC4 => Opcode::CPY_ZPG,
            0xCC => Opcode::CPY_ABS,

            // Increments and decrements 
            0xE6 => Opcode::INC_ZPG,
            0xF6 => Opcode::INC_ZPX,
            0xEE => Opcode::INC_ABS,
            0xFE => Opcode::INC_ABX,

            0xE8 => Opcode::INX,
            0xC8 => Opcode::INY,

            0xC6 => Opcode::DEC_ZPG,
            0xD6 => Opcode::DEC_ZPX,
            0xCE => Opcode::DEC_ABS,
            0xDE => Opcode::DEC_ABX,

            0xCA => Opcode::DEX,
            0x88 => Opcode::DEY,

            0x0A => Opcode::ASL_ACC,
            0x06 => Opcode::ASL_ZPG,
            0x16 => Opcode::ASL_ZPX,
            0x0E => Opcode::ASL_ABS,
            0x1E => Opcode::ASL_ABX,
              
            0x4A => Opcode::LSR_ACC,
            0x46 => Opcode::LSR_ZPG,
            0x56 => Opcode::LSR_ZPX,
            0x4E => Opcode::LSR_ABS,
            0x5E => Opcode::LSR_ABX,
              
            0x2A => Opcode::ROL_ACC,
            0x26 => Opcode::ROL_ZPG,
            0x36 => Opcode::ROL_ZPX,
            0x2E => Opcode::ROL_ABS,
            0x3E => Opcode::ROL_ABX,
              
            0x6A => Opcode::ROR_ACC,
            0x66 => Opcode::ROR_ZPG,
            0x76 => Opcode::ROR_ZPX,
            0x6E => Opcode::ROR_ABS,
            0x7E => Opcode::ROR_ABX,

            // Jumps and calls
            0x4C => Opcode::JMP_ABS,
            0x6C => Opcode::JMP_IND,
            0x20 => Opcode::JSR_ABS,
            0x60 => Opcode::RTS,

            // Branches
            0x90 => Opcode::BCC,
            0xB0 => Opcode::BCS,
            0xF0 => Opcode::BEQ,
            0x30 => Opcode::BMI,
            0xD0 => Opcode::BNE,
            0x10 => Opcode::BPL,
            0x50 => Opcode::BVC,
            0x70 => Opcode::BVS,

            0x18 => Opcode::CLC,
            0xD8 => Opcode::CLD,
            0x58 => Opcode::CLI,
            0xB8 => Opcode::CLV,
            0x38 => Opcode::SEC,
            0xF8 => Opcode::SED,
            0x78 => Opcode::SEI,

            // System functions
            0x00 => Opcode::BRK,
            0xEA => Opcode::NOP,
            0x40 => Opcode::RTI,

            _ => panic!("Opcode not found")
        }
    } 
}

impl Opcode {
    // TODO: is there a better way to do this than using vectors?
    const IMM_OP:[Opcode; 11] = [
        Opcode::LDA_IMM,
        Opcode::LDX_IMM,
        Opcode::LDY_IMM,
        Opcode::AND_IMM,
        Opcode::EOR_IMM,
        Opcode::ORA_IMM,
        Opcode::ADC_IMM,
        Opcode::SBC_IMM,
        Opcode::CMP_IMM,
        Opcode::CPX_IMM,
        Opcode::CPY_IMM
    ];
    const ZPG_OP:[Opcode; 20] = [
        Opcode::LDA_ZPG,
        Opcode::LDX_ZPG,
        Opcode::LDY_ZPG,
        Opcode::STX_ZPG,
        Opcode::STY_ZPG,
        Opcode::AND_ZPG,
        Opcode::EOR_ZPG,
        Opcode::ORA_ZPG,
        Opcode::BIT_ZPG,
        Opcode::ADC_ZPG,
        Opcode::SBC_ZPG,
        Opcode::CMP_ZPG,
        Opcode::CPX_ZPG,
        Opcode::CPY_ZPG,
        Opcode::INC_ZPG,
        Opcode::DEC_ZPG,
        Opcode::ASL_ZPG,
        Opcode::LSR_ZPG,
        Opcode::ROL_ZPG,
        Opcode::ROR_ZPG,
    ];
    const ZPX_OP:[Opcode; 16] = [
        Opcode::LDA_ZPX,
        Opcode::LDY_ZPX,
        Opcode::STA_ZPX,
        Opcode::STY_ZPX,
        Opcode::AND_ZPX,
        Opcode::EOR_ZPX,
        Opcode::ORA_ZPX,
        Opcode::ADC_ZPX,
        Opcode::SBC_ZPX,
        Opcode::CMP_ZPX,
        Opcode::INC_ZPX,
        Opcode::DEC_ZPX,
        Opcode::ASL_ZPX,
        Opcode::LSR_ZPX,
        Opcode::ROL_ZPX,
        Opcode::ROR_ZPX,
    ];
    const ZPY_OP:[Opcode; 2] = [Opcode::LDX_ZPY, Opcode::STX_ZPY];
    const ABS_OP:[Opcode; 23] = [
        Opcode::LDA_ABS,
        Opcode::LDX_ABS,
        Opcode::LDY_ABS,
        Opcode::STA_ABS,
        Opcode::STX_ABS,
        Opcode::STY_ABS,
        Opcode::AND_ABS,
        Opcode::EOR_ABS,
        Opcode::ORA_ABS,
        Opcode::BIT_ABS,
        Opcode::ADC_ABS,
        Opcode::SBC_ABS,
        Opcode::CMP_ABS,
        Opcode::CPX_ABS,
        Opcode::CPY_ABS,
        Opcode::INC_ABS,
        Opcode::DEC_ABS,
        Opcode::ASL_ABS,
        Opcode::LSR_ABS,
        Opcode::ROL_ABS,
        Opcode::ROR_ABS,
        Opcode::JMP_ABS,
        Opcode::JSR_ABS,
    ];
    const ABX_OP:[Opcode; 15] = [
        Opcode::LDA_ABX,
        Opcode::LDY_ABX,
        Opcode::STA_ABX,
        Opcode::AND_ABX,
        Opcode::EOR_ABX,
        Opcode::ORA_ABX,
        Opcode::ADC_ABX,
        Opcode::SBC_ABX,
        Opcode::CMP_ABX,
        Opcode::INC_ABX,
        Opcode::DEC_ABX,
        Opcode::ASL_ABX,
        Opcode::LSR_ABX,
        Opcode::ROL_ABX,
        Opcode::ROR_ABX,
    ];
    const ABY_OP:[Opcode; 9] = [
        Opcode::LDA_ABY,
        Opcode::LDX_ABY,
        Opcode::STA_ABY,
        Opcode::AND_ABY,
        Opcode::EOR_ABY,
        Opcode::ORA_ABY,
        Opcode::ADC_ABY,
        Opcode::SBC_ABY,
        Opcode::CMP_ABY,
    ];
        
    pub fn get_addr_mode(op: &Opcode) -> Option<AddrMode> {

        match op {
            x if Self::IMM_OP.contains(x) => Some(AddrMode::IMM),            
            x if Self::ZPG_OP.contains(x) => Some(AddrMode::ZPG),
            x if Self::ZPX_OP.contains(x) => Some(AddrMode::ZPX),
            x if Self::ZPY_OP.contains(x) => Some(AddrMode::ZPY),
            x if Self::ABS_OP.contains(x) => Some(AddrMode::ABS),
            x if Self::ABX_OP.contains(x) => Some(AddrMode::ABX),
            x if Self::ABY_OP.contains(x) => Some(AddrMode::ABY),
            _ => None
        }
    }
}

// TODO: There is a lo of redundancy in the opcodes; is there a better way to deal with this
// classification system?
// impl Opcode {
//

//
// ZPG = 0x01,
//     ZPX = 0x02,
//     ABS = 0x03,
//     ABX = 0x04,
//     ABY = 0x05,
//     INX = 0x06,
//     INY = 0x07,
//
//     fn get_addressing_mode(&self) -> Option<AddrMode> {
//         match self {
//             x if Self::IMM_OP.contains(x) => Some(AddrMode::IMM),        
//             x if Self::ZP
//             _ => None
//         }
//     }
//
// }
//
// TODO: 
// Iterate through each opcode and make sure going from opcode to int and int to opcode gives what
// we expect
