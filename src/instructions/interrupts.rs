use crate::{Mnemonic, cpu::Cpu};

use super::{Instruction, InstructionResult};

/// DI
/// Disable Interrupts by clearing the IME flag.
pub fn di(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.ime = false;
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::DI,
        bytes: 1,
        cycles: 1,
    })
}

/// EI
/// Enable Interrupts by setting the IME flag.
/// The flag is only set after the instruction following EI.
pub fn ei(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::EI,
        bytes: 1,
        cycles: 1,
    })
}

/// HALT
/// The exact behavior of this instruction depends on the state of the IME flag, and whether interrupts are pending (i.e. whether ‘[IE] & [IF]’ is non-zero):
/// If the IME flag is set:
/// The CPU enters low-power mode until after an interrupt is about to be serviced. The handler is executed normally, and the CPU resumes execution after the HALT when that returns.
/// If the IME flag is not set, and no interrupts are pending:
/// As soon as an interrupt becomes pending, the CPU resumes execution. This is like the above, except that the handler is not called.
/// If the IME flag is not set, and some interrupt is pending:
/// The CPU continues execution after the HALT, but the byte after it is read twice in a row (PC is not incremented, due to a hardware bug).
pub fn halt(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::HALT,
        bytes: 1,
        cycles: 0,
    })
}
