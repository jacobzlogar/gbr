use std::ops::{Index, IndexMut};

use crate::{errors::CpuError, instructions::INSTRUCTION_SET, memory::MemoryMap};

/// The 6 CPU registers of the Game Boy
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
    Flags,
}

#[derive(Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub struct Cpu {
    pub stack: Vec<u8>,
    instruction_stream: Vec<u8>,
    pub registers: Registers,
    //pub instruction_pointer: usize,
    pub stack_pointer: u16,
    pub program_counter: usize,
}

impl Cpu {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            stack: vec![],
            instruction_stream: rom,
            registers: [0u16; 4],
            stack_pointer: 0,
            program_counter: 0,
            //instruction_pointer: 0,
        }
    }
    /// https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
    pub fn inititalize_registers(&mut self) {
        self.registers[Register16::AF] = 0x180;
        self.registers[Register16::BC] = 0x13;
        self.registers[Register16::DE] = 0xd8;
        self.registers[Register16::HL] = 0x14d;
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
            Register8::Flags => self.set_r16(
                Register16::AF,
                (self.registers[Register16::AF] & 0xff00) | value,
            ),
            _ => (),
        };
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let mut flags = self.get_r8(Register8::Flags);
        if value {
            match flag {
                Flag::Z => flags |= 1 << 7,
                Flag::N => flags |= 1 << 6,
                Flag::H => flags |= 1 << 5,
                Flag::C => flags |= 1 << 4,
            }
        } else {
            match flag {
                Flag::Z => flags &= !(1 << 7),
                Flag::N => flags &= !(1 << 6),
                Flag::H => flags &= !(1 << 5),
                Flag::C => flags &= !(1 << 4),
            }
        }
        self.set_r8(Register8::Flags, flags);
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
            Register8::Flags => self.registers[Register16::AF] & 0xff,
        };
        reg as u8
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        let flags = self.get_r8(Register8::Flags);
        let flag = match flag {
            // mask everything but the nth bit and shift right
            Flag::Z => (flags & 0x80) >> 7,
            Flag::N => (flags & 0x40) >> 6,
            Flag::H => (flags & 0x20) >> 5,
            Flag::C => (flags & 0x10) >> 4,
        };
        flag == 1
    }

    pub fn execute(&mut self, mut memory: &mut MemoryMap) -> Result<u8, CpuError> {
        let instruction_stream = &self.instruction_stream.clone();
        let mut iter = instruction_stream[self.program_counter..].iter();
        let opcode_byte = iter.next().ok_or(CpuError::MissingOpcodeByte)?;
        if let Ok(instruction) =
            INSTRUCTION_SET[*opcode_byte as usize](&mut iter, self, &mut memory)
        {
            // increment the PC based on the number of bytes consumed by this instruction
            self.program_counter += instruction.bytes as usize;
            match instruction.mnemonic {
                _ => (),
            };
            return Ok(instruction.cycles);
        }
        Err(CpuError::NoCycles)
    }
}
