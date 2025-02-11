// TODO: not sure this is correct
use crate::{
    Cpu, Mnemonic,
    cpu::{Flag, Register8, Register16},
    memory::MemoryMap,
};

use super::{Instruction, InstructionResult};

pub fn rla(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.set_r8(Register8::Flags, 0x00);
    let mut carry = cpu.get_flag(Flag::C);
    let mut a = cpu.get_r8(Register8::A);
    let msb = (a & 0b1000_0000) != 0;
    a = (a << 1) | (carry as u8);
    carry = msb;
    cpu.set_flag(Flag::C, carry);
    cpu.set_r8(Register8::A, a);
    Ok(Instruction {
        mnemonic: Mnemonic::RLA,
        bytes: 1,
        cycles: 1,
    })
}

pub fn rra(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let carry = cpu.get_flag(Flag::C) as u8;
    let mut a = cpu.get_r8(Register8::A);
    // H=0,Z=0,N=0,C=LSB of A
    cpu.set_r8(Register8::Flags, (a & 1) << 4);
    // put the previous carry flag into the MSB of A
    // shift the rest of the bits right 1
    a = (a >> 1) | (carry << 7);
    cpu.set_r8(Register8::A, a);
    Ok(Instruction {
        mnemonic: Mnemonic::RRA,
        bytes: 1,
        cycles: 1,
    })
}

/// Rotate register r8 right, through the carry flag.
/// Update Z to result == 0
/// Update C to LSB of A == 0
pub fn rr_r8(cpu: &mut Cpu, register: Register8) -> InstructionResult<Instruction> {
    let carry = cpu.get_flag(Flag::C) as u8;
    let mut r8 = cpu.get_r8(register);
    let lsb = r8 & 1;
    // H=0,N=0, update Z/C according to result
    cpu.set_r8(Register8::Flags, 0);
    cpu.set_flag(Flag::C, lsb == 0);
    // put the previous carry flag into the MSB of r8
    // shift the rest of the bits right 1
    r8 = (r8 >> 1) | (carry << 7);
    cpu.set_r8(register, r8);
    cpu.set_flag(Flag::Z, r8 == 0);
    Ok(Instruction {
        mnemonic: Mnemonic::RR,
        bytes: 2,
        cycles: 2,
    })
}

/// Rotate the byte pointed to by HL right, through the carry flag.
/// Flags are updated the same way as RR, R8
pub fn rr_immed8(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let carry = cpu.get_flag(Flag::C) as u8;
    let r16 = cpu.registers[Register16::HL];
    let mut immed8 = mem.read(r16 as usize);
    let lsb = immed8 & 1;
    // H=0,N=0, update Z/C according to result
    cpu.set_r8(Register8::Flags, 0);
    cpu.set_flag(Flag::C, lsb == 0);
    // put the previous carry flag into the MSB of immed8
    // shift the rest of the bits right 1
    immed8 = (immed8 >> 1) | (carry << 7);
    mem.write(r16 as usize, immed8);
    cpu.set_flag(Flag::Z, immed8 == 0);
    Ok(Instruction {
        mnemonic: Mnemonic::RR,
        bytes: 2,
        cycles: 4,
    })
}

pub fn rlca(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.set_r8(Register8::Flags, 0x00);
    let a = cpu.get_r8(Register8::A);
    if a & 0xf != 0 {
        cpu.set_flag(Flag::C, true);
    }
    cpu.set_r8(Register8::A, a << 1);
    Ok(Instruction {
        mnemonic: Mnemonic::RLCA,
        bytes: 1,
        cycles: 1,
    })
}

pub fn rrca(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.set_r8(Register8::Flags, 0x00);
    let a = cpu.get_r8(Register8::A);
    if a & 1 != 0 {
        cpu.set_flag(Flag::C, true);
    }
    cpu.set_r8(Register8::A, a >> 1);
    Ok(Instruction {
        mnemonic: Mnemonic::RLCA,
        bytes: 1,
        cycles: 1,
    })
}
