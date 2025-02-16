use crate::{
    Cpu, Mnemonic,
    memory::{Memory, registers::DIV},
};

use super::{Instruction, InstructionResult};

/// Decimal Adjust Accumulator.
/// Designed to be used after performing an arithmetic instruction (ADD, ADC, SUB, SBC) whose inputs were in Binary-Coded Decimal (BCD), adjusting the result to likewise be in BCD.
/// The exact behavior of this instruction depends on the state of the subtract flag N:
/// If the subtract flag N is set:
/// Initialize the adjustment to 0.
/// If the half-carry flag H is set, then add $6 to the adjustment.
/// If the carry flag is set, then add $60 to the adjustment.
/// Subtract the adjustment from A.
/// If the subtract flag N is not set:
/// Initialize the adjustment to 0.
/// If the half-carry flag H is set or A & $F > $9, then add $6 to the adjustment.
/// If the carry flag is set or A > $99, then add $60 to the adjustment and set the carry flag.
/// Add the adjustment to A.
pub fn daa(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::DAA,
        bytes: 1,
        cycles: 1,
    })
}

/// NOP
/// No OPeration.
pub fn nop() -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::NOP,
        bytes: 1,
        cycles: 1,
    })
}

/// STOP
/// Enter CPU very low power mode. Also used to switch between GBC double speed and normal speed CPU modes.
/// The exact behavior of this instruction is fragile and may interpret its second byte as a separate instruction (see the Pan Docs),
/// which is why rgbasm(1) allows explicitly specifying the second byte (STOP n8) to override the default of $00 (a NOP instruction).
pub fn stop(mem: &mut Memory) -> InstructionResult<Instruction> {
    mem.write(DIV, 0);
    Ok(Instruction {
        mnemonic: Mnemonic::STOP,
        bytes: 2,
        cycles: 0,
    })
}
