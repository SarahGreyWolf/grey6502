use std::time::Duration;
use std::{fmt::Display, sync::Arc};
use std::sync::Mutex;

use crate::{instructions::{Instruction, init_instructions, Mode}};

#[derive(Clone, Copy)]
pub struct StatRegister {
    pub negative: bool,
    pub overflow: bool,
    pub ignored: bool,
    pub sbreak: bool,
    pub decimal: bool,
    pub interrupt: bool,
    pub zero: bool,
    pub carry: bool
}

impl Display for StatRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            r"
                    StatRegister:
                        negative: {},
                        overflow: {},
                        ignored: {},
                        sbreak: {},
                        decimal: {},
                        interrupt: {},
                        zero: {},
                        carry: {}
            ",
            self.negative,
            self.overflow,
            self.ignored,
            self.sbreak,
            self.decimal,
            self.interrupt,
            self.zero,
            self.carry
        )
    }
}

impl From<u8> for StatRegister {
    fn from(byte: u8) -> Self {
        Self {
            negative: byte & 0x1 == 1,
            overflow: byte & 0x2 == 1,
            ignored: byte & 0x4 == 1,
            sbreak: byte & 0x8 == 1,
            decimal: byte & 0x10 == 1,
            interrupt: byte & 0x20 == 1,
            zero: byte & 0x40 == 1,
            carry: byte & 0x80 == 1,
        }
    }
}

impl From<StatRegister> for u8 {
    fn from(statreg: StatRegister) -> Self {
        (statreg.negative as u8)  << 7  |
        (statreg.overflow as u8)  << 6  |
        (statreg.ignored as u8)   << 5  |
        (statreg.sbreak as u8)    << 4  |
        (statreg.decimal as u8)   << 3  |
        (statreg.interrupt as u8) << 2  |
        (statreg.zero as u8)      << 1  |
        statreg.carry as u8
    }
}

pub struct Registers {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: StatRegister,
    pub sp: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: StatRegister::from(0),
            sp: 0
        }
    }

    pub fn increment_pc(&mut self) -> u16 {
        self.pc = self.pc.wrapping_add(1);
        self.pc
    }
    pub fn increment_pc_by(&mut self, amount: u16) -> u16 {
        self.pc = self.pc.wrapping_add(amount);
        self.pc
    }
    pub fn decrement_pc(&mut self) -> u16 {
        self.pc = self.pc.wrapping_sub(1);
        self.pc
    }
}

pub struct CPU {
    speed: std::time::Duration,
    pub memory: Arc<Mutex<[i16; 0xFFFF]>>,
    // Possibly change this so the stack uses space in memory
    pub stack: [u8; 0xFF],
    pub registers: Registers,
    pub instructions: Arc<Vec<Box<dyn Instruction>>>,
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            r"
            CPU:
                Registers:
                    pc: {:04x},
                    ac: {:04x},
                    x: {:04x},
                    y: {:04x},
                    sp: {:04x},
                        {}
            ",
            self.registers.pc,
            self.registers.ac,
            self.registers.x,
            self.registers.y,
            self.registers.sp,
            self.registers.sr
        )
    }
}

impl CPU {
    pub fn new(speed: std::time::Duration) -> Self {
        let mem: [i16; 0xFFFF] = [0xEA; 0xFFFF];
        Self {
            speed,
            memory: Arc::new(Mutex::new(mem)),
            stack: [0; 0xFF],
            registers: Registers::new(),
            instructions: Arc::new(init_instructions()),
        }
    }

    pub fn run(&mut self) {
        let memory_lock = self.memory.clone();
        let mut time = std::time::Instant::now();
        loop {
            let memory = memory_lock.lock().unwrap();
            if time.elapsed() >= self.speed {
                println!("{}", self);
                let instruct = memory[self.registers.pc as usize];
                drop(memory);
                self.execute_instruction(&instruct);
                time = std::time::Instant::now();
            }
        }
    }

    pub fn push_to_stack(&mut self, value: u8) {
        if self.registers.sp as usize == self.stack.len() {
            self.registers.sp = 0;
        }
        self.stack[self.registers.sp as usize] = value;
        self.registers.sp = self.registers.sp.wrapping_add(1);
    }

    pub fn pull_from_stack(&mut self) -> u8 {
        if self.registers.sp as usize == 0 {
            self.registers.sp = self.stack.len() as u8;
        }
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.stack[self.registers.sp.wrapping_add(1) as usize]
    }

    pub fn set_memory_at_address(&mut self, address: u16, byte: i16) {
        let memory_lock = self.memory.clone();
        let mut memory = memory_lock.lock().expect("Failed to lock memory");
        memory[address as usize] = byte;
        drop(memory);
    }

    pub fn get_memory_at_address(&mut self, address: u16) -> i16 {
        let memory_lock = self.memory.clone();
        let memory = memory_lock.lock().expect("Failed to lock memory");
        let mut address = address;
        if memory.len() == address as usize {
            address = self.registers.increment_pc();
        }
        let out = memory[address as usize];
        drop(memory);
        out
    }
    pub fn get_umemory_at_address(&mut self, address: u16) -> u8 {
        let memory_lock = self.memory.clone();
        let memory = memory_lock.lock().expect("Failed to lock memory");
        let mut address = address;
        if memory.len() == address as usize {
            address = self.registers.increment_pc();
        }
        let out = memory[address as usize] as u8;
        drop(memory);
        out
    }

    pub fn execute_instruction(&mut self, opcode: &i16) {
        let instructions = self.instructions.clone();
        let instruction = match instructions.iter().find(|i| i.get_opcodes().contains(opcode)) {
            Some(i) => i,
            None => {
                panic!("An unknown instruction was called at address: {:04x}:{:04x}", self.registers.sp, opcode);
            }
        };
        if instruction.execute(opcode, self) {
            let address = self.registers.increment_pc();
            println!("{:04x}:{:04x}", address, self.get_umemory_at_address(address));
        }
    }
}
