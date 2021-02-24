use std::ops::DerefMut;

use cpu::CPU;

mod cpu;
mod instructions;


fn main() {
    let mut cpu = CPU::new();
    let memory_lock = cpu.memory.clone();
    let mut mem = memory_lock.lock().unwrap();
    mem[5] = 0x50;
    mem[6] = -0x05;
    println!("{:#}", -0x05 as i8);
    drop(mem);
    cpu.run();
}
