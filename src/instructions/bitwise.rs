use crate::{
    Mnemonic,
    cpu::{Cpu, Register8, Register16},
    memory::MemoryMap,
};

use super::{Instruction, InstructionResult};

fn and_flags(result: u8) -> u8 {
    let mut flags: u8 = 0;
    flags |= ((result == 0) as u8) << 7;
    flags |= 0 << 6;
    flags |= 1 << 5;
    flags |= 0 << 4;
    flags
}

/// AND A,r8
/// Set A to the bitwise AND between the value in r8 and A.
pub fn and_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let b = a & r8;
    cpu.set_r8(Register8::A, b);
    cpu.flags.set(and_flags(b));
    Ok(Instruction {
        mnemonic: Mnemonic::AND,
        bytes: 1,
        cycles: 1,
    })
}

/// AND A, [HL]
/// Set A to the bitwise AND between the byte pointed to by HL and A.
pub fn and_a_immed_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let b = byte & a;
    cpu.set_r8(Register8::A, b);
    cpu.flags.set(and_flags(b));
    Ok(Instruction {
        mnemonic: Mnemonic::AND,
        bytes: 1,
        cycles: 2,
    })
}

/// AND A, n8
/// Set A to the bitwise AND between the value n8 and A.
pub fn and_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let b = n8 & a;
    cpu.set_r8(Register8::A, b);
    cpu.flags.set(and_flags(b));
    Ok(Instruction {
        mnemonic: Mnemonic::AND,
        bytes: 2,
        cycles: 2,
    })
}

/// CPL
/// ComPLement accumulator (A = ~A); also called bitwise NOT.
pub fn cpl(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let a = a != a;
    cpu.flags.subtraction = true;
    cpu.flags.half_carry = true;
    cpu.set_r8(Register8::A, a as u8);
    Ok(Instruction {
        mnemonic: Mnemonic::CPL,
        bytes: 1,
        cycles: 1,
    })
}

/// OR A, r8
/// Set A to the bitwise OR between the value in r8 and A.
pub fn or_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let b = a | r8;
    cpu.flags.clear();
    cpu.flags.zero = b == 0;
    cpu.set_r8(Register8::A, b);
    Ok(Instruction {
        mnemonic: Mnemonic::OR,
        bytes: 1,
        cycles: 1,
    })
}
/// OR A, [HL]
/// Set A to the bitwise OR between the byte pointed to by HL and A.
pub fn or_a_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let b = a | byte;
    cpu.flags.clear();
    cpu.flags.zero = b == 0;
    cpu.set_r8(Register8::A, b);
    Ok(Instruction {
        mnemonic: Mnemonic::OR,
        bytes: 1,
        cycles: 2,
    })
}
/// OR A, n8
/// Set A to the bitwise OR between the value n8 and A.
pub fn or_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let b = a | n8;
    cpu.flags.clear();
    cpu.flags.zero = b == 0;
    cpu.set_r8(Register8::A, a as u8);
    Ok(Instruction {
        mnemonic: Mnemonic::OR,
        bytes: 2,
        cycles: 2,
    })
}

/// XOR A, r8
/// Set A to the bitwise XOR between the value in r8 and A.
pub fn xor_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let b = a ^ r8;
    cpu.flags.clear();
    cpu.flags.zero = b == 0;
    cpu.set_r8(Register8::A, b);
    Ok(Instruction {
        mnemonic: Mnemonic::XOR,
        bytes: 1,
        cycles: 1,
    })
}

/// XOR A, [HL]
/// Set A to the bitwise XOR between the byte pointed to by HL and A.
pub fn xor_a_immed_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let b = a ^ byte;
    cpu.flags.clear();
    cpu.flags.zero = b == 0;
    cpu.set_r8(Register8::A, b);
    Ok(Instruction {
        mnemonic: Mnemonic::XOR,
        bytes: 1,
        cycles: 2,
    })
}

/// XOR A, n8
/// Set A to the bitwise XOR between the value n8 and A.
pub fn xor_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let b = a ^ n8;
    cpu.flags.clear();
    cpu.flags.zero = b == 0;
    cpu.set_r8(Register8::A, a as u8);
    Ok(Instruction {
        mnemonic: Mnemonic::XOR,
        bytes: 2,
        cycles: 2,
    })
}
