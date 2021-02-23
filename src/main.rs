use cpu::CPU;

mod cpu;
mod instructions;


fn main() {
    let mut cpu = CPU::new();
    println!("{:04x}:{:04x}", cpu.get_memory_at_address(0xFF44), cpu.registers.x);
    cpu.execute_instruction(&0xA2, vec![0x44, 0xFF]);
    println!("{:04x}:{:04x}", cpu.get_memory_at_address(0xFF44), cpu.registers.x);
}
