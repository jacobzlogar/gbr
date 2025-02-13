use crate::{cpu::Cpu, Mnemonic};

use super::{Instruction, InstructionResult};

/// CCF
/// Complement Carry Flag.
pub fn ccf(
    cpu: &mut Cpu
) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::CCF,
        bytes: 1,
        cycles: 1,
    })
}

/// SCF
/// Set Carry Flag.
pub fn scf(
    cpu: &mut Cpu
) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::SCF,
        bytes: 1,
        cycles: 1,
    })
}
