use crate::{
    Mnemonic,
    cpu::{Cpu, R8, R16},
    instructions::InstructionResult,
    memory::Memory,
};

use super::Instruction;

/// BIT u3,r8
/// Test bit u3 in register r8, set the zero flag if bit not set.
pub fn bit_u3_r8(u3: u8, r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r8 = cpu.registers.get_r8(r8);
    let bit = (r8 >> u3) & u3;
    cpu.registers.flags.zero = bit == 1;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = true;
    Ok(Instruction {
        mnemonic: Mnemonic::BIT,
        bytes: 2,
        cycles: 2,
    })
}

/// BIT u3,[HL]
/// Test bit u3 in the byte pointed by HL, set the zero flag if bit not set.
pub fn bit_u3_hl(u3: u8, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let byte = mem.read(hl as usize);
    let bit = (byte >> u3) & u3;
    cpu.registers.flags.zero = bit == 1;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = true;
    Ok(Instruction {
        mnemonic: Mnemonic::BIT,
        bytes: 2,
        cycles: 3,
    })
}

/// RES u3,r8
/// Set bit u3 in register r8 to 0. Bit 0 is the rightmost one, bit 7 the leftmost one.
pub fn res_u3_r8(u3: u8, r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    reg |= 0 << u3;
    cpu.registers.set_r8(r8, reg);
    Ok(Instruction {
        mnemonic: Mnemonic::RES,
        bytes: 2,
        cycles: 2,
    })
}

/// RES u3,[HL]
/// Set bit u3 in the byte pointed by HL to 0. Bit 0 is the rightmost one, bit 7 the leftmost one.
pub fn res_u3_hl(u3: u8, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    byte |= 0 << u3;
    mem.write(hl as usize, byte);
    Ok(Instruction {
        mnemonic: Mnemonic::RES,
        bytes: 2,
        cycles: 4,
    })
}

/// SET u3,r8
/// Set bit u3 in register r8 to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
pub fn set_u3_r8(u3: u8, r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    reg |= 1 << u3;
    cpu.registers.set_r8(r8, reg);
    Ok(Instruction {
        mnemonic: Mnemonic::SET,
        bytes: 2,
        cycles: 2,
    })
}

/// SET u3,[HL]
/// Set bit u3 in the byte pointed by HL to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
pub fn set_u3_hl(u3: u8, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    byte |= 1 << u3;
    mem.write(hl as usize, byte);
    Ok(Instruction {
        mnemonic: Mnemonic::SET,
        bytes: 2,
        cycles: 4,
    })
}
