#![allow(non_snake_case)]
use std::hash::Hash;

 #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Inst {
    LDA = 0x00,
    LDX = 0x01,
    LDY = 0x02,
    STA = 0x03,
    STX = 0x04,
    STY = 0x05,
    TAX = 0x06,
    TAY = 0x07,
    TXA = 0x08,
    TYA = 0x09,
    TSX = 0x0A,
    TXS = 0x0B,
    PHA = 0x0C,
    PHP = 0x0D,
    PLA = 0x0E,
    PLP = 0x0F,
    AND = 0x10,
    EOR = 0x11,
    ORA = 0x12,
    BIT = 0x13,
    ADC = 0x14,
    SBC = 0x15,
    CMP = 0x16,
    CPX = 0x17,
    CPY = 0x18,
    INC = 0x19,
    INX = 0x1A,
    INY = 0x1B,
    DEC = 0x1C,
    DEX = 0x1D,
    DEY = 0x1E,
    ASL = 0x1F,
    LSR = 0x20,
    ROL = 0x21,
    ROR = 0x22,
    JMP = 0x23,
    JSR = 0x24,
    RTS = 0x25,
    BCC = 0x26,
    BCS = 0x27,
    BEQ = 0x28,
    BMI = 0x29,
    BNE = 0x2A,
    BPL = 0x2B,
    BVC = 0x2C,
    BVS = 0x2D,
    CLC = 0x2E,
    CLD = 0x2F,
    CLI = 0x30,
    CLV = 0x31,
    SEC = 0x32,
    SED = 0x33,
    SEI = 0x34,
    BRK = 0x35,
    NOP = 0x36,
    RTI = 0x37,
    ILL = 0x38,
}


#[derive(PartialEq, Eq, Copy, Clone)]
pub enum AddrMode {
    IMP = 0x00, // implied
    ACC = 0x01, // accumulator
    IMM = 0x02, // immediate
    ZPG = 0x03, // zero page
    ZPX = 0x04, // zero page, x
    ZPY = 0x05, // zero page, y
    REL = 0x06, // relative
    ABS = 0x07, // absolute
    ABX = 0x08, // absolute, x
    ABY = 0x09, // absolute, y
    IND = 0x0a, // indirect
    IDX = 0x0b, // indexed indirect: (Indirect, X) e.g. LDA($40,X)
    IDY = 0x0c, // indirect indexed: (Indirect), Y e.g. LDA($40),Y
    ILL = 0x0d, // illegal operation
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Opcode {
    BRK_IMP = 0x0,
    ORA_IDX = 0x1,
    ORA_ZPG = 0x5,
    ASL_ZPG = 0x6,
    PHP_IMP = 0x8,
    ORA_IMM = 0x9,
    ASL_ACC = 0xa,
    ORA_ABS = 0xd,
    ASL_ABS = 0xe,
    BPL_REL = 0x10,
    ORA_IDY = 0x11,
    ORA_ZPX = 0x15,
    ASL_ZPX = 0x16,
    CLC_IMP = 0x18,
    ORA_ABY = 0x19,
    ORA_ABX = 0x1d,
    ASL_ABX = 0x1e,
    JSR_ABS = 0x20,
    AND_IDX = 0x21,
    BIT_ZPG = 0x24,
    AND_ZPG = 0x25,
    ROL_ZPG = 0x26,
    PLP_IMP = 0x28,
    AND_IMM = 0x29,
    ROL_ACC = 0x2a,
    BIT_ABS = 0x2c,
    AND_ABS = 0x2d,
    ROL_ABS = 0x2e,
    BMI_REL = 0x30,
    AND_IDY = 0x31,
    AND_ZPX = 0x35,
    ROL_ZPX = 0x36,
    SEC_IMP = 0x38,
    AND_ABY = 0x39,
    AND_ABX = 0x3d,
    ROL_ABX = 0x3e,
    RTI_IMP = 0x40,
    EOR_IDX = 0x41,
    EOR_ZPG = 0x45,
    LSR_ZPG = 0x46,
    PHA_IMP = 0x48,
    EOR_IMM = 0x49,
    LSR_ACC = 0x4a,
    JMP_ABS = 0x4c,
    EOR_ABS = 0x4d,
    LSR_ABS = 0x4e,
    BVC_REL = 0x50,
    EOR_IDY = 0x51,
    EOR_ZPX = 0x55,
    LSR_ZPX = 0x56,
    CLI_IMP = 0x58,
    EOR_ABY = 0x59,
    EOR_ABX = 0x5d,
    LSR_ABX = 0x5e,
    RTS_IMP = 0x60,
    ADC_IDX = 0x61,
    ADC_ZPG = 0x65,
    ROR_ZPG = 0x66,
    PLA_IMP = 0x68,
    ADC_IMM = 0x69,
    ROR_ACC = 0x6a,
    JMP_IND = 0x6c,
    ADC_ABS = 0x6d,
    ROR_ABS = 0x6e,
    BVS_REL = 0x70,
    ADC_IDY = 0x71,
    ADC_ZPX = 0x75,
    ROR_ZPX = 0x76,
    SEI_IMP = 0x78,
    ADC_ABY = 0x79,
    ADC_ABX = 0x7d,
    ROR_ABX = 0x7e,
    STA_IDX = 0x81,
    STY_ZPG = 0x84,
    STA_ZPG = 0x85,
    STX_ZPG = 0x86,
    DEY_IMP = 0x88,
    TXA_IMP = 0x8a,
    STY_ABS = 0x8c,
    STA_ABS = 0x8d,
    STX_ABS = 0x8e,
    BCC_REL = 0x90,
    STA_IDY = 0x91,
    STY_ZPX = 0x94,
    STA_ZPX = 0x95,
    STX_ZPY = 0x96,
    STA_ABY = 0x99,
    TXS_IMP = 0x9a,
    STA_ABX = 0x9d,
    LDY_IMM = 0xa0,
    LDA_IDX = 0xa1,
    LDX_IMM = 0xa2,
    LDY_ZPG = 0xa4,
    LDA_ZPG = 0xa5,
    LDX_ZPG = 0xa6,
    TAY_IMP = 0xa8,
    LDA_IMM = 0xa9,
    TAX_IMP = 0xaa,
    LDY_ABS = 0xac,
    LDA_ABS = 0xad,
    LDX_ABS = 0xae,
    BCS_REL = 0xb0,
    LDA_IDY = 0xb1,
    LDY_ZPX = 0xb4,
    LDA_ZPX = 0xb5,
    LDX_ZPY = 0xb6,
    CLV_IMP = 0xb8,
    LDA_ABY = 0xb9,
    TSX_IMP = 0xba,
    LDY_ABX = 0xbc,
    LDA_ABX = 0xbd,
    LDX_ABY = 0xbe,
    CPY_IMM = 0xc0,
    CMP_IDX = 0xc1,
    CPY_ZPG = 0xc4,
    CMP_ZPG = 0xc5,
    DEC_ZPG = 0xc6,
    INY_IMP = 0xc8,
    CMP_IMM = 0xc9,
    DEX_IMP = 0xca,
    CPY_ABS = 0xcc,
    CMP_ABS = 0xcd,
    DEC_ABS = 0xce,
    BNE_REL = 0xd0,
    CMP_IDY = 0xd1,
    CMP_ZPX = 0xd5,
    DEC_ZPX = 0xd6,
    CLD_IMP = 0xd8,
    CMP_ABY = 0xd9,
    CMP_ABX = 0xdd,
    DEC_ABX = 0xde,
    CPX_IMM = 0xe0,
    SBC_IDX = 0xe1,
    CPX_ZPG = 0xe4,
    SBC_ZPG = 0xe5,
    INC_ZPG = 0xe6,
    INX_IMP = 0xe8,
    SBC_IMM = 0xe9,
    NOP_IMP = 0xea,
    CPX_ABS = 0xec,
    SBC_ABS = 0xed,
    INC_ABS = 0xee,
    BEQ_REL = 0xf0,
    SBC_IDY = 0xf1,
    SBC_ZPX = 0xf5,
    INC_ZPX = 0xf6,
    SED_IMP = 0xf8,
    SBC_ABY = 0xf9,
    SBC_ABX = 0xfd,
    INC_ABX = 0xfe,
    ILL_OP = 0xff,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Opcode::BRK_IMP,
            0x1 => Opcode::ORA_IDX,
            0x2 => Opcode::ILL_OP,
            0x3 => Opcode::ILL_OP,
            0x4 => Opcode::ILL_OP,
            0x5 => Opcode::ORA_ZPG,
            0x6 => Opcode::ASL_ZPG,
            0x7 => Opcode::ILL_OP,
            0x8 => Opcode::PHP_IMP,
            0x9 => Opcode::ORA_IMM,
            0xa => Opcode::ASL_ACC,
            0xb => Opcode::ILL_OP,
            0xc => Opcode::ILL_OP,
            0xd => Opcode::ORA_ABS,
            0xe => Opcode::ASL_ABS,
            0xf => Opcode::ILL_OP,
            0x10 => Opcode::BPL_REL,
            0x11 => Opcode::ORA_IDY,
            0x12 => Opcode::ILL_OP,
            0x13 => Opcode::ILL_OP,
            0x14 => Opcode::ILL_OP,
            0x15 => Opcode::ORA_ZPX,
            0x16 => Opcode::ASL_ZPX,
            0x17 => Opcode::ILL_OP,
            0x18 => Opcode::CLC_IMP,
            0x19 => Opcode::ORA_ABY,
            0x1a => Opcode::ILL_OP,
            0x1b => Opcode::ILL_OP,
            0x1c => Opcode::ILL_OP,
            0x1d => Opcode::ORA_ABX,
            0x1e => Opcode::ASL_ABX,
            0x1f => Opcode::ILL_OP,
            0x20 => Opcode::JSR_ABS,
            0x21 => Opcode::AND_IDX,
            0x22 => Opcode::ILL_OP,
            0x23 => Opcode::ILL_OP,
            0x24 => Opcode::BIT_ZPG,
            0x25 => Opcode::AND_ZPG,
            0x26 => Opcode::ROL_ZPG,
            0x27 => Opcode::ILL_OP,
            0x28 => Opcode::PLP_IMP,
            0x29 => Opcode::AND_IMM,
            0x2a => Opcode::ROL_ACC,
            0x2b => Opcode::ILL_OP,
            0x2c => Opcode::BIT_ABS,
            0x2d => Opcode::AND_ABS,
            0x2e => Opcode::ROL_ABS,
            0x2f => Opcode::ILL_OP,
            0x30 => Opcode::BMI_REL,
            0x31 => Opcode::AND_IDY,
            0x32 => Opcode::ILL_OP,
            0x33 => Opcode::ILL_OP,
            0x34 => Opcode::ILL_OP,
            0x35 => Opcode::AND_ZPX,
            0x36 => Opcode::ROL_ZPX,
            0x37 => Opcode::ILL_OP,
            0x38 => Opcode::SEC_IMP,
            0x39 => Opcode::AND_ABY,
            0x3a => Opcode::ILL_OP,
            0x3b => Opcode::ILL_OP,
            0x3c => Opcode::ILL_OP,
            0x3d => Opcode::AND_ABX,
            0x3e => Opcode::ROL_ABX,
            0x3f => Opcode::ILL_OP,
            0x40 => Opcode::RTI_IMP,
            0x41 => Opcode::EOR_IDX,
            0x42 => Opcode::ILL_OP,
            0x43 => Opcode::ILL_OP,
            0x44 => Opcode::ILL_OP,
            0x45 => Opcode::EOR_ZPG,
            0x46 => Opcode::LSR_ZPG,
            0x47 => Opcode::ILL_OP,
            0x48 => Opcode::PHA_IMP,
            0x49 => Opcode::EOR_IMM,
            0x4a => Opcode::LSR_ACC,
            0x4b => Opcode::ILL_OP,
            0x4c => Opcode::JMP_ABS,
            0x4d => Opcode::EOR_ABS,
            0x4e => Opcode::LSR_ABS,
            0x4f => Opcode::ILL_OP,
            0x50 => Opcode::BVC_REL,
            0x51 => Opcode::EOR_IDY,
            0x52 => Opcode::ILL_OP,
            0x53 => Opcode::ILL_OP,
            0x54 => Opcode::ILL_OP,
            0x55 => Opcode::EOR_ZPX,
            0x56 => Opcode::LSR_ZPX,
            0x57 => Opcode::ILL_OP,
            0x58 => Opcode::CLI_IMP,
            0x59 => Opcode::EOR_ABY,
            0x5a => Opcode::ILL_OP,
            0x5b => Opcode::ILL_OP,
            0x5c => Opcode::ILL_OP,
            0x5d => Opcode::EOR_ABX,
            0x5e => Opcode::LSR_ABX,
            0x5f => Opcode::ILL_OP,
            0x60 => Opcode::RTS_IMP,
            0x61 => Opcode::ADC_IDX,
            0x62 => Opcode::ILL_OP,
            0x63 => Opcode::ILL_OP,
            0x64 => Opcode::ILL_OP,
            0x65 => Opcode::ADC_ZPG,
            0x66 => Opcode::ROR_ZPG,
            0x67 => Opcode::ILL_OP,
            0x68 => Opcode::PLA_IMP,
            0x69 => Opcode::ADC_IMM,
            0x6a => Opcode::ROR_ACC,
            0x6b => Opcode::ILL_OP,
            0x6c => Opcode::JMP_IND,
            0x6d => Opcode::ADC_ABS,
            0x6e => Opcode::ROR_ABS,
            0x6f => Opcode::ILL_OP,
            0x70 => Opcode::BVS_REL,
            0x71 => Opcode::ADC_IDY,
            0x72 => Opcode::ILL_OP,
            0x73 => Opcode::ILL_OP,
            0x74 => Opcode::ILL_OP,
            0x75 => Opcode::ADC_ZPX,
            0x76 => Opcode::ROR_ZPX,
            0x77 => Opcode::ILL_OP,
            0x78 => Opcode::SEI_IMP,
            0x79 => Opcode::ADC_ABY,
            0x7a => Opcode::ILL_OP,
            0x7b => Opcode::ILL_OP,
            0x7c => Opcode::ILL_OP,
            0x7d => Opcode::ADC_ABX,
            0x7e => Opcode::ROR_ABX,
            0x7f => Opcode::ILL_OP,
            0x80 => Opcode::ILL_OP,
            0x81 => Opcode::STA_IDX,
            0x82 => Opcode::ILL_OP,
            0x83 => Opcode::ILL_OP,
            0x84 => Opcode::STY_ZPG,
            0x85 => Opcode::STA_ZPG,
            0x86 => Opcode::STX_ZPG,
            0x87 => Opcode::ILL_OP,
            0x88 => Opcode::DEY_IMP,
            0x89 => Opcode::ILL_OP,
            0x8a => Opcode::TXA_IMP,
            0x8b => Opcode::ILL_OP,
            0x8c => Opcode::STY_ABS,
            0x8d => Opcode::STA_ABS,
            0x8e => Opcode::STX_ABS,
            0x8f => Opcode::ILL_OP,
            0x90 => Opcode::BCC_REL,
            0x91 => Opcode::STA_IDY,
            0x92 => Opcode::ILL_OP,
            0x93 => Opcode::ILL_OP,
            0x94 => Opcode::STY_ZPX,
            0x95 => Opcode::STA_ZPX,
            0x96 => Opcode::STX_ZPY,
            0x97 => Opcode::ILL_OP,
            0x98 => Opcode::ILL_OP,
            0x99 => Opcode::STA_ABY,
            0x9a => Opcode::TXS_IMP,
            0x9b => Opcode::ILL_OP,
            0x9c => Opcode::ILL_OP,
            0x9d => Opcode::STA_ABX,
            0x9e => Opcode::ILL_OP,
            0x9f => Opcode::ILL_OP,
            0xa0 => Opcode::LDY_IMM,
            0xa1 => Opcode::LDA_IDX,
            0xa2 => Opcode::LDX_IMM,
            0xa3 => Opcode::ILL_OP,
            0xa4 => Opcode::LDY_ZPG,
            0xa5 => Opcode::LDA_ZPG,
            0xa6 => Opcode::LDX_ZPG,
            0xa7 => Opcode::ILL_OP,
            0xa8 => Opcode::TAY_IMP,
            0xa9 => Opcode::LDA_IMM,
            0xaa => Opcode::TAX_IMP,
            0xab => Opcode::ILL_OP,
            0xac => Opcode::LDY_ABS,
            0xad => Opcode::LDA_ABS,
            0xae => Opcode::LDX_ABS,
            0xaf => Opcode::ILL_OP,
            0xb0 => Opcode::BCS_REL,
            0xb1 => Opcode::LDA_IDY,
            0xb2 => Opcode::ILL_OP,
            0xb3 => Opcode::ILL_OP,
            0xb4 => Opcode::LDY_ZPX,
            0xb5 => Opcode::LDA_ZPX,
            0xb6 => Opcode::LDX_ZPY,
            0xb7 => Opcode::ILL_OP,
            0xb8 => Opcode::CLV_IMP,
            0xb9 => Opcode::LDA_ABY,
            0xba => Opcode::TSX_IMP,
            0xbb => Opcode::ILL_OP,
            0xbc => Opcode::LDY_ABX,
            0xbd => Opcode::LDA_ABX,
            0xbe => Opcode::LDX_ABY,
            0xbf => Opcode::ILL_OP,
            0xc0 => Opcode::CPY_IMM,
            0xc1 => Opcode::CMP_IDX,
            0xc2 => Opcode::ILL_OP,
            0xc3 => Opcode::ILL_OP,
            0xc4 => Opcode::CPY_ZPG,
            0xc5 => Opcode::CMP_ZPG,
            0xc6 => Opcode::DEC_ZPG,
            0xc7 => Opcode::ILL_OP,
            0xc8 => Opcode::INY_IMP,
            0xc9 => Opcode::CMP_IMM,
            0xca => Opcode::DEX_IMP,
            0xcb => Opcode::ILL_OP,
            0xcc => Opcode::CPY_ABS,
            0xcd => Opcode::CMP_ABS,
            0xce => Opcode::DEC_ABS,
            0xcf => Opcode::ILL_OP,
            0xd0 => Opcode::BNE_REL,
            0xd1 => Opcode::CMP_IDY,
            0xd2 => Opcode::ILL_OP,
            0xd3 => Opcode::ILL_OP,
            0xd4 => Opcode::ILL_OP,
            0xd5 => Opcode::CMP_ZPX,
            0xd6 => Opcode::DEC_ZPX,
            0xd7 => Opcode::ILL_OP,
            0xd8 => Opcode::CLD_IMP,
            0xd9 => Opcode::CMP_ABY,
            0xda => Opcode::ILL_OP,
            0xdb => Opcode::ILL_OP,
            0xdc => Opcode::ILL_OP,
            0xdd => Opcode::CMP_ABX,
            0xde => Opcode::DEC_ABX,
            0xdf => Opcode::ILL_OP,
            0xe0 => Opcode::CPX_IMM,
            0xe1 => Opcode::SBC_IDX,
            0xe2 => Opcode::ILL_OP,
            0xe3 => Opcode::ILL_OP,
            0xe4 => Opcode::CPX_ZPG,
            0xe5 => Opcode::SBC_ZPG,
            0xe6 => Opcode::INC_ZPG,
            0xe7 => Opcode::ILL_OP,
            0xe8 => Opcode::INX_IMP,
            0xe9 => Opcode::SBC_IMM,
            0xea => Opcode::NOP_IMP,
            0xeb => Opcode::ILL_OP,
            0xec => Opcode::CPX_ABS,
            0xed => Opcode::SBC_ABS,
            0xee => Opcode::INC_ABS,
            0xef => Opcode::ILL_OP,
            0xf0 => Opcode::BEQ_REL,
            0xf1 => Opcode::SBC_IDY,
            0xf2 => Opcode::ILL_OP,
            0xf3 => Opcode::ILL_OP,
            0xf4 => Opcode::ILL_OP,
            0xf5 => Opcode::SBC_ZPX,
            0xf6 => Opcode::INC_ZPX,
            0xf7 => Opcode::ILL_OP,
            0xf8 => Opcode::SED_IMP,
            0xf9 => Opcode::SBC_ABY,
            0xfa => Opcode::ILL_OP,
            0xfb => Opcode::ILL_OP,
            0xfc => Opcode::ILL_OP,
            0xfd => Opcode::SBC_ABX,
            0xfe => Opcode::INC_ABX,
            0xff => Opcode::ILL_OP,
        }
    } 
}

pub struct OpWrapper {
    pub op: Opcode, // TODO: i dont think this is needed because we can do a conversion from the u8 value
    pub inst: Inst,
    pub addr_mode: AddrMode,
    pub cycle: u8,
    pub flags: u8,
}

impl OpWrapper {
    pub fn new(op: Opcode, addr_mode: AddrMode, cycle: u8, inst: Inst, flags: u8) -> Self {
        Self {
            op: op,
            inst: inst,
            addr_mode: addr_mode,
            cycle: cycle,
            flags: flags
        }
    }
}


pub const opcode_arr: [Opcode; 256] = [
    Opcode::BRK_IMP,Opcode::ORA_IDX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ORA_ZPG,Opcode::ASL_ZPG,Opcode::ILL_OP,Opcode::PHP_IMP,Opcode::ORA_IMM,Opcode::ASL_ACC,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ORA_ABS,Opcode::ASL_ABS,Opcode::ILL_OP,Opcode::BPL_REL,Opcode::ORA_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ORA_ZPX,Opcode::ASL_ZPX,Opcode::ILL_OP,Opcode::CLC_IMP,Opcode::ORA_ABY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ORA_ABX,Opcode::ASL_ABX,Opcode::ILL_OP,Opcode::JSR_ABS,Opcode::AND_IDX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::BIT_ZPG,Opcode::AND_ZPG,Opcode::ROL_ZPG,Opcode::ILL_OP,Opcode::PLP_IMP,Opcode::AND_IMM,Opcode::ROL_ACC,Opcode::ILL_OP,Opcode::BIT_ABS,Opcode::AND_ABS,Opcode::ROL_ABS,Opcode::ILL_OP,Opcode::BMI_REL,Opcode::AND_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::AND_ZPX,Opcode::ROL_ZPX,Opcode::ILL_OP,Opcode::SEC_IMP,Opcode::AND_ABY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::AND_ABX,Opcode::ROL_ABX,Opcode::ILL_OP,Opcode::RTI_IMP,Opcode::EOR_IDX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::EOR_ZPG,Opcode::LSR_ZPG,Opcode::ILL_OP,Opcode::PHA_IMP,Opcode::EOR_IMM,Opcode::LSR_ACC,Opcode::ILL_OP,Opcode::JMP_ABS,Opcode::EOR_ABS,Opcode::LSR_ABS,Opcode::ILL_OP,Opcode::BVC_REL,Opcode::EOR_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::EOR_ZPX,Opcode::LSR_ZPX,Opcode::ILL_OP,Opcode::CLI_IMP,Opcode::EOR_ABY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::EOR_ABX,Opcode::LSR_ABX,Opcode::ILL_OP,Opcode::RTS_IMP,Opcode::ADC_IDX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ADC_ZPG,Opcode::ROR_ZPG,Opcode::ILL_OP,Opcode::PLA_IMP,Opcode::ADC_IMM,Opcode::ROR_ACC,Opcode::ILL_OP,Opcode::JMP_IND,Opcode::ADC_ABS,Opcode::ROR_ABS,Opcode::ILL_OP,Opcode::BVS_REL,Opcode::ADC_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ADC_ZPX,Opcode::ROR_ZPX,Opcode::ILL_OP,Opcode::SEI_IMP,Opcode::ADC_ABY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ADC_ABX,Opcode::ROR_ABX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::STA_IDX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::STY_ZPG,Opcode::STA_ZPG,Opcode::STX_ZPG,Opcode::ILL_OP,Opcode::DEY_IMP,Opcode::ILL_OP,Opcode::TXA_IMP,Opcode::ILL_OP,Opcode::STY_ABS,Opcode::STA_ABS,Opcode::STX_ABS,Opcode::ILL_OP,Opcode::BCC_REL,Opcode::STA_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::STY_ZPX,Opcode::STA_ZPX,Opcode::STX_ZPY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::STA_ABY,Opcode::TXS_IMP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::STA_ABX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::LDY_IMM,Opcode::LDA_IDX,Opcode::LDX_IMM,Opcode::ILL_OP,Opcode::LDY_ZPG,Opcode::LDA_ZPG,Opcode::LDX_ZPG,Opcode::ILL_OP,Opcode::TAY_IMP,Opcode::LDA_IMM,Opcode::TAX_IMP,Opcode::ILL_OP,Opcode::LDY_ABS,Opcode::LDA_ABS,Opcode::LDX_ABS,Opcode::ILL_OP,Opcode::BCS_REL,Opcode::LDA_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::LDY_ZPX,Opcode::LDA_ZPX,Opcode::LDX_ZPY,Opcode::ILL_OP,Opcode::CLV_IMP,Opcode::LDA_ABY,Opcode::TSX_IMP,Opcode::ILL_OP,Opcode::LDY_ABX,Opcode::LDA_ABX,Opcode::LDX_ABY,Opcode::ILL_OP,Opcode::CPY_IMM,Opcode::CMP_IDX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::CPY_ZPG,Opcode::CMP_ZPG,Opcode::DEC_ZPG,Opcode::ILL_OP,Opcode::INY_IMP,Opcode::CMP_IMM,Opcode::DEX_IMP,Opcode::ILL_OP,Opcode::CPY_ABS,Opcode::CMP_ABS,Opcode::DEC_ABS,Opcode::ILL_OP,Opcode::BNE_REL,Opcode::CMP_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::CMP_ZPX,Opcode::DEC_ZPX,Opcode::ILL_OP,Opcode::CLD_IMP,Opcode::CMP_ABY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::CMP_ABX,Opcode::DEC_ABX,Opcode::ILL_OP,Opcode::CPX_IMM,Opcode::SBC_IDX,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::CPX_ZPG,Opcode::SBC_ZPG,Opcode::INC_ZPG,Opcode::ILL_OP,Opcode::INX_IMP,Opcode::SBC_IMM,Opcode::NOP_IMP,Opcode::ILL_OP,Opcode::CPX_ABS,Opcode::SBC_ABS,Opcode::INC_ABS,Opcode::ILL_OP,Opcode::BEQ_REL,Opcode::SBC_IDY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::SBC_ZPX,Opcode::INC_ZPX,Opcode::ILL_OP,Opcode::SED_IMP,Opcode::SBC_ABY,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::ILL_OP,Opcode::SBC_ABX,Opcode::INC_ABX,Opcode::ILL_OP,
];
pub const addrmode_arr: [AddrMode; 256] = [
    AddrMode::IMP,AddrMode::IDX,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::IMM,AddrMode::ACC,AddrMode::ILL,AddrMode::ILL,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ILL,AddrMode::IMP,AddrMode::ABY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ABX,AddrMode::ABX,AddrMode::ILL,AddrMode::ABS,AddrMode::IDX,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::IMM,AddrMode::ACC,AddrMode::ILL,AddrMode::ABS,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ILL,AddrMode::IMP,AddrMode::ABY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ABX,AddrMode::ABX,AddrMode::ILL,AddrMode::IMP,AddrMode::IDX,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::IMM,AddrMode::ACC,AddrMode::ILL,AddrMode::ABS,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ILL,AddrMode::IMP,AddrMode::ABY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ABX,AddrMode::ABX,AddrMode::ILL,AddrMode::IMP,AddrMode::IDX,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::IMM,AddrMode::ACC,AddrMode::ILL,AddrMode::IND,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ILL,AddrMode::IMP,AddrMode::ABY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ABX,AddrMode::ABX,AddrMode::ILL,AddrMode::ILL,AddrMode::IDX,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::ILL,AddrMode::IMP,AddrMode::ILL,AddrMode::ABS,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ZPY,AddrMode::ILL,AddrMode::ILL,AddrMode::ABY,AddrMode::IMP,AddrMode::ILL,AddrMode::ILL,AddrMode::ABX,AddrMode::ILL,AddrMode::ILL,AddrMode::IMM,AddrMode::IDX,AddrMode::IMM,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::IMM,AddrMode::IMP,AddrMode::ILL,AddrMode::ABS,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ZPY,AddrMode::ILL,AddrMode::IMP,AddrMode::ABY,AddrMode::IMP,AddrMode::ILL,AddrMode::ABX,AddrMode::ABX,AddrMode::ABY,AddrMode::ILL,AddrMode::IMM,AddrMode::IDX,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::IMM,AddrMode::IMP,AddrMode::ILL,AddrMode::ABS,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ILL,AddrMode::IMP,AddrMode::ABY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ABX,AddrMode::ABX,AddrMode::ILL,AddrMode::IMM,AddrMode::IDX,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ZPG,AddrMode::ILL,AddrMode::IMP,AddrMode::IMM,AddrMode::IMP,AddrMode::ILL,AddrMode::ABS,AddrMode::ABS,AddrMode::ABS,AddrMode::ILL,AddrMode::REL,AddrMode::IDY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ZPX,AddrMode::ZPX,AddrMode::ILL,AddrMode::IMP,AddrMode::ABY,AddrMode::ILL,AddrMode::ILL,AddrMode::ILL,AddrMode::ABX,AddrMode::ABX,AddrMode::ILL,
];

pub const cycle_arr: [u8; 256] = [7,6,0,0,0,3,5,0,3,2,2,0,0,4,6,0,2,5,0,0,0,4,6,0,2,4,0,0,0,4,7,0,6,6,0,0,3,3,5,0,4,2,2,0,4,4,6,0,2,5,0,0,0,4,6,0,2,4,0,0,0,4,7,0,6,6,0,0,0,3,5,0,3,2,2,0,3,4,6,0,2,5,0,0,0,4,6,0,2,4,0,0,0,4,7,0,6,6,0,0,0,3,5,0,4,2,2,0,5,4,6,0,2,5,0,0,0,4,6,0,2,4,0,0,0,4,7,0,0,6,0,0,3,3,3,0,2,0,2,0,4,4,4,0,2,6,0,0,4,4,4,0,0,5,2,0,0,5,0,0,2,6,2,0,3,3,3,0,2,2,2,0,4,4,4,0,2,5,0,0,4,4,4,0,2,4,2,0,4,4,4,0,2,6,0,0,3,3,5,0,2,2,2,0,4,4,6,0,2,5,0,0,0,4,6,0,2,4,0,0,0,4,7,0,2,6,0,0,3,3,5,0,2,2,2,0,4,4,6,0,2,5,0,0,0,4,6,0,2,4,0,0,0,4,7,0,];

pub const flag_arr: [u8; 256] = [
    0,130,0,0,0,130,131,0,0,130,131,0,0,130,131,0,0,130,0,0,0,130,131,0,0,130,0,0,0,130,131,0,0,130,0,0,2,130,131,0,0,130,131,0,2,130,131,0,0,130,0,0,0,130,131,0,0,130,0,0,0,130,131,0,0,130,0,0,0,130,3,0,0,130,3,0,0,130,3,0,0,130,0,0,0,130,3,0,0,130,0,0,0,130,3,0,0,195,0,0,0,195,131,0,130,195,131,0,0,195,131,0,0,195,0,0,0,195,131,0,0,195,0,0,0,195,131,0,0,0,0,0,0,0,0,0,130,0,130,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,130,130,130,0,130,130,130,0,130,130,130,0,130,130,130,0,0,130,0,0,130,130,130,0,0,130,130,0,130,130,130,0,131,131,0,0,131,131,130,0,130,131,130,0,131,131,130,0,0,131,0,0,0,131,130,0,0,131,0,0,0,131,130,0,131,195,0,0,131,195,130,0,130,195,0,0,131,195,130,0,0,195,0,0,0,195,130,0,0,195,0,0,0,195,130,0,
];

pub const inst_arr: [Inst; 256] = [
    Inst::BRK,Inst::ORA,Inst::ILL,Inst::ILL,Inst::ILL,Inst::ORA,Inst::ASL,Inst::ILL,Inst::PHP,Inst::ORA,Inst::ASL,Inst::ILL,Inst::ILL,Inst::ORA,Inst::ASL,Inst::ILL,Inst::BPL,Inst::ORA,Inst::ILL,Inst::ILL,Inst::ILL,Inst::ORA,Inst::ASL,Inst::ILL,Inst::CLC,Inst::ORA,Inst::ILL,Inst::ILL,Inst::ILL,Inst::ORA,Inst::ASL,Inst::ILL,Inst::JSR,Inst::AND,Inst::ILL,Inst::ILL,Inst::BIT,Inst::AND,Inst::ROL,Inst::ILL,Inst::PLP,Inst::AND,Inst::ROL,Inst::ILL,Inst::BIT,Inst::AND,Inst::ROL,Inst::ILL,Inst::BMI,Inst::AND,Inst::ILL,Inst::ILL,Inst::ILL,Inst::AND,Inst::ROL,Inst::ILL,Inst::SEC,Inst::AND,Inst::ILL,Inst::ILL,Inst::ILL,Inst::AND,Inst::ROL,Inst::ILL,Inst::RTI,Inst::EOR,Inst::ILL,Inst::ILL,Inst::ILL,Inst::EOR,Inst::LSR,Inst::ILL,Inst::PHA,Inst::EOR,Inst::LSR,Inst::ILL,Inst::JMP,Inst::EOR,Inst::LSR,Inst::ILL,Inst::BVC,Inst::EOR,Inst::ILL,Inst::ILL,Inst::ILL,Inst::EOR,Inst::LSR,Inst::ILL,Inst::CLI,Inst::EOR,Inst::ILL,Inst::ILL,Inst::ILL,Inst::EOR,Inst::LSR,Inst::ILL,Inst::RTS,Inst::ADC,Inst::ILL,Inst::ILL,Inst::ILL,Inst::ADC,Inst::ROR,Inst::ILL,Inst::PLA,Inst::ADC,Inst::ROR,Inst::ILL,Inst::JMP,Inst::ADC,Inst::ROR,Inst::ILL,Inst::BVS,Inst::ADC,Inst::ILL,Inst::ILL,Inst::ILL,Inst::ADC,Inst::ROR,Inst::ILL,Inst::SEI,Inst::ADC,Inst::ILL,Inst::ILL,Inst::ILL,Inst::ADC,Inst::ROR,Inst::ILL,Inst::ILL,Inst::STA,Inst::ILL,Inst::ILL,Inst::STY,Inst::STA,Inst::STX,Inst::ILL,Inst::DEY,Inst::ILL,Inst::TXA,Inst::ILL,Inst::STY,Inst::STA,Inst::STX,Inst::ILL,Inst::BCC,Inst::STA,Inst::ILL,Inst::ILL,Inst::STY,Inst::STA,Inst::STX,Inst::ILL,Inst::ILL,Inst::STA,Inst::TXS,Inst::ILL,Inst::ILL,Inst::STA,Inst::ILL,Inst::ILL,Inst::LDY,Inst::LDA,Inst::LDX,Inst::ILL,Inst::LDY,Inst::LDA,Inst::LDX,Inst::ILL,Inst::TAY,Inst::LDA,Inst::TAX,Inst::ILL,Inst::LDY,Inst::LDA,Inst::LDX,Inst::ILL,Inst::BCS,Inst::LDA,Inst::ILL,Inst::ILL,Inst::LDY,Inst::LDA,Inst::LDX,Inst::ILL,Inst::CLV,Inst::LDA,Inst::TSX,Inst::ILL,Inst::LDY,Inst::LDA,Inst::LDX,Inst::ILL,Inst::CPY,Inst::CMP,Inst::ILL,Inst::ILL,Inst::CPY,Inst::CMP,Inst::DEC,Inst::ILL,Inst::INY,Inst::CMP,Inst::DEX,Inst::ILL,Inst::CPY,Inst::CMP,Inst::DEC,Inst::ILL,Inst::BNE,Inst::CMP,Inst::ILL,Inst::ILL,Inst::ILL,Inst::CMP,Inst::DEC,Inst::ILL,Inst::CLD,Inst::CMP,Inst::ILL,Inst::ILL,Inst::ILL,Inst::CMP,Inst::DEC,Inst::ILL,Inst::CPX,Inst::SBC,Inst::ILL,Inst::ILL,Inst::CPX,Inst::SBC,Inst::INC,Inst::ILL,Inst::INX,Inst::SBC,Inst::NOP,Inst::ILL,Inst::CPX,Inst::SBC,Inst::INC,Inst::ILL,Inst::BEQ,Inst::SBC,Inst::ILL,Inst::ILL,Inst::ILL,Inst::SBC,Inst::INC,Inst::ILL,Inst::SED,Inst::SBC,Inst::ILL,Inst::ILL,Inst::ILL,Inst::SBC,Inst::INC,Inst::ILL,
];
