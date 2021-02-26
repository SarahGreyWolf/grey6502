use std::fs::read;

use cpu::CPU;

mod cpu;
mod instructions;


fn main() {
    // let mut cpu = CPU::new(std::time::Duration::from_nanos(1));
    let mut cpu = CPU::new(std::time::Duration::from_millis(100));
    let memory_lock = cpu.memory.clone();
    let mut mem = memory_lock.lock().unwrap();
    let data = read("./6502_functional_test.bin").unwrap();
    for i in 0..data.len() {
        mem[i] = data[i];
    }
    drop(mem);
    cpu.run();
}
