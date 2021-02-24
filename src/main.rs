use cpu::CPU;

mod cpu;
mod instructions;


fn main() {
    let mut cpu = CPU::new();
    let memory_lock = cpu.memory.clone();
    let mut mem = memory_lock.lock().unwrap();
    mem[2] = 0xA0;
    mem[3] = 0x05;

    mem[10] = 0x10;
    mem[11] = -0x02;
    drop(mem);
    cpu.run();
}
