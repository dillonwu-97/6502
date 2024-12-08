mod cpu;
mod opcodes;

use bitflags::bitflags;
use cpu::CPU;
use opcodes::Opcode;

fn main() {
    println!("hello world");
    let mut cpu = CPU::new();
    cpu.reset();
    cpu.memory[0x0] = 0xA9;
    cpu.memory[0x1] = 0x41;
    cpu.execute();
    println!("Accumulator: {}", cpu.ac);
}




