use crate::{Mnemonic, cpu::Cpu};

use super::{Condition, Instruction, InstructionResult};

pub fn jr(e8: i16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    //cpu.program_counter = cpu.program_counter + e8;
    Ok(Instruction {
        mnemonic: Mnemonic::JR,
        bytes: 2,
        cycles: 3,
    })
}
pub fn jr_cc(condition: Condition, e8: i8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::JR,
        bytes: 2,
        cycles: 3,
    })
}
