// TODO: there is an error with the test cases;
// the program counter should not be incremented at any point when we access memory, only during
// so need to go back and double check my use of fetch_byte() because that increments the program
// counter and it shouldn't; those things should instead just be a simple memory access
// code execution / byte fetches 
// TODO: maybe add an opcode size to the table 
// 
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
        let mut cpu = CPU::new();
        cpu.x = 0x10;
        cpu.memory[0x0] = Opcode::LDA_ZPX as u8;
        cpu.memory[0x1] = 0x10;
        cpu.memory[0x20] = 0x41;
        cpu.execute();
        assert_eq!(cpu.ac, 0x41);

        cpu.pc = 0x0; // testing wrap around
        cpu.x = 0xff;
        let mem_loc = cpu.x.wrapping_add(0x10);
        cpu.memory[0xf] = 0x41;
        assert_eq!(cpu.ac, 0x41);
    }

    #[test]
    fn test_lda_ab() {
        let mut cpu = CPU::new();
        cpu.memory[0x0] = Opcode::LDA_ABS as u8;
        cpu.memory[0x1] = 0x10;
        cpu.memory[0x2] = 0xff;
        cpu.memory[0xff10] = 0x41;
        cpu.execute();
        assert_eq!(cpu.ac, 0x41);

        cpu.memory[0x0] = Opcode::LDA_ABX as u8;
        cpu.pc = 0x0;
        cpu.x = 0xff;
        cpu.memory[0x1] = 0xaa;
        cpu.memory[0x2] = 0xff;
        cpu.memory[0xa9] = 0x41;
        assert_eq!(cpu.ac, 0x41);
            
    }

    #[test]
    fn test_lda_idx() {
        let mut cpu = CPU::new();
        let old_pc = cpu.pc;
        cpu.x = 0x00;
        cpu.memory[0x0] = Opcode::LDA_IDX as u8;
        cpu.memory[0x1] = 0x10;
        cpu.memory[0x10] = 0xff;
        cpu.memory[0x11] = 0x00;
        cpu.memory[0x00ff] = 0x41;
        cpu.execute();
        let new_pc = cpu.pc;
        assert_eq!(cpu.ac, 0x41);
        assert_eq!(new_pc - old_pc, 2); // only 2 bytes of execution

    }

    #[test]
    fn test_lda_idy() {
        let mut cpu = CPU::new();
        let old_pc = cpu.pc;
        cpu.y = 0x90;
        // this test is incorrect
        cpu.memory[0x0] = Opcode::LDA_IDY as u8;
        cpu.memory[0x1] = 0x10;
        cpu.memory[0x10] = 0x80;
        cpu.memory[0x11] = 0x02;
        cpu.memory[0x0310] = 0x41;
        cpu.execute();
        let new_pc = cpu.pc;
        assert_eq!(cpu.ac, 0x41);
        assert_eq!(new_pc - old_pc, 2); // only 2 bytes of execution
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
