#[cfg(test)]
mod tests {
    use crate::emulator::CPU;
    use crate::emulator::Opcode;
    use crate::emulator::cpu::StatusRegister;
    use crate::emulator::cpu::N;
    use crate::emulator::cpu::Z;
    use crate::emulator::cpu::C;
    use crate::emulator::cpu::V;

    fn setup_adc_imm(ac: i8, mem_val: i8) -> CPU {
        let a = ac as u8;
        let b = mem_val as u8;

        let mut cpu = CPU::new();
        let op: u8 = Opcode::ADC_IMM as u8;
        cpu.ac = a;
        cpu.memory[0x0] = op;
        cpu.memory[0x1] = b;
        cpu.execute();
        return cpu;
    }

    #[test]
    // 1 + 1
    fn test_add_1_1() {
        let mut cpu: CPU = setup_adc_imm(0x1, 0x1);
        assert_eq!(cpu.ac, 0x2);
        assert_eq!(false, cpu.get_status(StatusRegister::C));
        assert_eq!(false, cpu.get_status(StatusRegister::V));
    }

    #[test]
    // 1 + -1
    fn test_add_1_n1() {
        let mut cpu: CPU = setup_adc_imm(0x1, -0x1);
        assert_eq!(cpu.ac, 0x0);
        assert_eq!(true, cpu.get_status(StatusRegister::C));
        assert_eq!(false, cpu.get_status(StatusRegister::V));
    }

    #[test]
    // 127 + 1
    fn test_add_127_1() {
        let mut cpu: CPU = setup_adc_imm(0x1, 127);
        assert_eq!(cpu.ac, 128);
        assert_eq!(false, cpu.get_status(StatusRegister::C));
        assert_eq!(true, cpu.get_status(StatusRegister::V));
    }

    #[test]
    // -128 + -1
    fn test_add_n128_n1() {
        let mut cpu: CPU = setup_adc_imm(-0x1, -128);
        assert_eq!(cpu.ac, 127);
        assert_eq!(true, cpu.get_status(StatusRegister::C));
        assert_eq!(true, cpu.get_status(StatusRegister::V));
    }

    #[test]
    // 127 + -1
    fn test_add_127_n1() {
        let mut cpu: CPU = setup_adc_imm(-1, 127);
        assert_eq!(cpu.ac, 126);
        assert_eq!(true, cpu.get_status(StatusRegister::C));
        assert_eq!(false, cpu.get_status(StatusRegister::V));
    }

    #[test]
    // -128 + 1
    fn test_add_n128_1() {
        let mut cpu: CPU = setup_adc_imm(1, -128);
        assert_eq!(cpu.ac as i8, -127);
        assert_eq!(false, cpu.get_status(StatusRegister::C));
        assert_eq!(false, cpu.get_status(StatusRegister::V));
    }

    #[test]
    // 127 + 127
    fn test_add_127_127(){ 
        let mut cpu: CPU = setup_adc_imm(127, 127);
        assert_eq!(cpu.ac, 254);
        assert_eq!(false, cpu.get_status(StatusRegister::C));
        assert_eq!(true, cpu.get_status(StatusRegister::V));
    }

}
