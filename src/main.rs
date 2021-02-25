use cpu::CPU;

mod cpu;
mod instructions;


fn main() {
    // let mut cpu = CPU::new(std::time::Duration::from_nanos(1));
    let mut cpu = CPU::new(std::time::Duration::from_millis(1000));
    let memory_lock = cpu.memory.clone();
    let mut mem = memory_lock.lock().unwrap();
    mem[0x5000] = 0x10;
    mem[2] = 0xAC;
    mem[3] = 0x00;
    mem[4] = 0x50;
    mem[5] = 0x2C;
    mem[6] = 0x00;

    mem[10] = 0xD0;
    mem[11] = 0x82;
    drop(mem);
    cpu.run();
}
