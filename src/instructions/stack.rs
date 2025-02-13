use crate::{
    Mnemonic,
    cpu::{Cpu, Flag, Register8, Register16},
    memory::MemoryMap,
};

use super::{arithmetic_16bit::{add_16bit, Arith16Bit}, Instruction, InstructionResult};

/// ADD HL, SP
/// Add the value in SP to HL
pub fn add_hl_sp(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r16 = cpu.stack_pointer;
    let hl = cpu.registers[Register16::HL];
    let Arith16Bit { sum, flags } = add_16bit(r16, hl, None);
    cpu.set_r16(Register16::HL, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}

/// ADD SP,e8
/// Add the signed value e8 to SP.
pub fn add_sp_e8(e8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    // let r16 = cpu.stack_pointer;
    // let hl = cpu.registers[Register16::HL];
    // let Arith16Bit { sum, flags } = add_16bit(r16, hl, None);
    // cpu.set_r16(Register16::HL, sum);
    // cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 2,
        cycles: 4,
    })
}

/// DEC SP
/// Decrement the value in register SP by 1.
pub fn dec_sp(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::DEC,
        bytes: 1,
        cycles: 2,
    })
}

/// INC SP
/// Increment the value in register SP by 1
pub fn inc_sp(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::INC,
        bytes: 1,
        cycles: 2,
    })
}

/// LD SP,n16
/// Copy the value n16 into register SP.
pub fn load_sp_n16(n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.stack_pointer = n16;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 3,
    })
}

/// LD [n16],SP
/// Copy SP & $FF at address n16 and SP >> 8 at address n16 + 1.
pub fn load_a16_sp(n16: u16, cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let sp = cpu.stack_pointer;
    let n16 = n16 as usize;
    mem.write(n16, (sp & 0xff) as u8);
    mem.write(n16 + 1, (sp & 0xff) as u8);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 5,
    })
}

/// LD HL,SP+e8
/// Add the signed value e8 to SP and copy the result in HL.
pub fn load_hl_sp_e8(e8: i8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let sp = cpu.stack_pointer;
    let x: i16 = sp as i16 + e8 as i16;
    let y = x.clamp(0, u16::MAX as i16) as u16;
    cpu.set_r16(Register16::HL, y);
    cpu.flags.zero = false;
    cpu.flags.subtraction = false;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD SP, HL
/// Copy register HL into register SP.
pub fn load_sp_hl(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    cpu.stack_pointer = hl;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// POP AF
/// Pop register AF from the stack. This is roughly equivalent to the following imaginary instructions:
/// LD F, [SP]  ; See below for individual flags
/// INC SP
/// LD A, [SP]
/// INC SP
pub fn pop_af(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::POP,
        bytes: 1,
        cycles: 3,
    })
}

/// POP r16
/// Pop register r16 from the stack. This is roughly equivalent to the following imaginary instructions:
/// LD LOW(r16), [SP]   ; C, E or L
/// INC SP
/// LD HIGH(r16), [SP]  ; B, D or H
/// INC SP
pub fn pop_r16(r16: Register16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::POP,
        bytes: 1,
        cycles: 3,
    })
}

/// PUSH AF
/// Push register AF into the stack. This is roughly equivalent to the following imaginary instructions:
/// DEC SP
/// LD [SP], A
/// DEC SP
/// LD [SP], F.Z << 7 | F.N << 6 | F.H << 5 | F.C << 4
pub fn push_af(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::PUSH,
        bytes: 1,
        cycles: 4,
    })
}

/// PUSH r16
/// Push register r16 into the stack. This is roughly equivalent to the following imaginary instructions:
/// DEC SP
/// LD [SP], HIGH(r16)  ; B, D or H
/// DEC SP
/// LD [SP], LOW(r16)   ; C, E or L
pub fn push_r16(r16: Register16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::PUSH,
        bytes: 1,
        cycles: 4,
    })
}
