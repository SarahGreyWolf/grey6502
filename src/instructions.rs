use std::ops::Shl;

use crate::{CPU, cpu::{self, StatRegister}};

// Operates in Little-Endian, lowest byte first then highest byte
pub enum Mode {
    // Operates on the accumulator
    A,
    // Operates on address, EG. $HHLL which becomes LLHH
    Absolute,
    // Operates on address, incremented by X register
    AbsoluteX,
    // Operates on address, incremented by Y register
    AbsoluteY,
    // Operates on byte, EG. LDA #10, loads 10 into the Accumulator
    Immediate,
    // Basically does shit without needing any extra data to be provided
    Implied,
    // Operates on address, address is contents of provided address
    // EG. if location $0120 contains $FC and location $0121 contains $BA
    // the instruction JMP ($0120) will cause the next instruction execution to occur at $BAFC (e.g. the contents of $0120 and $0121).
    Indirect,
    // Same as above but result address is address in provided address + X register
    IndirectX,
    // Same as above but result address is address in provided address + Y register
    IndirectY,
    // Target is program counter (PC) + signed offset
    Relative,
    // Operates on an address that is only 8 bits, so only first 256 bytes of memory
    Zeropage,
    // Operates on an address that is only 8 bits, so only first 256 bytes of memory, incremented by X, note will never access more than #FF so #01FF will ignore the 01
    ZeropageX,
    // Operates on an address that is only 8 bits, so only first 256 bytes of memory, incremented by Y, note will never access more than #FF so #01FF will ignore the 01
    ZeropageY,
}

impl Mode {
    fn get_memory(&self, cpu: &mut cpu::CPU) -> u8 {
        let mut end_address: u16 = 0x0000;
        match self {
            Mode::Immediate => {
                end_address = cpu.registers.increment_pc();
            },
            Mode::Zeropage => {
                let mut address = cpu.registers.increment_pc();
                let f_mem_address = cpu.get_memory_at_address(address);
                address = cpu.registers.increment_pc();
                let s_mem_address = cpu.get_memory_at_address(address);
                end_address = 
                (f_mem_address as u16 | (s_mem_address as u16) << 8) & 0xFF;
            },
            Mode::ZeropageX => {
                let mut address = cpu.registers.increment_pc();
                let f_mem_address = cpu.get_memory_at_address(address);
                address = cpu.registers.increment_pc();
                let s_mem_address = cpu.get_memory_at_address(address);
                end_address = 
                (f_mem_address as u16 | (s_mem_address as u16) << 8)
                    .wrapping_add(cpu.registers.x as u16) & 0xFF;
            },
            Mode::ZeropageY => {
                let mut address = cpu.registers.increment_pc();
                let f_mem_address = cpu.get_memory_at_address(address);
                address = cpu.registers.increment_pc();
                let s_mem_address = cpu.get_memory_at_address(address);
                end_address = 
                (f_mem_address as u16 | (s_mem_address as u16) << 8)
                    .wrapping_add(cpu.registers.y as u16) & 0xFF;
            },
            Mode::Absolute => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                end_address = first_half_memory as u16 | (second_half_memory as u16) << 8;
            },
            Mode::AbsoluteX => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                let x_register = cpu.registers.x;
                end_address = (first_half_memory as u16 | (second_half_memory as u16) << 8)
                    .wrapping_add(x_register as u16);
            },
            Mode::AbsoluteY => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                let y_register = cpu.registers.y;
                end_address = (first_half_memory as u16 | (second_half_memory as u16) << 8)
                    .wrapping_add(y_register as u16);
            },
            Mode::Indirect => {
                let f_og_address = cpu.registers.increment_pc();
                let f_address = cpu.get_memory_at_address(f_og_address);
                let s_og_address = cpu.registers.increment_pc();
                let s_address = cpu.get_memory_at_address(s_og_address);
                end_address = f_address as u16 | (s_address as u16) << 8;
            },
            Mode::IndirectX => {
                let f_og_address = cpu.registers.increment_pc();
                let f_address = cpu.get_memory_at_address(f_og_address);
                let s_og_address = cpu.registers.increment_pc();
                let s_address = cpu.get_memory_at_address(s_og_address);
                end_address = (f_address as u16 | (s_address as u16) << 8)
                    .wrapping_add(cpu.registers.x as u16);
            },
            Mode::IndirectY => {
                let f_og_address = cpu.registers.increment_pc();
                let f_address = cpu.get_memory_at_address(f_og_address);
                let s_og_address = cpu.registers.increment_pc();
                let s_address = cpu.get_memory_at_address(s_og_address);
                end_address = (f_address as u16 | (s_address as u16) << 8)
                    .wrapping_add(cpu.registers.y as u16);
            },
            _ => {}
        }
        cpu.get_memory_at_address(end_address)
    }

    fn set_memory(&self, byte: u8, cpu: &mut cpu::CPU) {
        let mut end_address: u16 = 0x0000;
        match self {
            Mode::Immediate => {
                end_address = cpu.registers.increment_pc();
            },
            Mode::Zeropage => {
                let mut address = cpu.registers.increment_pc();
                let f_mem_address = cpu.get_memory_at_address(address);
                address = cpu.registers.increment_pc();
                let s_mem_address = cpu.get_memory_at_address(address);
                end_address = 
                (f_mem_address as u16 | (s_mem_address as u16) << 8) & 0xFF;
            },
            Mode::ZeropageX => {
                let mut address = cpu.registers.increment_pc();
                let f_mem_address = cpu.get_memory_at_address(address);
                address = cpu.registers.increment_pc();
                let s_mem_address = cpu.get_memory_at_address(address);
                end_address = 
                (f_mem_address as u16 | (s_mem_address as u16) << 8)
                    .wrapping_add(cpu.registers.x as u16) & 0xFF;
            },
            Mode::ZeropageY => {
                let mut address = cpu.registers.increment_pc();
                let f_mem_address = cpu.get_memory_at_address(address);
                address = cpu.registers.increment_pc();
                let s_mem_address = cpu.get_memory_at_address(address);
                end_address = 
                (f_mem_address as u16 | (s_mem_address as u16) << 8)
                    .wrapping_add(cpu.registers.y as u16) & 0xFF;
            },
            Mode::Absolute => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                end_address = first_half_memory as u16 | (second_half_memory as u16) << 8;
            },
            Mode::AbsoluteX => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                let x_register = cpu.registers.x;
                end_address = (first_half_memory as u16 | (second_half_memory as u16) << 8)
                .wrapping_add(x_register as u16);
            },
            Mode::AbsoluteY => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                let y_register = cpu.registers.y;
                end_address = (first_half_memory as u16 | (second_half_memory as u16) << 8)
                    .wrapping_add(y_register as u16);
            },
            Mode::Indirect => {
                let f_og_address = cpu.registers.increment_pc();
                let f_address = cpu.get_memory_at_address(f_og_address);
                let s_og_address = cpu.registers.increment_pc();
                let s_address = cpu.get_memory_at_address(s_og_address);
                end_address = f_address as u16 | (s_address as u16) << 8;
            },
            Mode::IndirectX => {
                let f_og_address = cpu.registers.increment_pc();
                let f_address = cpu.get_memory_at_address(f_og_address);
                let s_og_address = cpu.registers.increment_pc();
                let s_address = cpu.get_memory_at_address(s_og_address);
                end_address = (f_address as u16 | (s_address as u16) << 8)
                    .wrapping_add(cpu.registers.x as u16);
            },
            Mode::IndirectY => {
                let f_og_address = cpu.registers.increment_pc();
                let f_address = cpu.get_memory_at_address(f_og_address);
                let s_og_address = cpu.registers.increment_pc();
                let s_address = cpu.get_memory_at_address(s_og_address);
                end_address = (f_address as u16 | (s_address as u16) << 8)
                    .wrapping_add(cpu.registers.y as u16);
            },
            _ => {}
        }
        cpu.registers.sr.negative = byte & 0x80 == 0x80;
        cpu.registers.sr.zero = byte == 0;
        cpu.set_memory_at_address(end_address, byte);
    }
}

pub trait Instruction {
    fn get_opcodes(&self) -> Vec<u8>;
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool;
}

#[macro_export]
macro_rules! instruction {
    ( $name:ident, $opcodes:expr, $execute:item) => {
        #[allow(dead_code)]
        pub struct $name {
            opcodes: Vec<u8>,
        }

        impl Instruction for $name {
            fn get_opcodes(&self) -> Vec<u8> {
                self.opcodes.clone()
            }

            $execute
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new() -> Self {
                Self { opcodes: $opcodes }
            }

            #[allow(dead_code)]
            pub fn get_opcode(&self, index: usize) -> u8 {
                self.opcodes[index]
            }
        }
    };
}


pub fn init_instructions() -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
    instructions.push(Box::new(BRK::new()));
    instructions.push(Box::new(BPL::new()));
    instructions.push(Box::new(JSR::new()));
    instructions.push(Box::new(BMI::new()));
    instructions.push(Box::new(RTI::new()));
    instructions.push(Box::new(BVC::new()));
    instructions.push(Box::new(RTS::new()));
    instructions.push(Box::new(BVS::new()));
    instructions.push(Box::new(BCC::new()));
    instructions.push(Box::new(LDY::new()));
    instructions.push(Box::new(BCS::new()));
    instructions.push(Box::new(CPY::new()));
    instructions.push(Box::new(BNE::new()));
    instructions.push(Box::new(CPX::new()));
    instructions.push(Box::new(BEQ::new()));
    instructions.push(Box::new(ORA::new()));
    instructions.push(Box::new(AND::new()));
    instructions.push(Box::new(EOR::new()));
    instructions.push(Box::new(ADC::new()));
    instructions.push(Box::new(STA::new()));
    instructions.push(Box::new(LDA::new()));
    instructions.push(Box::new(CMP::new()));
    instructions.push(Box::new(SBC::new()));
    instructions.push(Box::new(LDX::new()));
    instructions.push(Box::new(BIT::new()));
    instructions.push(Box::new(STY::new()));
    instructions.push(Box::new(ASL::new()));
    instructions.push(Box::new(ROL::new()));
    instructions.push(Box::new(LSR::new()));
    instructions.push(Box::new(ROR::new()));
    instructions.push(Box::new(STX::new()));
    instructions.push(Box::new(DEC::new()));
    instructions.push(Box::new(INC::new()));
    instructions.push(Box::new(NOP::new()));
    instructions.push(Box::new(PHP::new()));
    instructions.push(Box::new(CLC::new()));
    instructions.push(Box::new(PLP::new()));
    instructions.push(Box::new(SEC::new()));
    instructions.push(Box::new(PHA::new()));
    instructions.push(Box::new(CLI::new()));
    instructions.push(Box::new(PLA::new()));
    instructions.push(Box::new(SEI::new()));
    instructions.push(Box::new(DEY::new()));
    instructions.push(Box::new(TYA::new()));
    instructions.push(Box::new(TAY::new()));
    instructions.push(Box::new(CLV::new()));
    instructions.push(Box::new(INY::new()));
    instructions.push(Box::new(CLD::new()));
    instructions.push(Box::new(INX::new()));
    instructions.push(Box::new(SED::new()));
    instructions.push(Box::new(TXA::new()));
    instructions.push(Box::new(TXS::new()));
    instructions.push(Box::new(TAX::new()));
    instructions.push(Box::new(TSX::new()));
    instructions.push(Box::new(DEX::new()));

    instructions
}

instruction!(BRK, vec![0x00],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let current_address = cpu.registers.increment_pc_by(2);
        cpu.push_to_stack((current_address >> 8) as u8);
        cpu.push_to_stack((current_address) as u8);
        cpu.push_to_stack(u8::from(cpu.registers.sr));
        let interrupt: u8 = u8::from(cpu.registers.sr) & 0b100;
        cpu.registers.sr = StatRegister::from(interrupt);
        let address_first = cpu.registers.increment_pc();
        let address_second = cpu.registers.increment_pc();
        cpu.registers.pc = cpu.get_memory_at_address(address_first) as u16 | (cpu.get_memory_at_address(address_second) as u16) << 8;
        false
    }
);
instruction!(BPL, vec![0x10],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if !cpu.registers.sr.negative {
            cpu.registers.pc = (cpu.registers.pc).wrapping_add(cpu.get_memory_at_address(address) as u16) as u16;
            false
        } else {
            cpu.registers.increment_pc();
            true
        }
    }
);
instruction!(JSR, vec![0x20],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.push_to_stack((cpu.registers.pc + 2 >> 8) as u8);
        cpu.push_to_stack((cpu.registers.pc + 2) as u8);
        cpu.registers.pc = (cpu.registers.pc as i16 + cpu.get_memory_at_address(cpu.registers.pc + 1) as i16) as u16;
        false
    }
);
instruction!(BMI, vec![0x30],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if cpu.registers.sr.negative {
            let memory = cpu.get_memory_at_address(address);
            if memory & 0x80 == 0x80 {
                cpu.registers.pc = (cpu.registers.pc).wrapping_sub((memory as u16) & 0x7F);
            } else {
                cpu.registers.pc = (cpu.registers.pc).wrapping_add(memory as u16);
            }
            false
        } else {
            cpu.registers.increment_pc();
            true
        }
    }
);
instruction!(RTI, vec![0x40],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr = StatRegister::from(cpu.pull_from_stack());
        cpu.registers.pc = cpu.pull_from_stack() as u16 | ((cpu.pull_from_stack() as u16) << 8);
        false
    }
);
instruction!(BVC, vec![0x50],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if !cpu.registers.sr.overflow {
            let memory = cpu.get_memory_at_address(address);
            if memory & 0x80 == 0x80 {
                cpu.registers.pc = (cpu.registers.pc).wrapping_sub((memory as u16) & 0x7F);
            } else {
                cpu.registers.pc = (cpu.registers.pc).wrapping_add(memory as u16);
            }
            false
        } else {
            cpu.registers.increment_pc();
            true
        }
    }
);
instruction!(RTS, vec![0x60],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.pc = cpu.pull_from_stack() as u16 | ((cpu.pull_from_stack() as u16) << 8);
        false
    }
);
instruction!(BVS, vec![0x70],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if cpu.registers.sr.overflow {
            let memory = cpu.get_memory_at_address(address);
            if memory & 0x80 == 0x80 {
                cpu.registers.pc = (cpu.registers.pc).wrapping_sub((memory as u16) & 0x7F);
            } else {
                cpu.registers.pc = (cpu.registers.pc).wrapping_add(memory as u16);
            }
            false
        } else {
            cpu.registers.increment_pc();
            true
        }
    }
);
instruction!(BCC, vec![0x90],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if !cpu.registers.sr.carry {
            let memory = cpu.get_memory_at_address(address);
            if memory & 0x80 == 0x80 {
                cpu.registers.pc = (cpu.registers.pc).wrapping_sub((memory as u16) & 0x7F);
            } else {
                cpu.registers.pc = (cpu.registers.pc).wrapping_add(memory as u16);
            }
            false
        } else {
            cpu.registers.increment_pc();
            true
        }
    }
);
instruction!(LDY, vec![0xA0, 0xA4, 0xB4, 0xAC, 0xBC],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory: u8 = 0x00;
        match opcode {
            0xA0 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address);
            },
            0xA4 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0xB4 => {
                let address = cpu.registers.increment_pc();
                let x_register = cpu.registers.x;
                memory = cpu.get_memory_at_address(address.wrapping_add(x_register as u16) & 0xFF);
            },
            0xAC => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                memory = cpu.get_memory_at_address(first_half_memory as u16 | (second_half_memory as u16) << 8);
            },
            0xBC => {
                let first_half_address = cpu.registers.increment_pc();
                let first_half_memory = cpu.get_memory_at_address(first_half_address);
                let second_half_address = cpu.registers.increment_pc();
                let second_half_memory = cpu.get_memory_at_address(second_half_address);
                let x_register = cpu.registers.x;
                memory = cpu.get_memory_at_address(
                    (first_half_memory as u16 | (second_half_memory as u16) << 8)
                    .wrapping_add(x_register as u16));
            },
            _ => {}
        }
        cpu.registers.y = memory;
        cpu.registers.sr.negative = cpu.registers.y & 0x80 == 1;
        cpu.registers.sr.zero = cpu.registers.y == 0;
        true
    }
);
instruction!(BCS, vec![0xB0],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if cpu.registers.sr.carry {
            let memory = cpu.get_memory_at_address(address);
            if memory & 0x80 == 0x80 {
                cpu.registers.pc = (cpu.registers.pc).wrapping_sub((memory as u16) & 0x7F);
            } else {
                cpu.registers.pc = (cpu.registers.pc).wrapping_add(memory as u16);
            }
            false
        } else {
            cpu.registers.increment_pc();
            true
        }
    }
);
instruction!(CPY, vec![0xC0, 0xC4, 0xCC],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory: u8 = 0x00;
        let address = cpu.registers.increment_pc();
        match opcode {
            0xC0 => {
                memory = cpu.get_memory_at_address(address);
            },
            0xC4 => {
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0xCC => {
                let address_first = cpu.registers.increment_pc();
                let mem_first = cpu.get_memory_at_address(address_first);
                let address_second = cpu.registers.increment_pc();
                let mem_second = cpu.get_memory_at_address(address_second);
                memory = cpu.get_memory_at_address(mem_first as u16 | (mem_second as u16) << 8)
            },
            _ => {}
        }
        let (result, overflowed) = (cpu.registers.y).overflowing_sub(memory as u8);
        cpu.registers.sr.negative = (result & 0x80) == 0x80;
        cpu.registers.sr.zero = result == 0;
        cpu.registers.sr.carry = overflowed;
        true
    }
);
instruction!(BNE, vec![0xD0],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if !cpu.registers.sr.zero {
            let memory = cpu.get_memory_at_address(address);
            if memory & 0x80 == 0x80 {
                cpu.registers.pc = (cpu.registers.pc).wrapping_sub((memory as u16) & 0x7F);
            } else {
                cpu.registers.pc = (cpu.registers.pc).wrapping_add(memory as u16);
            }
            false
        } else {
            true
        }
    }
);
instruction!(CPX, vec![0xE0, 0xE4],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory: u8 = 0x00;
        let address = cpu.registers.increment_pc();
        match opcode {
            0xC0 => {
                memory = cpu.get_memory_at_address(address);
            },
            0xC4 => {
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0xCC => {
                let address_first = cpu.registers.increment_pc();
                let mem_first = cpu.get_memory_at_address(address_first);
                let address_second = cpu.registers.increment_pc();
                let mem_second = cpu.get_memory_at_address(address_second);
                memory = cpu.get_memory_at_address(mem_first as u16 | (mem_second as u16) << 8)
            },
            _ => {}
        }
        let (result, overflowed) = (cpu.registers.x).overflowing_sub(memory as u8);
        cpu.registers.sr.negative = (result & 0x80) == 0x80;
        cpu.registers.sr.zero = result == 0;
        cpu.registers.sr.carry = overflowed;
        true
    }
);
instruction!(BEQ, vec![0xF0],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        let address = cpu.registers.increment_pc();
        if cpu.registers.sr.zero {
            let memory = cpu.get_memory_at_address(address);
            if memory & 0x80 == 0x80 {
                cpu.registers.pc = (cpu.registers.pc).wrapping_sub((memory as u16) & 0x7F);
            } else {
                cpu.registers.pc = (cpu.registers.pc).wrapping_add(memory as u16);
            }
            false
        } else {
            true
        }
    }
);
instruction!(ORA, vec![0x09, 0x05, 0x15, 0x0D, 0x1D, 0x19, 0x01, 0x11],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0x09 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address);
            },
            0x05 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0x15 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(
                    (address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
            },
            0x0D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(first_mem as u16 | ((second_mem as u16) << 8) as u16);
            },
            0x1D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.x as u16));
            },
            0x19 => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.y as u16));
            },
            0x01 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            0x11 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.y as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            _ => {}
        }
        cpu.registers.ac = cpu.registers.ac | memory;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 1;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        true
    }
);
instruction!(AND, vec![0x29, 0x25, 0x35, 0x2D, 0x3D, 0x39, 0x21, 0x31],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0x29 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address);
            },
            0x25 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0x35 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(
                    (address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
            },
            0x2D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(first_mem as u16 | ((second_mem as u16) << 8) as u16);
            },
            0x3D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.x as u16));
            },
            0x39 => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.y as u16));
            },
            0x21 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            0x32 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.y as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            _ => {}
        }
        cpu.registers.ac = cpu.registers.ac & memory;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 1;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        true
    }
);
instruction!(EOR, vec![0x49, 0x45, 0x55, 0x4D, 0x5D, 0x59, 0x41, 0x51],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0x49 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address);
            },
            0x45 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0x55 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(
                    (address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
            },
            0x4D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(first_mem as u16 | ((second_mem as u16) << 8) as u16);
            },
            0x5D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.x as u16));
            },
            0x59 => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.y as u16));
            },
            0x41 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            0x51 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.y as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            _ => {}
        }
        cpu.registers.ac = cpu.registers.ac ^ memory;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 1;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        true
    }
);
instruction!(ADC, vec![0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0x69 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address);
            },
            0x65 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0x75 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(
                    (address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
            },
            0x6D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(first_mem as u16 | ((second_mem as u16) << 8) as u16);
            },
            0x7D => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.x as u16));
            },
            0x79 => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.y as u16));
            },
            0x61 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            0x71 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.y as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            _ => {}
        }
        let (res, overflowed) = cpu.registers.ac.overflowing_add(memory);
        let (sres, soverflowed) = res.overflowing_add(cpu.registers.sr.carry as u8);
        cpu.registers.ac = sres;
        cpu.registers.sr.carry = overflowed & soverflowed;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 1;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        cpu.registers.sr.overflow = overflowed & soverflowed;
        true
    }
);
instruction!(STA, vec![0x85, 0x95, 0x8D, 0x9D, 0x99, 0x81, 0x91],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        match opcode {
            0x85 => {
                let address = cpu.registers.increment_pc();
                cpu.set_memory_at_address(address & 0xFF, cpu.registers.ac);
            },
            0x95 => {
                let address = cpu.registers.increment_pc();
                cpu.set_memory_at_address((address & 0xFF)
                    .wrapping_add(cpu.registers.x as u16), cpu.registers.ac);
            },
            0x8D => {
                let fhalf_address = cpu.registers.increment_pc();
                let fhalf_memory = cpu.get_memory_at_address(fhalf_address);
                let shalf_address = cpu.registers.increment_pc();
                let shalf_memory = cpu.get_memory_at_address(shalf_address);
                let address = (fhalf_memory as u16) & ((shalf_memory as u16) << 8);
                cpu.set_memory_at_address(address, cpu.registers.ac);
            },
            0x9D => {
                let fhalf_address = cpu.registers.increment_pc();
                let fhalf_memory = cpu.get_memory_at_address(fhalf_address);
                let shalf_address = cpu.registers.increment_pc();
                let shalf_memory = cpu.get_memory_at_address(shalf_address);
                let address = (fhalf_memory as u16) & ((shalf_memory as u16) << 8);
                cpu.set_memory_at_address(address.wrapping_add(cpu.registers.x as u16),
                    cpu.registers.ac);
            },
            0x99 => {
                let fhalf_address = cpu.registers.increment_pc();
                let fhalf_memory = cpu.get_memory_at_address(fhalf_address);
                let shalf_address = cpu.registers.increment_pc();
                let shalf_memory = cpu.get_memory_at_address(shalf_address);
                let address = (fhalf_memory as u16) & ((shalf_memory as u16) << 8);
                cpu.set_memory_at_address(address.wrapping_add(cpu.registers.y as u16),
                    cpu.registers.ac);
            },
            0x81 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    og_address.wrapping_add(cpu.registers.x as u16));
                cpu.set_memory_at_address(address as u16, cpu.registers.ac);
            },
            0x91 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    og_address.wrapping_add(cpu.registers.y as u16));
                cpu.set_memory_at_address(address as u16, cpu.registers.ac);
            }
            _ => {}
        }
        true
    }
);
instruction!(LDA, vec![0xA9, 0xA5, 0xB5, 0xAD, 0xBD, 0xB9, 0xA1, 0xB1],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00 as u8;
        match opcode {
            0xA9 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address);
            },
            0xA5 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0xB5 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address((address & 0xFF)
                    .wrapping_add(cpu.registers.x as u16));
            },
            0xAD => {
                let fhalf_address = cpu.registers.increment_pc();
                let fhalf_memory = cpu.get_memory_at_address(fhalf_address);
                let shalf_address = cpu.registers.increment_pc();
                let shalf_memory = cpu.get_memory_at_address(shalf_address);
                let address = (fhalf_memory as u16) & ((shalf_memory as u16) << 8);
                memory = cpu.get_memory_at_address(address);
            },
            0xBD => {
                let fhalf_address = cpu.registers.increment_pc();
                let fhalf_memory = cpu.get_memory_at_address(fhalf_address);
                let shalf_address = cpu.registers.increment_pc();
                let shalf_memory = cpu.get_memory_at_address(shalf_address);
                let address = (fhalf_memory as u16) & ((shalf_memory as u16) << 8);
                memory = cpu.get_memory_at_address(address.
                        wrapping_add(cpu.registers.x as u16));
            },
            0xB9 => {
                let fhalf_address = cpu.registers.increment_pc();
                let fhalf_memory = cpu.get_memory_at_address(fhalf_address);
                let shalf_address = cpu.registers.increment_pc();
                let shalf_memory = cpu.get_memory_at_address(shalf_address);
                let address = (fhalf_memory as u16) & ((shalf_memory as u16) << 8);
                memory = cpu.get_memory_at_address(address.
                    wrapping_add(cpu.registers.y as u16));
            },
            0xA1 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    og_address.wrapping_add(cpu.registers.x as u16));
                memory = cpu.get_memory_at_address(address as u16);
            },
            0xB1 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    og_address.wrapping_add(cpu.registers.y as u16));
                    memory = cpu.get_memory_at_address(address as u16);
            }
            _ => {}
        }
        cpu.registers.ac = memory;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        true
    }
);
instruction!(CMP, vec![0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory: u8 = 0x00;
        let address = cpu.registers.increment_pc();
        match opcode {
            0xC9 => {
                memory = cpu.get_memory_at_address(address);
            },
            0xC5 => {
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0xD5 => {
                memory = cpu.get_memory_at_address((
                    address & 0xFF).wrapping_add(cpu.registers.x as u16));
            },
            0xCD => {
                let address_first = cpu.registers.increment_pc();
                let mem_first = cpu.get_memory_at_address(address_first);
                let address_second = cpu.registers.increment_pc();
                let mem_second = cpu.get_memory_at_address(address_second);
                memory = cpu.get_memory_at_address(
                    mem_first as u16 | (mem_second as u16) << 8);
            },
            0xDD => {
                let address_first = cpu.registers.increment_pc();
                let mem_first = cpu.get_memory_at_address(address_first);
                let address_second = cpu.registers.increment_pc();
                let mem_second = cpu.get_memory_at_address(address_second);
                memory = cpu.get_memory_at_address(
                    (mem_first as u16 | (mem_second as u16) << 8)
                        .wrapping_add(cpu.registers.x as u16));
            },
            0xD9 => {
                let address_first = cpu.registers.increment_pc();
                let mem_first = cpu.get_memory_at_address(address_first);
                let address_second = cpu.registers.increment_pc();
                let mem_second = cpu.get_memory_at_address(address_second);
                memory = cpu.get_memory_at_address(
                    (mem_first as u16 | (mem_second as u16) << 8)
                        .wrapping_add(cpu.registers.y as u16));
            },
            0xC1 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    og_address.wrapping_add(cpu.registers.x as u16));
                memory = cpu.get_memory_at_address(address as u16);
            },
            0xD1 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    og_address.wrapping_add(cpu.registers.y as u16));
                memory = cpu.get_memory_at_address(address as u16);
            },
            _ => {}
        }
        let (result, overflowed) = (cpu.registers.ac).overflowing_sub(memory as u8);
        cpu.registers.sr.negative = (result & 0x80) == 0x80;
        cpu.registers.sr.zero = result == 0;
        cpu.registers.sr.carry = overflowed;
        true
    }
);
instruction!(SBC, vec![0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1, 0xF1],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0xE9 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address);
            },
            0xE5 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0xF5 => {
                let address = cpu.registers.increment_pc();
                memory = cpu.get_memory_at_address(
                    (address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
            },
            0xED => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(first_mem as u16 | ((second_mem as u16) << 8) as u16);
            },
            0xFD => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.x as u16));
            },
            0xF9 => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                memory = cpu.get_memory_at_address(
                    (first_mem as u16 | ((second_mem as u16) << 8) as u16)
                    .wrapping_add(cpu.registers.y as u16));
            },
            0xE1 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.x as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            0xF1 => {
                let og_address = cpu.registers.increment_pc();
                let address = cpu.get_memory_at_address(
                    (og_address.wrapping_add(cpu.registers.y as u16)) & 0xFF);
                memory = cpu.get_memory_at_address(address as u16);
            },
            _ => {}
        }
        let (res, overflowed) = cpu.registers.ac.overflowing_sub(memory);
        let (sres, soverflowed) = res.overflowing_sub(cpu.registers.sr.carry as u8);
        cpu.registers.ac = sres;
        cpu.registers.sr.carry = overflowed & soverflowed;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 1;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        cpu.registers.sr.overflow = overflowed & soverflowed;
        true
    }
);
instruction!(LDX, vec![0xA2, 0xA6, 0xB6, 0xAE, 0xBE],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        match opcode {
            0xA2 => {
                let address = cpu.registers.increment_pc();
                cpu.registers.x = cpu.get_memory_at_address(address);
            },
            0xA6 => {
                let address = cpu.registers.increment_pc();
                cpu.registers.x = cpu.get_memory_at_address(address & 0xFF);
            },
            0xB6 => {
                let address = cpu.registers.increment_pc();
                cpu.registers.x = cpu.get_memory_at_address(
                    (address & 0xFF).wrapping_add(cpu.registers.x as u16));
            },
            0xAE => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                cpu.registers.x = cpu.get_memory_at_address(
                    (first_mem as u16) & (second_mem as u16) << 8
                )
            },
            0xBE => {
                let first_address = cpu.registers.increment_pc();
                let first_mem = cpu.get_memory_at_address(first_address);
                let second_address = cpu.registers.increment_pc();
                let second_mem = cpu.get_memory_at_address(second_address);
                cpu.registers.x = cpu.get_memory_at_address(
                    ((first_mem as u16) & (second_mem as u16) << 8)
                        .wrapping_add(cpu.registers.y as u16)
                )
            },
            _ => {}
        }
        cpu.registers.sr.negative = cpu.registers.x & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.x == 0;
        true
    }
);
instruction!(BIT, vec![0x24, 0x2C],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        let address = cpu.registers.increment_pc();
        match opcode {
            0x24 => {
                memory = cpu.get_memory_at_address(address & 0xFF);
            },
            0x2C => {
                memory = cpu.get_memory_at_address(address);
            },
            _ => {}
        }
        cpu.registers.sr.negative = memory & 0x40 == 0x40;
        cpu.registers.sr.overflow = memory & 0x20 == 0x20;
        cpu.registers.sr.zero = cpu.registers.ac & memory == 0;
        true
    }
);
instruction!(STY, vec![0x84, 0x94, 0x8C],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        match opcode {
            0x84 => {
                let address = cpu.registers.increment_pc();
                cpu.set_memory_at_address(address & 0xFF, cpu.registers.y);
            },
            0x94 => {
                let address = cpu.registers.increment_pc();
                cpu.set_memory_at_address((address.wrapping_add(cpu.registers.x as u16))
                    & 0xFF, cpu.registers.y);
            },
            0x8C => {
                let address = cpu.registers.increment_pc();
                cpu.set_memory_at_address(address, cpu.registers.y);
            },
            _ => {}
        }
        true
    }
);
instruction!(ASL, vec![0x0A, 0x06, 0x16, 0x0E, 0x1E],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0x0A => {
                let (res, carry) = cpu.registers.ac.overflowing_shl(1);
                cpu.registers.ac = res;
                cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
                cpu.registers.sr.zero = cpu.registers.ac == 0;
                cpu.registers.sr.carry = carry;
            },
            0x06 => {
                memory = Mode::Zeropage.get_memory(cpu);
            },
            0x16 => {
                memory = Mode::ZeropageX.get_memory(cpu);
            },
            0x0E => {
                memory = Mode::Absolute.get_memory(cpu);
            },
            0x1E => {
                memory = Mode::AbsoluteX.get_memory(cpu);
            }
            _ => {}
        }
        let (res, carry) = memory.overflowing_shl(1);
        let address = cpu.registers.pc - 1;
        cpu.set_memory_at_address(address, res);
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        cpu.registers.sr.carry = carry;
        true
    }
);
instruction!(ROL, vec![0x2A, 0x26, 0x36, 0x2E, 0x3E],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0x2A => {
                let result = cpu.registers.ac.rotate_left(1);
                cpu.registers.ac = result as u8;
                cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
                cpu.registers.sr.zero = cpu.registers.ac == 0;
                cpu.registers.sr.carry = false;
            },
            0x26 => {
                memory = Mode::Zeropage.get_memory(cpu);
            },
            0x36 => {
                memory = Mode::ZeropageX.get_memory(cpu);
            },
            0x2E => {
                memory = Mode::Absolute.get_memory(cpu);
            },
            0x3E => {
                memory = Mode::AbsoluteX.get_memory(cpu);
            }
            _ => {}
        }
        let result = memory.rotate_left(1);
        let address = cpu.registers.pc - 1;
        cpu.set_memory_at_address(address, result as u8);
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        cpu.registers.sr.carry = false;
        true
    }
);
instruction!(LSR, vec![0x4A, 0x46, 0x56, 0x4E, 0x5E],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory: u8 = 0x00;
        match opcode {
            0x4A => {
                let (res, carry) = cpu.registers.ac.overflowing_shr(1);
                cpu.registers.ac = res;
                cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
                cpu.registers.sr.zero = cpu.registers.ac == 0;
                cpu.registers.sr.carry = carry;
            },
            0x46 => {
                memory = Mode::Zeropage.get_memory(cpu);
            },
            0x56 => {
                memory = Mode::ZeropageX.get_memory(cpu);
            },
            0x4E => {
                memory = Mode::Absolute.get_memory(cpu);
            },
            0x5E => {
                memory = Mode::AbsoluteX.get_memory(cpu);
            },
            _ => {}
        }
        let (res, carry) = memory.overflowing_shr(1);
        let address = cpu.registers.pc - 1;
        cpu.set_memory_at_address(address, res);
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        cpu.registers.sr.carry = carry;
        true
    }
);
instruction!(ROR, vec![0x6A, 0x66, 0x76, 0x6E, 0x7E],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        let mut memory = 0x00;
        match opcode {
            0x6A => {
                let result = cpu.registers.ac.rotate_right(1);
                cpu.registers.ac = result as u8;
                cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
                cpu.registers.sr.zero = cpu.registers.ac == 0;
                cpu.registers.sr.carry = false;
            },
            0x66 => {
                memory = Mode::Zeropage.get_memory(cpu);
            },
            0x76 => {
                memory = Mode::ZeropageX.get_memory(cpu);
            },
            0x6E => {
                memory = Mode::Absolute.get_memory(cpu);
            },
            0x7E => {
                memory = Mode::AbsoluteX.get_memory(cpu);
            }
            _ => {}
        }
        let result = memory.rotate_right(1);
        let address = cpu.registers.pc - 1;
        cpu.set_memory_at_address(address, result);
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        cpu.registers.sr.carry = false;
        true
    }
);
instruction!(STX, vec![0x86, 0x96, 0x8E],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        match opcode {
            0x86 => {
                Mode::Zeropage.set_memory(cpu.registers.x, cpu);
            },
            0x96 => {
                Mode::ZeropageY.set_memory(cpu.registers.x, cpu);
            },
            0x8E => {
                Mode::Absolute.set_memory(cpu.registers.x, cpu);
            },
            _ => {}
        }
        println!("{:04x}", cpu.get_memory_at_address(0x4200));
        true
    }
);
instruction!(DEC, vec![0xC6, 0xD6, 0xCE, 0xDE],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        match opcode {
            0xC6 => {
                let memory = Mode::Zeropage.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::Zeropage.set_memory(memory.wrapping_sub(1), cpu);
            },
            0xD6 => {
                let memory = Mode::ZeropageX.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::ZeropageX.set_memory(memory.wrapping_sub(1), cpu);
            },
            0xCE => {
                let memory = Mode::Absolute.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::AbsoluteX.set_memory(memory.wrapping_sub(1), cpu);
            },
            0xDE => {
                let memory = Mode::Absolute.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::AbsoluteX.set_memory(memory.wrapping_sub(1), cpu);
            },
            _ => {}
        }
        true
    }
);
instruction!(INC, vec![0xE6, 0xF6, 0xEE, 0xFE],
    fn execute(&self, opcode: &u8, cpu: &mut CPU) -> bool {
        match opcode {
            0xE6 => {
                let memory = Mode::Zeropage.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::Zeropage.set_memory(memory.wrapping_add(1), cpu);
            },
            0xF6 => {
                let memory = Mode::ZeropageX.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::ZeropageX.set_memory(memory.wrapping_add(1), cpu);
            },
            0xEE => {
                let memory = Mode::Absolute.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::AbsoluteX.set_memory(memory.wrapping_add(1), cpu);
            },
            0xFE => {
                let memory = Mode::Absolute.get_memory(cpu);
                cpu.registers.decrement_pc();
                cpu.registers.decrement_pc();
                Mode::AbsoluteX.set_memory(memory.wrapping_add(1), cpu);
            },
            _ => {}
        }
        true
    }
);
instruction!(PHP, vec![0x08],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.push_to_stack(u8::from(cpu.registers.sr));
        true
    }
);
instruction!(CLC, vec![0x18],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr.carry = false;
        true
    }
);
instruction!(PLP, vec![0x28],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr = StatRegister::from(cpu.pull_from_stack());
        true
    }
);
instruction!(SEC, vec![0x38],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr.carry = true;
        true
    }
);
instruction!(PHA, vec![0x48],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.push_to_stack(cpu.registers.ac);
        true
    }
);
instruction!(CLI, vec![0x58],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr.interrupt = false;
        true
    }
);
instruction!(PLA, vec![0x68],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.ac = cpu.pull_from_stack();
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        true
    }
);
instruction!(SEI, vec![0x78],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr.interrupt = true;
        true
    }
);
instruction!(DEY, vec![0x88],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.y = cpu.registers.y.wrapping_sub(1);
        cpu.registers.sr.negative = cpu.registers.y & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.y == 0;
        true
    }
);
instruction!(TYA, vec![0x98],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.ac = cpu.registers.y;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        true
    }
);
instruction!(TAY, vec![0xA8],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.y = cpu.registers.ac;
        cpu.registers.sr.negative = cpu.registers.y & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.y == 0;
        true
    }
);
instruction!(CLV, vec![0xB8],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr.overflow = true;
        true
    }
);
instruction!(INY, vec![0xC8],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.y = cpu.registers.y.wrapping_add(1);
        cpu.registers.sr.negative = cpu.registers.y & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.y == 0;
        true
    }
);
instruction!(CLD, vec![0xD8],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr.decimal = false;
        true
    }
);
instruction!(INX, vec![0xE8],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.x = cpu.registers.x.wrapping_add(1);
        cpu.registers.sr.negative = cpu.registers.x & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.x == 0;
        true
    }
);
instruction!(SED, vec![0xF8],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sr.decimal = true;
        true
    }
);
instruction!(TXA, vec![0x8A],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.ac = cpu.registers.x;
        cpu.registers.sr.negative = cpu.registers.ac & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.ac == 0;
        true
    }
);
instruction!(TXS, vec![0x9A],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.sp = cpu.registers.x;
        cpu.registers.sr.negative = cpu.registers.sp & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.sp == 0;
        true
    }
);
instruction!(TAX, vec![0xAA],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.x = cpu.registers.ac;
        cpu.registers.sr.negative = cpu.registers.x & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.x == 0;
        true
    }
);
instruction!(TSX, vec![0xBA],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.x = cpu.registers.sp;
        cpu.registers.sr.negative = cpu.registers.x & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.x == 0;
        true
    }
);
instruction!(DEX, vec![0xCA],
    fn execute(&self, _opcode: &u8, cpu: &mut CPU) -> bool {
        cpu.registers.x = cpu.registers.x.wrapping_sub(1);
        cpu.registers.sr.negative = cpu.registers.x & 0x80 == 0x80;
        cpu.registers.sr.zero = cpu.registers.x == 0;
        true
    }
);

instruction!(NOP, vec![0xEA],
    fn execute(&self, _opcode: &u8, _cpu: &mut CPU) -> bool {
        true
    }
);