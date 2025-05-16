#![allow(warnings)]
use mos::emulator::Opcode;
use mos::emulator::CPU;


fn dbg_setup(cpu: &mut CPU) {
    for i in 0..0x10000 {
        cpu.memory[i] = (i % 256) as u8;
    }
}

fn main() {
    println!("hello world");
    let mut cpu = CPU::new();
    cpu.reset();
    cpu.memory[0x4141] = 0x41;
    dbg_setup(&mut cpu);

    let op: u8 = Opcode::LDA_IMM as u8;
    // println!("Opcode for LDA_IMM: {op}");
    // println!("{}", cpu.imm());
    // println!("{}", cpu.imm());
    // println!("{}", cpu.imm());
    // println!("zpg {}", cpu.zpg()); 
    // let check = *cpu.zpg();
    // let check = *cpu.zpg();
    // let check = *cpu.zpg();
    // *cpu.zpg() = 0x43;
    // cpu.pc -=1;
    // let check = *cpu.zpg();
    // println!("{}", check);

    // cpu.execute();

    println!("hi Accumulator: {}", cpu.ac);
}




