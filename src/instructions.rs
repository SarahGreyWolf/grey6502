use crate::{CPU, cpu::StatRegister};

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

pub trait Instruction {
    fn get_opcodes(&self) -> Vec<u8>;
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU);
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

    instructions
}

instruction!(BRK, vec![0x00],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {
        let interrupt: u8 = u8::from(cpu.registers.sr) & 0b100;
        cpu.registers.sr = StatRegister::from(interrupt);
    }
);
instruction!(BPL, vec![0x10],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {
        cpu.registers.pc = (cpu.registers.pc as i16 + values[0]) as u16;
    }
);
instruction!(JSR, vec![0x20],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {
        
    }
);
instruction!(BMI, vec![0x30],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(RTI, vec![0x40],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(BVC, vec![0x50],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(RTS, vec![0x60],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(BVS, vec![0x70],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(BCC, vec![0x90],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(LDY, vec![0xA0, 0xA4, 0xB4],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(BCS, vec![0xB0],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(CPY, vec![0xC0, 0xC4],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(BNE, vec![0xD0],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(CPX, vec![0xE0, 0xE4],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(BEQ, vec![0xF0],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(ORA, vec![0x01, 0x11, 0x05, 0x15],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(AND, vec![0x21, 0x31, 0x25, 0x35],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(EOR, vec![0x41, 0x51, 0x45, 0x55],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(ADC, vec![0x61, 0x71, 0x65, 0x75],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(STA, vec![0x81, 0x91, 0x85, 0x95],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(LDA, vec![0xA1, 0xB1, 0xA5, 0xB5],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(CMP, vec![0xC1, 0xD1, 0xC5, 0xD5],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(SBC, vec![0xE1, 0xF1, 0xE5, 0xF5],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(LDX, vec![0xA2, 0xA6, 0xB6],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {
        match opcode {
            0xA2 => {
                let address: u16 = values[0] as u16 | (values[1] as u16) << 8;
                cpu.registers.x = cpu.get_memory_at_address(address);
            }
            _ => {}
        }
    }
);
instruction!(BIT, vec![0x24],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(STY, vec![0x84, 0x94],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(ASL, vec![0x06, 0x16],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(ROL, vec![0x26, 0x36],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(LSR, vec![0x46, 0x56],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(ROR, vec![0x66, 0x76],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(STX, vec![0x86, 0x96],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(DEC, vec![0xC6, 0xD6],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);
instruction!(INC, vec![0xE6, 0xF6],
    fn execute(&self, opcode: &u8, values: Vec<i16>, cpu: &mut CPU) {

    }
);