use crate::{
    DecodeContext, Mnemonic, errors::CpuError, extract_bytes, instructions::INSTRUCTION_SET,
    memory::Memory,
};

#[derive(Debug, Copy, Clone)]
pub enum Condition {
    NotZero,
    NotCarry,
    Zero,
    Carry,
}

/// Represents CPU registers of the Game Boy
#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub af: u16, // accumulator & flags
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16, // stack pointer
    pub pc: u16, // programer counter/pointer
    pub flags: Flags,
}

impl Default for Registers {
    /// https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
    fn default() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            af: 0x01b0,
            bc: 0x0013,
            de: 0x00d8,
            hl: 0x014d,
            sp: 0xfffe,
            pc: 0x0100,
            flags: Flags::default(),
        }
    }
}

impl Registers {
    pub fn get_r8(&self, register: R8) -> u8 {
        match register {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
        }
    }
    /// Set r8, mask higher or lower bits of parent 16-bit register and update accordingly.
    pub fn set_r8(&mut self, register: R8, value: u8) {
        match register {
            R8::A => {
                self.a = value;
                self.af = (self.af & 0x00ff) | ((value as u16) << 8);
            }
            R8::B => {
                self.b = value;
                self.bc = (self.bc & 0x00ff) | ((value as u16) << 8);
            }
            R8::C => {
                self.c = value;
                self.bc = self.bc & 0xff00 | value as u16;
            }
            R8::D => {
                self.d = value;
                self.de = (self.de & 0x00ff) | ((value as u16) << 8);
            }
            R8::E => {
                self.e = value;
                self.de = self.de & 0xff00 | value as u16;
            }
            R8::H => {
                self.h = value;
                self.hl = (self.hl & 0x00ff) | ((value as u16) << 8);
            }
            R8::L => {
                self.l = value;
                self.hl = self.hl & 0x0ff0 | value as u16;
            }
        }
    }
    pub fn get_r16(&self, register: R16) -> u16 {
        match register {
            R16::AF => self.af,
            R16::BC => self.bc,
            R16::DE => self.de,
            R16::HL => self.hl,
            R16::SP => self.sp,
            R16::PC => self.pc,
        }
    }
    /// Update 16-bit register, ensure the lower & higher 8-bit registers are updated as well.
    pub fn set_r16(&mut self, register: R16, value: u16) {
        let (msb, lsb) = extract_bytes(value);
        match register {
            R16::AF => {
                self.af = value;
                self.a = msb;
                self.flags.set(lsb);
            }
            R16::BC => {
                self.bc = value;
                self.b = msb;
                self.c = lsb;
            }
            R16::DE => {
                self.de = value;
                self.d = msb;
                self.e = lsb;
            }
            R16::HL => {
                self.hl = value;
                self.h = msb;
                self.l = lsb;
            }
            R16::SP => {
                self.sp = value;
            }
            R16::PC => {
                self.pc = value;
            }
        };
    }
}
/// 16-bit registers
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}
/// 8-bit registers
#[derive(Debug, Copy, Clone)]
pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    // Interrupt master enable flag
    pub ime: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            ime: false,
        }
    }
}

impl Cpu {
    /// Compare Condition to register flag
    pub fn cc(&mut self, condition: Condition) -> bool {
        match condition {
            Condition::NotZero => self.registers.flags.zero == false,
            Condition::Zero => self.registers.flags.zero == true,
            Condition::NotCarry => self.registers.flags.carry == false,
            Condition::Carry => self.registers.flags.carry == true,
        }
    }
    pub fn execute(&mut self, memory: &mut Memory) -> Result<u8, CpuError> {
        let pc = self.registers.pc as usize;
        let cloned_memory = memory.clone();
        let rom = &cloned_memory.rom()[pc..];
        let mut iter = rom.iter();
        let opcode_byte = *iter.next().ok_or(CpuError::MissingOpcodeByte)?;
        let mut ctx = DecodeContext {
            iter,
            cpu: self,
            memory,
        };
        if let Ok(instruction) = INSTRUCTION_SET[opcode_byte as usize](&mut ctx) {
            match instruction.mnemonic {
                Mnemonic::NOP | Mnemonic::RST => (),
                Mnemonic::RETI | Mnemonic::EI => self.ime = true,
                _ => ()
                // _ => println!("{instruction:?}"),
            };
            return Ok(instruction.cycles);
        }
        // perhaps panicking here makes more sense?
        Err(CpuError::NoCycles)
    }
}
/// Z = Zero, N = Subtraction, H = Half Carry, C = Carry
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Flags {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}
/// The flags register is actually the lower 8-bits of the AF register
/// flags are a part of almost all operations.
/// i've opted for a higher-level representation of flags instead of bit-fiddling the lower 8 bits of the AF register constantly
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
// make sure we can still cast flags back into a byte when certain operations i.e: PUSH AF need to operate on them
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
