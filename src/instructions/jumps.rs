use crate::{Mnemonic, cpu::Cpu, memory::MemoryMap};

use super::{Condition, Instruction, InstructionResult};

/// CALL n16
/// Call address n16.
/// This pushes the address of the instruction after the CALL on the stack, such that RET can pop it later; then, it executes an implicit JP n16.
pub fn call_n16(n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.stack.push(n16);
    Ok(Instruction {
        mnemonic: Mnemonic::CALL,
        bytes: 3,
        cycles: 6,
    })
}

/// CALL cc,n16
/// Call address n16 if condition cc is met.
pub fn call_cc_n16(
    n16: u16,
    condition: Condition,
    cpu: &mut Cpu,
) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::CALL,
        bytes: 3,
        cycles: 6,
    })
}

/// JP HL
/// Jump to address in HL; effectively, copy the value in register HL into PC.
pub fn jp_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::JP,
        bytes: 1,
        cycles: 1,
    })
}

/// JP n16
/// Jump to address n16; effectively, copy n16 into PC.
pub fn jp_n16(n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::JP,
        bytes: 3,
        cycles: 4,
    })
}

/// JP cc, n16
/// Jump to address n16 if condition cc is met.
pub fn jp_cc_n16(n16: u16, condition: Condition, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::JP,
        bytes: 1,
        cycles: 1,
    })
}

/// JR n16
/// Relative Jump to address n16.
/// The address is encoded as a signed 8-bit offset from the address immediately following the JR instruction, so the target address n16 must be between -128 and 127 bytes away. For example:
pub fn jr_n16(n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    //cpu.program_counter = cpu.program_counter + e8;
    Ok(Instruction {
        mnemonic: Mnemonic::JR,
        bytes: 2,
        cycles: 3,
    })
}

/// JP cc,n16
/// Relative Jump to address n16 if condition cc is met.
pub fn jr_cc_n16(n16: u16, condition: Condition, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    //cpu.program_counter = cpu.program_counter + e8;
    Ok(Instruction {
        mnemonic: Mnemonic::JR,
        bytes: 2,
        cycles: 3,
    })
}

/// RET cc
/// Return from subroutine if condition cc is met.
pub fn ret_cc(condition: Condition, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::RET,
        bytes: 1,
        cycles: 5,
    })
}

/// RET
/// Return from subroutine. This is basically a POP PC (if such an instruction existed). See POP r16 for an explanation of how POP works
pub fn ret(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::RET,
        bytes: 1,
        cycles: 4,
    })
}

/// RETI
/// Return from subroutine and enable interrupts. This is basically equivalent to executing EI then RET, meaning that IME is set right after this instruction.
pub fn reti(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::RETI,
        bytes: 1,
        cycles: 4,
    })
}

/// RST vec
/// Call address vec. This is a shorter and faster equivalent to CALL for suitable values of vec.
pub fn rst(vec: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::RST,
        bytes: 1,
        cycles: 4,
    })
}
