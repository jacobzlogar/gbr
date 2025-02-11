use crate::{
    Mnemonic,
    cpu::{Cpu, Flag, Register8, Register16},
    memory::MemoryMap,
};

use super::{Arith16Bit, Instruction, InstructionResult, add_16bit};

/// LD HL,SP+e8
/// Add the signed value e8 to SP and copy the result in HL.
pub fn load_sp_e8_to_hl(e8: i8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let sp = cpu.stack_pointer;
    let x: i16 = sp as i16 + e8 as i16;
    let y = x.clamp(0, u16::MAX as i16) as u16;
    cpu.set_r16(Register16::HL, y);
    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);

    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD SP, HL
/// Copy register HL into register SP.
pub fn load_hl_to_sp(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    cpu.stack_pointer = hl;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD SP,n16
/// Copy the value n16 into register SP.
pub fn load_n16_to_sp(n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.stack_pointer = n16;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 3,
    })
}

/// LD [n16],SP
/// Copy SP & $FF at address n16 and SP >> 8 at address n16 + 1.
pub fn load_sp_to_immed_n16(
    n16: u16,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
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

/// ADD HL, SP
/// Add the value in SP to HL
pub fn add_sp_to_hl(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r16 = cpu.stack_pointer;
    let hl = cpu.registers[Register16::HL];
    let Arith16Bit { sum, flags } = add_16bit(r16, hl, None);
    cpu.set_r16(Register16::HL, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}
