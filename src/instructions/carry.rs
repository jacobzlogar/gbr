use crate::{Mnemonic, cpu::Cpu};

use super::{Instruction, InstructionResult};

/// CCF
/// Complement Carry Flag.
pub fn ccf(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = !cpu.registers.flags.carry;
    Ok(Instruction {
        mnemonic: Mnemonic::CCF,
        bytes: 1,
        cycles: 1,
    })
}

/// SCF
/// Set Carry Flag.
pub fn scf(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = true;
    Ok(Instruction {
        mnemonic: Mnemonic::SCF,
        bytes: 1,
        cycles: 1,
    })
}
