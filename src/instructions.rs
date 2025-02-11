use crate::instructions::interrupts::halt;
use arithmetic_16bit::{add_r16_to_hl, dec_r16, inc_r16};
use arithmetic_8bit::{dec_r8, inc_r8};
use bitshift::{rla, rlca, rra, rrca};
use jumps::{jr, jr_cc};
use load::{load_a_to_immed_r16, load_hl_to_r8, load_immed_r16_to_a, load_n16_to_r16, load_n8_to_r8, load_r8_to_hl, load_r8_to_r8};
use misc::{daa, nop, stop};
use stack::load_sp_to_immed_n16;

use crate::{
    Mnemonic, Thunk,
    cpu::{Register8, Register16},
    errors::DecodeError,
};

pub mod arithmetic_16bit;
pub mod arithmetic_8bit;
pub mod bitshift;
pub mod jumps;
pub mod load;
pub mod misc;
pub mod stack;
pub mod interrupts;

#[derive(Debug)]
pub enum Condition {
    NotZero,
    NotCarry,
    Zero,
    Carry,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub bytes: u8,
    pub cycles: u8,
}

#[derive(Debug)]
pub struct Arith8Bit {
    sum: u8,
    flags: u8,
}

#[derive(Debug)]
pub struct Arith16Bit {
    sum: u16,
    flags: u8,
}

// maybe this should just be a method on `Arith8Bit`
pub fn add_8bit(a: u8, b: u8, carry_flag: Option<u8>) -> Arith8Bit {
    let carry = match carry_flag {
        Some(num) => num,
        None => 0,
    };
    // https://stackoverflow.com/a/57822729 thanks
    let half_carry = ((a & 0x0f) + (b & 0x0f) & 0x10) == 0x10;
    let (sum, carry) = a.overflowing_add(b + carry);
    let mut flags: u8 = 0;
    // set the zero flag if sum == 0
    flags |= ((sum == 0) as u8) << 7;
    // set the subtraction flag to false
    flags |= 0 << 6;
    // set the half carry flag
    flags |= (half_carry as u8) << 5;
    // set the carry flag
    flags |= (carry as u8) << 4;
    Arith8Bit { sum, flags }
}

pub fn sub_8bit(a: u8, b: u8, carry_flag: Option<u8>) -> Arith8Bit {
    let carry = match carry_flag {
        Some(num) => num as u8,
        None => 0,
    };
    let a_mask = a as i16 & 0x0f;
    let b_mask = b as i16 & 0x0f;
    let half_carry = a_mask - b_mask < 0;
    let (sum, _) = a.overflowing_sub(b);
    let carry = b >= sum;
    let mut flags: u8 = 0;
    flags |= ((sum == 0) as u8) << 7;
    flags |= 1 << 6;
    flags |= (half_carry as u8) << 5;
    flags |= (carry as u8) << 4;
    Arith8Bit { sum, flags }
}

pub fn add_16bit(a: u16, b: u16, carry_flag: Option<u8>) -> Arith16Bit {
    let carry = match carry_flag {
        Some(num) => num as u16,
        None => 0,
    };
    let half_carry = ((a & 0x00ff) + (b & 0x00ff) & 0x0100) == 0x0100;
    let (sum, carry) = a.overflowing_add(b + carry);
    let mut flags: u8 = 0;
    flags |= ((sum == 0) as u8) << 7;
    flags |= 0 << 6;
    flags |= (half_carry as u8) << 5;
    flags |= (carry as u8) << 4;
    Arith16Bit { sum, flags }
}

pub fn sub_16bit(a: u16, b: u16, carry_flag: Option<u8>) -> Arith16Bit {
    // let carry = match carry_flag {
    //     Some(num) => num as u16,
    //     None => 0,
    // };

    let half_carry = (a & 0x00ff) - (b & 0x00ff) < 0;
    let (sum, carry) = a.overflowing_sub(b);
    let mut flags: u8 = 0;
    flags |= ((sum == 0) as u8) << 7;
    flags |= 0 << 6;
    flags |= (half_carry as u8) << 5;
    flags |= (carry as u8) << 4;
    Arith16Bit { sum, flags }
}

pub type InstructionResult<T> = std::result::Result<T, DecodeError>;

fn get_i8(iter: &mut std::slice::Iter<u8>) -> InstructionResult<i8> {
    Ok(*iter.next().ok_or(DecodeError::MissingDataByte)? as i8)
}
fn get_u8(iter: &mut std::slice::Iter<u8>) -> InstructionResult<u8> {
    Ok(*iter.next().ok_or(DecodeError::MissingDataByte)?)
}
fn get_i16(iter: &mut std::slice::Iter<u8>) -> InstructionResult<i16> {
    let n16 = i16::from_le_bytes([
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
    ]);
    Ok(n16)
}
fn get_u16(iter: &mut std::slice::Iter<u8>) -> InstructionResult<u16> {
    let n16 = u16::from_le_bytes([
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
    ]);
    Ok(n16)
}

pub const INSTRUCTION_SET: [Thunk; 112] = [
    // row 1
    |_, _, _| nop(),
    |iter, cpu, _| load_n16_to_r16(Register16::BC, get_u16(iter)?, cpu),
    |_, cpu, mem| load_a_to_immed_r16(Register16::BC, cpu, mem),
    |_, cpu, _| inc_r16(Register16::BC, cpu),
    |_, cpu, _| inc_r8(Register8::B, cpu),
    |_, cpu, _| dec_r8(Register8::B, cpu),
    |iter, cpu, _| load_n8_to_r8(Register8::B, get_u8(iter)?, cpu),
    |_, cpu, _| rlca(cpu),
    |iter, cpu, mem| load_sp_to_immed_n16(get_u16(iter)?, cpu, mem),
    |_, cpu, _| add_r16_to_hl(Register16::BC, cpu),
    |_, cpu, mem| load_immed_r16_to_a(Register16::BC, cpu, mem),
    |_, cpu, _| dec_r16(Register16::BC, cpu),
    |_, cpu, _| inc_r8(Register8::C, cpu),
    |_, cpu, _| dec_r8(Register8::C, cpu),
    |iter, cpu, _| load_n8_to_r8(Register8::C, get_u8(iter)?, cpu),
    |_, cpu, _| rrca(cpu),
    // row 2
    |_, _, _| stop(),
    |iter, cpu, _| load_n16_to_r16(Register16::DE, get_u16(iter)?, cpu),
    |_, cpu, mem| load_a_to_immed_r16(Register16::DE, cpu, mem),
    |_, cpu, _| inc_r16(Register16::DE, cpu),
    |_, cpu, _| inc_r8(Register8::D, cpu),
    |_, cpu, _| dec_r8(Register8::D, cpu),
    |iter, cpu, _| load_n8_to_r8(Register8::D, get_u8(iter)?, cpu),
    |_, cpu, _| rla(cpu),
    |iter, cpu, _| jr(get_i16(iter)?, cpu),
    |_, cpu, _| add_r16_to_hl(Register16::DE, cpu),
    |_, cpu, mem| load_immed_r16_to_a(Register16::DE, cpu, mem),
    |_, cpu, _| dec_r16(Register16::DE, cpu),
    |_, cpu, _| inc_r8(Register8::E, cpu),
    |_, cpu, _| dec_r8(Register8::E, cpu),
    |iter, cpu, _| load_n8_to_r8(Register8::E, get_u8(iter)?, cpu),
    |_, cpu, _| rra(cpu),
    // row 3
    |iter, cpu, _| jr_cc(Condition::NotZero, get_i8(iter)?, cpu),
    |iter, cpu, _| load_n16_to_r16(Register16::DE, get_u16(iter)?, cpu),
    |_, cpu, mem| load_a_to_immed_r16(Register16::DE, cpu, mem),
    |_, cpu, _| inc_r16(Register16::HL, cpu),
    |_, cpu, _| inc_r8(Register8::H, cpu),
    |_, cpu, _| dec_r8(Register8::H, cpu),
    |iter, cpu, _| load_n8_to_r8(Register8::H, get_u8(iter)?, cpu),
    |_, cpu, _| daa(cpu),
    |iter, cpu, _| jr_cc(Condition::Zero, get_i8(iter)?, cpu),
    |_, cpu, _| add_r16_to_hl(Register16::HL, cpu),
    |_, cpu, mem| load_immed_r16_to_a(Register16::DE, cpu, mem),
    |_, cpu, _| dec_r16(Register16::DE, cpu),
    |_, cpu, _| inc_r8(Register8::E, cpu),
    |_, cpu, _| dec_r8(Register8::E, cpu),
    |iter, cpu, _| load_n8_to_r8(Register8::E, get_u8(iter)?, cpu),
    |_, cpu, _| rra(cpu),
    // row 4
    |_, cpu, _| load_r8_to_r8(Register8::B, Register8::B, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::C, Register8::B, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::D, Register8::B, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::E, Register8::B, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::H, Register8::B, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::L, Register8::B, cpu),
    |_, cpu, mem| load_hl_to_r8(Register8::B, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::A, Register8::B, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::B, Register8::C, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::C, Register8::C, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::D, Register8::C, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::E, Register8::C, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::H, Register8::C, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::L, Register8::C, cpu),
    |_, cpu, mem| load_hl_to_r8(Register8::C, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::A, Register8::C, cpu),
    // row 5
    |_, cpu, _| load_r8_to_r8(Register8::B, Register8::D, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::C, Register8::D, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::D, Register8::D, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::E, Register8::D, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::H, Register8::D, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::L, Register8::D, cpu),
    |_, cpu, mem| load_hl_to_r8(Register8::D, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::A, Register8::D, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::B, Register8::E, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::C, Register8::E, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::D, Register8::E, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::E, Register8::E, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::H, Register8::E, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::L, Register8::E, cpu),
    |_, cpu, mem| load_hl_to_r8(Register8::E, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::A, Register8::E, cpu),
    // row 6
    |_, cpu, _| load_r8_to_r8(Register8::B, Register8::H, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::C, Register8::H, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::D, Register8::H, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::E, Register8::H, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::H, Register8::H, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::L, Register8::H, cpu),
    |_, cpu, mem| load_hl_to_r8(Register8::H, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::A, Register8::H, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::B, Register8::L, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::C, Register8::L, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::D, Register8::L, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::E, Register8::L, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::H, Register8::L, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::L, Register8::L, cpu),
    |_, cpu, mem| load_hl_to_r8(Register8::L, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::A, Register8::L, cpu),
    // row 7
    |_, cpu, mem| load_r8_to_hl(Register8::B, cpu, mem),
    |_, cpu, mem| load_r8_to_hl(Register8::C, cpu, mem),
    |_, cpu, mem| load_r8_to_hl(Register8::D, cpu, mem),
    |_, cpu, mem| load_r8_to_hl(Register8::E, cpu, mem),
    |_, cpu, mem| load_r8_to_hl(Register8::H, cpu, mem),
    |_, cpu, mem| load_r8_to_hl(Register8::L, cpu, mem),
    |_, _, _| halt(),
    |_, cpu, mem| load_r8_to_hl(Register8::A, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::B, Register8::A, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::C, Register8::A, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::D, Register8::A, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::E, Register8::A, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::H, Register8::A, cpu),
    |_, cpu, _| load_r8_to_r8(Register8::L, Register8::A, cpu),
    |_, cpu, mem| load_hl_to_r8(Register8::A, cpu, mem),
    |_, cpu, _| load_r8_to_r8(Register8::A, Register8::A, cpu),
];
