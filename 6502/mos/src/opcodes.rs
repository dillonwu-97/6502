pub enum Opcode {
    LDA = 0xA9,  // immediate load accumulator
    LDA_ZPG = 0xA5, // load zero page
    LDA_ZPX = 0xB5, // load value at zero page offset + x into accumulator
    LDA_ABS = 0xAD,
    LDA_ABX = 0xBD,
    LDA_ABY = 0xB9,
}
