use crate::{
    Mnemonic, cc,
    cpu::{Cpu, R16},
    memory::Memory,
};

use super::{Condition, Instruction, InstructionResult, pop_stack, push_stack};

/// CALL n16
/// Call address n16.
/// This pushes the address of the instruction after the CALL on the stack, such that RET can pop it later; then, it executes an implicit JP n16.
pub fn call_n16(n16: u16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    push_stack(cpu.registers.pc + 3, cpu, mem);
    cpu.registers.set_r16(R16::PC, n16);
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
    mem: &mut Memory,
) -> InstructionResult<Instruction> {
    if cc(cpu, condition) {
        push_stack(cpu.registers.pc + 3, cpu, mem);
        cpu.registers.set_r16(R16::PC, n16);
    }
    Ok(Instruction {
        mnemonic: Mnemonic::CALL,
        bytes: 3,
        cycles: 6,
    })
}

/// JP HL
/// Jump to address in HL; effectively, copy the value in register HL into PC.
pub fn jp_hl(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.set_r16(R16::PC, cpu.registers.hl);
    Ok(Instruction {
        mnemonic: Mnemonic::JP,
        bytes: 1,
        cycles: 1,
    })
}

/// JP n16
/// Jump to address n16; effectively, copy n16 into PC.
pub fn jp_n16(n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    println!("jump to: {n16}");
    cpu.registers.set_r16(R16::PC, n16);
    Ok(Instruction {
        mnemonic: Mnemonic::JP,
        bytes: 3,
        cycles: 4,
    })
}

/// JP cc, n16
/// Jump to address n16 if condition cc is met.
pub fn jp_cc_n16(n16: u16, condition: Condition, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    if cc(cpu, condition) {
        cpu.registers.set_r16(R16::PC, n16);
    }
    Ok(Instruction {
        mnemonic: Mnemonic::JP,
        bytes: 1,
        cycles: 1,
    })
}

/// JR n16
/// Relative Jump to address n16.
/// The address is encoded as a signed 8-bit offset from the address immediately following the JR instruction, so the target address n16 must be between -128 and 127 bytes away. For example:
pub fn jr_n16(e8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let offset = e8 as i8;
    cpu.registers
        .set_r16(R16::PC, cpu.registers.pc.wrapping_add(offset as u16));
    Ok(Instruction {
        mnemonic: Mnemonic::JR,
        bytes: 2,
        cycles: 3,
    })
}

/// JR cc,n16
/// Relative Jump to address n16 if condition cc is met.
pub fn jr_cc_n16(e8: u8, condition: Condition, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    if cc(cpu, condition) {
        let offset = e8 as i8;
        cpu.registers
            .set_r16(R16::PC, cpu.registers.pc.wrapping_add(offset as u16));
    }
    Ok(Instruction {
        mnemonic: Mnemonic::JR,
        bytes: 2,
        cycles: 3,
    })
}

/// RET cc
/// Return from subroutine if condition cc is met.
pub fn ret_cc(
    condition: Condition,
    cpu: &mut Cpu,
    mem: &mut Memory,
) -> InstructionResult<Instruction> {
    if cc(cpu, condition) {
        pop_stack(R16::PC, cpu, mem);
    }
    Ok(Instruction {
        mnemonic: Mnemonic::RET,
        bytes: 1,
        cycles: 5,
    })
}

/// RET
/// Return from subroutine. This is basically a POP PC (if such an instruction existed). See POP r16 for an explanation of how POP works
pub fn ret(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    pop_stack(R16::PC, cpu, mem);
    Ok(Instruction {
        mnemonic: Mnemonic::RET,
        bytes: 1,
        cycles: 4,
    })
}

/// RETI
/// Return from subroutine and enable interrupts. This is basically equivalent to executing EI then RET, meaning that IME is set right after this instruction.
pub fn reti(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    pop_stack(R16::PC, cpu, mem);
    Ok(Instruction {
        mnemonic: Mnemonic::RETI,
        bytes: 1,
        cycles: 4,
    })
}

/// RST vec
/// Call address vec. This is a shorter and faster equivalent to CALL for suitable values of vec.
pub fn rst(vec: u16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    push_stack(cpu.registers.pc + 2, cpu, mem);
    cpu.registers.set_r16(R16::PC, vec);
    Ok(Instruction {
        mnemonic: Mnemonic::RST,
        bytes: 1,
        cycles: 4,
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_call_n16() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();
        assert_eq!(cpu.registers.sp, 0xfffe);
        call_n16(0x420, &mut cpu, &mut mem).unwrap();
        assert_eq!(cpu.registers.sp, 0xfffc);
        assert_eq!(mem.read(0xfffc), 0x03);
        assert_eq!(mem.read(0xfffd), 0x01);
    }

    #[test]
    fn test_call_cc_n16() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();
        assert_eq!(cpu.registers.sp, 0xfffe);
        call_cc_n16(0x420, Condition::Carry, &mut cpu, &mut mem).unwrap();
        assert_eq!(cpu.registers.sp, 0xfffc);
        assert_eq!(mem.read(0xfffc), 0x03);
        assert_eq!(mem.read(0xfffd), 0x01);
    }

    #[test]
    fn test_jp_hl() {
        let mut cpu = Cpu::default();
        cpu.registers.set_r16(R16::HL, 0x420);
        jp_hl(&mut cpu).unwrap();
        assert_eq!(cpu.registers.pc, 0x420);
    }

    #[test]
    fn test_jp_n16() {
        let mut cpu = Cpu::default();
        jp_n16(0x420, &mut cpu).unwrap();
        assert_eq!(cpu.registers.pc, 0x420);
    }

    #[test]
    fn test_jp_cc_n16() {
        let mut cpu = Cpu::default();
        jp_cc_n16(0x420, Condition::Carry, &mut cpu).unwrap();
        assert_eq!(cpu.registers.pc, 0x420);
    }

    #[test]
    fn test_jr_n16() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.registers.pc, 0x0100);
        jr_n16(0xfc, &mut cpu).unwrap();
        assert_eq!(cpu.registers.pc, 0x00fc);
    }

    #[test]
    fn test_jr_cc_n16() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.registers.pc, 0x0100);
        jr_cc_n16(0xfc, Condition::Carry, &mut cpu).unwrap();
        assert_eq!(cpu.registers.pc, 0x00fc);
        cpu.registers.set_r16(R16::PC, 0x0100);
        jr_cc_n16(0xfc, Condition::NotCarry, &mut cpu).unwrap();
        assert_eq!(cpu.registers.pc, 0x0100);
    }

    #[test]
    fn test_ret_cc() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();
        push_stack(cpu.registers.pc + 3, &mut cpu, &mut mem);
        ret_cc(Condition::Carry, &mut cpu, &mut mem).unwrap();
        assert_eq!(cpu.registers.pc, 0x103);
    }

    #[test]
    fn test_ret() {
        // let (cpu, mem) = test.setup();
        // cpu.registers.sp = 0;
        // mem[0] =
    }

    #[test]
    fn test_reti() {}

    #[test]
    fn test_rst() {}
}
