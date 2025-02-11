use crate::{Cpu, Mnemonic};

use super::{Instruction, InstructionResult};

pub fn daa(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::DAA,
        bytes: 1,
        cycles: 1,
    })
}

pub fn nop() -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::NOP,
        bytes: 1,
        cycles: 1,
    })
}

pub fn stop() -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::STOP,
        bytes: 1,
        cycles: 1,
    })
}
