use cpu::CPU;

mod cpu;
mod instructions;


fn main() {
    let mut cpu = CPU::new();
    cpu.run();
}
