use std::ops::{Index, IndexMut};

use crate::{errors::CpuError, instructions::INSTRUCTION_SET, memory::MemoryMap, DecodeContext, Mnemonic};

/// Represents the AF, BC, DE, HL CPU registers of the Game Boy
/// `SP` and `PC` are fields of the `Cpu` struct
pub type Registers = [u16; 4];

impl Index<Register16> for Registers {
    type Output = u16;
    fn index(&self, index: Register16) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Register16> for Registers {
    fn index_mut(&mut self, index: Register16) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Debug, Copy, Clone)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

#[derive(Debug)]
pub struct Cpu {
    pub stack: Vec<u16>,
    instruction_stream: Vec<u8>,
    pub registers: Registers,
    pub flags: Flags,
    // Interrupt master enable flag
    pub ime: bool,
    pub stack_pointer: u16,
    pub program_counter: usize,
}

impl Cpu {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            stack: vec![],
            instruction_stream: rom,
            registers: [0u16; 4],
            flags: Flags::default(),
            ime: false,
            stack_pointer: 0xfffe,
            program_counter: 0x0100,
        }
    }
    /// https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
    pub fn inititalize_registers(&mut self) {
        self.registers[Register16::AF] = 0x1b0;
        self.registers[Register16::BC] = 0x0013;
        self.registers[Register16::DE] = 0x00d8;
        self.registers[Register16::HL] = 0x014d;
    }

    pub fn set_r16(&mut self, register: Register16, n16: u16) {
        self.registers[register] = n16;
    }

    pub fn set_r8(&mut self, register: Register8, value: u8) {
        let value = value as u16;
        match register {
            Register8::A => self.set_r16(
                Register16::AF,
                (self.registers[Register16::AF] & 0x00ff) | (value << 8),
            ),
            Register8::B => self.set_r16(
                Register16::BC,
                (self.registers[Register16::BC] & 0x00ff) | (value << 8),
            ),
            Register8::C => self.set_r16(
                Register16::BC,
                (self.registers[Register16::BC] & 0xff00) | value,
            ),
            Register8::D => self.set_r16(
                Register16::DE,
                (self.registers[Register16::DE] & 0x00ff) | (value << 8),
            ),
            Register8::E => self.set_r16(
                Register16::DE,
                (self.registers[Register16::DE] & 0xff00) | value,
            ),
            Register8::H => self.set_r16(
                Register16::HL,
                (self.registers[Register16::HL] & 0x00ff) | (value << 8),
            ),
            Register8::L => self.set_r16(
                Register16::HL,
                (self.registers[Register16::HL] & 0xff00) | value,
            ),
        };
    }

    pub fn get_r8(&self, register: Register8) -> u8 {
        let reg = match register {
            Register8::A => self.registers[Register16::AF] >> 8,
            Register8::B => self.registers[Register16::BC] >> 8,
            Register8::C => self.registers[Register16::BC] & 0xff,
            Register8::D => self.registers[Register16::DE] >> 8,
            Register8::E => self.registers[Register16::DE] & 0xff,
            Register8::H => self.registers[Register16::HL] >> 8,
            Register8::L => self.registers[Register16::HL] & 0xff,
        };
        reg as u8
    }

    pub fn execute(&mut self, memory: &mut MemoryMap) -> Result<u8, CpuError> {
        let instruction_stream = &self.instruction_stream.clone();
        let mut iter = instruction_stream[self.program_counter..].iter();
        let opcode_byte = iter.next().ok_or(CpuError::MissingOpcodeByte)?;
        let mut ctx = DecodeContext { iter: &mut iter, cpu: self, memory };
        if let Ok(instruction) = INSTRUCTION_SET[*opcode_byte as usize](&mut ctx) {
            self.program_counter += instruction.bytes as usize;
            println!("{:?}", memory.timer_control());
            match instruction.mnemonic {
                _ =>(),
                // _ => println!("{:?} registers: {:?} flags: {:?} SP: {:?} PC: {:0x}", instruction, self.registers, self.flags, self.stack_pointer, self.program_counter)
            };
            return Ok(instruction.cycles);
        }
        Err(CpuError::NoCycles)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Flags {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Flags {
    pub fn set(&mut self, value: u8) {
        self.zero = (value >> 7) & 1 == 1;
        self.subtraction = (value >> 6) & 1 == 1;
        self.half_carry = (value >> 5) & 1 == 1;
        self.carry = (value >> 4) & 1 == 1;
    }

    pub fn clear(&mut self) {
        self.zero = false;
        self.subtraction = false;
        self.half_carry = false;
        self.carry = false;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            zero: true,
            subtraction: false,
            half_carry: true,
            carry: true,
        }
    }
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        let mut flags: u8 = 0;
        flags |= (self.zero as u8) << 7;
        flags |= (self.subtraction as u8) << 6;
        flags |= (self.half_carry as u8) << 5;
        flags |= (self.carry as u8) << 7;
        flags
    }
}
HI
