#![allow(warnings)]
mod emulator;

fn main() {
    println!("hello world");
    let mut cpu = emulator::CPU::new();
    cpu.reset();
    cpu.memory[0x0] = 0xAD;
    cpu.memory[0x1] = 0x41;
    cpu.memory[0x2] = 0x40;
    cpu.memory[0x4041] = 100;

    cpu.memory[0x3] = 0x85;
    cpu.execute();
    println!("hi Accumulator: {}", cpu.ac);
}




