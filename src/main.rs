use cpu::CPU;

mod cpu;
mod instructions;


fn main() {
    // let mut cpu = CPU::new(std::time::Duration::from_nanos(1));
    let mut cpu = CPU::new(std::time::Duration::from_millis(1000));
    let memory_lock = cpu.memory.clone();
    let mut mem = memory_lock.lock().unwrap();
    cpu.registers.x = 0x52;
    mem[0x5000] = 0x10;
    mem[0] = 0xA9;
    mem[1] = 0x50;
    mem[3] = 0xAC;
    mem[4] = 0x00;
    mem[5] = 0x50;
    mem[6] = 0x8E;
    mem[7] = 0x00;
    mem[8] = 0x42;
    mem[10] = 0xD0;
    mem[11] = 0x82;
    drop(mem);
    cpu.run();
}
