use crate::{
    Mnemonic,
    cpu::{Cpu, Flag, Register8},
    instructions::Register16,
    memory::MemoryMap,
};

use super::{Instruction, InstructionResult};

/// LD r8, r8
/// Storing a register into itself is a no-op; however, some Game Boy emulators interpret LD B,B as a breakpoint, or LD D,D as a debug message
pub fn load_r8_r8(
    source: Register8,
    dest: Register8,
    cpu: &mut Cpu,
) -> InstructionResult<Instruction> {
    let source = cpu.get_r8(source);
    cpu.set_r8(dest, source);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 1,
    })
}

/// LD r8, n8
/// Copy the value n8 into register r8.
pub fn load_r8_n8(register: Register8, n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.set_r8(register, n8);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 2,
    })
}

/// LD r16, n16
/// Copy the value n16 into register r16.
pub fn load_r16_n16(
    register: Register16,
    n16: u16,
    cpu: &mut Cpu,
) -> InstructionResult<Instruction> {
    cpu.set_r16(register, n16);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 3,
    })
}

/// LD [HL], r8
/// Copy the value pointed to by HL into register r8.
pub fn load_r8_hl(
    r8: Register8,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let r8 = cpu.get_r8(r8);
    mem.write(hl as usize, r8);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD [HL], n8
/// Copy the value n8 into the byte pointed to by HL.
pub fn load_n8_hl(n8: u8, cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    mem.write(hl as usize, n8);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD r8, [HL]
/// Copy the value pointed to by HL into register r8.
pub fn load_hl_r8(
    r8: Register8,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    cpu.set_r8(r8, byte);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD [r16],A
/// Copy the value in register A into the byte pointed to by r16.
pub fn load_a_immed_r16(
    register: Register16,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let r16 = cpu.registers[register];
    mem.write(r16 as usize, a as u8);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD [n16], A
/// Copy the value in register A into the byte at address n16.
pub fn load_a_immed_n16(
    n16: u16,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    mem.write(n16 as usize, a as u8);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 4,
    })
}

/// LDH A, [n16]
/// Copy the byte at address n16 into register A, provided the address is between $FF00 and $FFFF.
/// TODO: fix this
pub fn loadh_a_immed_n16(
    n16: u16,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
    let byte = mem.read(n16 as usize);
    // if byte >= 0xff00 && byte <= 0xffff {
    //     cpu.set_r8(Register8::A, byte);
    // }
    Ok(Instruction {
        mnemonic: Mnemonic::LDH,
        bytes: 2,
        cycles: 3,
    })
}

/// LDH [n16], A
/// Copy the value in register A into the byte at address n16, provided the address is between $FF00 and $FFFF.
/// TODO: fix this
pub fn loadh_immed_n16_a(
    n16: u16,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
    let byte = mem.read(n16 as usize);
    // if byte >= 0xff00 && byte <= 0xffff {
    //     cpu.set_r8(Register8::A, byte);
    // }
    Ok(Instruction {
        mnemonic: Mnemonic::LDH,
        bytes: 2,
        cycles: 3,
    })
}

/// LDH [C],A
/// Copy the value in register A into the byte at address $FF00+C.
pub fn loadh_a_c(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let c = cpu.get_r8(Register8::C);
    mem.write(0xff00 + c as usize, a);
    Ok(Instruction {
        mnemonic: Mnemonic::LDH,
        bytes: 1,
        cycles: 2,
    })
}

/// LD A,[r16]
/// Copy the byte pointed to by r16 into register A.
pub fn load_immed_r16_a(
    r16: Register16,
    cpu: &mut Cpu,
    mem: &mut MemoryMap,
) -> InstructionResult<Instruction> {
    let r16 = cpu.registers[r16];
    let immed = mem.read(r16 as usize);
    cpu.set_r8(Register8::A, immed);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD [HLI],A
/// Copy the value in register A into the byte pointed by HL and increment HL afterwards.
pub fn load_a_hli(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let a = cpu.get_r8(Register8::A);
    mem.write(hl as usize, a);
    cpu.set_r16(Register16::HL, hl + 1);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD [HLD],A
/// Copy the value in register A into the byte pointed by HL and decrement HL afterwards.
pub fn load_a_hld(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let a = cpu.get_r8(Register8::A);
    mem.write(hl as usize, a);
    cpu.set_r16(Register16::HL, hl - 1);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}
/// LD A,[HLD]
/// Copy the byte pointed to by HL into register A, and decrement HL afterwards.
pub fn load_hld_a(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    cpu.set_r8(Register8::A, byte);
    cpu.set_r16(Register16::HL, hl - 1);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD A,[HLI]
/// Copy the byte pointed to by HL into register A, and increment HL afterwards.
pub fn load_hli_a(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    cpu.set_r8(Register8::A, byte);
    cpu.set_r16(Register16::HL, hl + 1);
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}
