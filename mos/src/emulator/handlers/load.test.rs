#[cfg(test)]
mod tests {
    use crate::emulator::CPU;
    use crate::emulator::Opcode;
    #[test]
    fn test_lda_imm() {
        let mut cpu = CPU::new();
        // why do I need use here when it's already declared at the top? v confused
        let mut cpu = CPU::new();
        cpu.reset();
        let op: u8 = Opcode::LDA_IMM as u8;
        cpu.memory[0x0] = op;
        cpu.memory[0x1] = 0x41;
        cpu.execute();
        assert_eq!(cpu.ac, 0x41);
    }

    #[test]
    fn test_lda_zpg() {
        // why do I need use here when it's already declared at the top? v confused
        let mut cpu = CPU::new();
        cpu.reset();
        let op: u8 = Opcode::LDA_ZPG as u8;
        cpu.memory[0x0] = op;
        cpu.memory[0x1] = 0x10;
        cpu.memory[0x10] = 0x41;
        cpu.execute();

        assert_eq!(cpu.ac, 0x41);
    }

    #[test]
    fn test_lda_zpx() {
        
    }


}



// Potential future test cases:
// status registers
// these are just test cases that i created for myself, are there other test cases online that i
// can just use instead with expected output / behavior
//
//
//
//
