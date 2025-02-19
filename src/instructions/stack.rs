use crate::{
    Mnemonic,
    cpu::{Cpu, Flag, R8, R16},
    memory::Memory,
};

use super::{
    Instruction, InstructionResult,
    arithmetic_16bit::{Arith16Bit, add_16bit},
};

/// Push onto the stack
pub fn push_stack(n16: u16, cpu: &mut Cpu, mem: &mut Memory) {
    let high = (n16 & 0xff00) >> 8;
    cpu.registers.set_r16(R16::SP, cpu.registers.sp - 1);
    mem.write(cpu.registers.sp as usize, high as u8);
    let low = (n16 & 0xff) as u8;
    cpu.registers.set_r16(R16::SP, cpu.registers.sp - 1);
    mem.write(cpu.registers.sp as usize, low);
}

/// Pop from the stack
pub fn pop_stack(r16: R16, cpu: &mut Cpu, mem: &mut Memory) {
    let mut n16: u16 = 0;
    let low = mem.read(cpu.registers.sp as usize) as u16;
    n16 |= low;
    cpu.registers.set_r16(R16::SP, cpu.registers.sp + 1);
    let high = mem.read(cpu.registers.sp as usize) as u16;
    n16 |= high << 8;
    cpu.registers.set_r16(R16::SP, cpu.registers.sp + 1);
    cpu.registers.set_r16(r16, n16);
}

/// ADD HL, SP
/// Add the value in SP to HL
pub fn add_hl_sp(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let Arith16Bit { sum, flags } = add_16bit(cpu.registers.sp, cpu.registers.hl, None);
    cpu.registers.flags.set(flags);
    cpu.registers.set_r16(R16::HL, sum);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}

/// ADD SP,e8
/// Add the signed value e8 to SP.
pub fn add_sp_e8(e8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let offset = e8 as i8;
    let _ = cpu.registers.pc.wrapping_add(offset as u16);
    // TODO
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 2,
        cycles: 4,
    })
}

/// DEC SP
/// Decrement the value in register SP by 1.
pub fn dec_sp(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.set_r16(R16::SP, cpu.registers.sp - 1);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::DEC,
        bytes: 1,
        cycles: 2,
    })
}

/// INC SP
/// Increment the value in register SP by 1
pub fn inc_sp(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.set_r16(R16::SP, cpu.registers.sp + 1);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::INC,
        bytes: 1,
        cycles: 2,
    })
}

/// LD SP,n16
/// Copy the value n16 into register SP.
pub fn load_sp_n16(n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.set_r16(R16::SP, n16);
    cpu.registers.pc += 3;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 3,
    })
}

/// LD [n16],SP
/// Copy SP & $FF at address n16 and SP >> 8 at address n16 + 1.
pub fn load_a16_sp(n16: u16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let sp = cpu.registers.sp;
    let n16 = n16 as usize;
    mem.write(n16, (sp & 0xff) as u8);
    mem.write(n16 + 1, (sp >> 8) as u8);
    cpu.registers.pc += 3;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 5,
    })
}

/// LD HL,SP+e8
/// Add the signed value e8 to SP and copy the result in HL.
pub fn load_hl_sp_e8(e8: i8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    //TODO
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD SP, HL
/// Copy register HL into register SP.
pub fn load_sp_hl(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    cpu.registers.set_r16(R16::SP, hl);
    cpu.registers.pc += 1;
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
pub fn pop_af(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let low = mem.read(cpu.registers.sp as usize);
    cpu.registers.flags.zero = low >> 7 == 1;
    cpu.registers.flags.subtraction = low >> 6 == 1;
    cpu.registers.flags.half_carry = low >> 5 == 1;
    cpu.registers.flags.carry = low >> 4 == 1;
    cpu.registers.set_r16(R16::SP, cpu.registers.sp + 1);
    let high = mem.read(cpu.registers.sp as usize);
    cpu.registers.set_r8(R8::A, high);
    cpu.registers.set_r16(R16::SP, cpu.registers.sp + 1);
    cpu.registers.pc += 1;
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
pub fn pop_r16(r16: R16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    pop_stack(r16, cpu, mem);
    cpu.registers.pc += 1;
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
pub fn push_af(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let mut af = cpu.registers.get_r16(R16::AF);
    af |= (cpu.registers.flags.zero as u16) << 7;
    af |= (cpu.registers.flags.subtraction as u16) << 6;
    af |= (cpu.registers.flags.half_carry as u16) << 5;
    af |= (cpu.registers.flags.carry as u16) << 4;
    push_stack(af, cpu, mem);
    cpu.registers.pc += 1;
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
pub fn push_r16(r16: R16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    push_stack(cpu.registers.get_r16(r16), cpu, mem);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::PUSH,
        bytes: 1,
        cycles: 4,
    })
}

mod tests {
    use crate::instructions::add_a_n8;

    use super::*;

    #[test]
    fn test_add_hl_sp() {
        let mut cpu = Cpu::default();
        add_a_n8(20, &mut cpu).unwrap();
        assert_eq!(cpu.registers.flags.carry, false);
        assert_eq!(cpu.registers.flags.half_carry, false);
        add_hl_sp(&mut cpu).unwrap();
        assert_eq!(cpu.registers.flags.carry, true);
        assert_eq!(cpu.registers.flags.half_carry, true);
    }

    #[test]
    fn test_push_af() {
        let mut cpu = Cpu::default();
        let mut mem = Memory::default();
        push_af(&mut cpu, &mut mem).unwrap();
    }
}
