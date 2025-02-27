use crate::{
    Mnemonic,
    cpu::{Cpu, R8},
    instructions::R16,
    memory::Memory,
};

use super::{Instruction, InstructionResult};

/// LD r8, r8
/// Storing a register into itself is a no-op; however, some Game Boy emulators interpret LD B,B as a breakpoint, or LD D,D as a debug message
pub fn ld_r8_r8(
    // it would be nice if i could just pass &mut u8's from the dispatch table
    // but then i wouldn't be able to set the lower/higher 8 bits of the destination's 16-bit register
    // maybe there's something clever that can be done here, but maybe doing something clever is a bad idea
    source: R8,
    dest: R8,
    cpu: &mut Cpu,
) -> InstructionResult<Instruction> {
    let src = cpu.registers.get_r8(source);
    cpu.registers.set_r8(dest, src);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 1,
    })
}

/// LD r8, n8
/// Copy the value n8 into register r8.
pub fn ld_r8_n8(r8: R8, n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.set_r8(r8, n8);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 2,
    })
}

/// LD r16, n16
/// Copy the value n16 into register r16.
pub fn ld_r16_n16(r16: R16, n16: u16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    cpu.registers.set_r16(r16, n16);
    cpu.registers.pc += 3;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 3,
    })
}

/// LD [HL], r8
/// Copy the r8 into the byte pointed to by [HL].
pub fn ld_r8_hl(r8: R8, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let r8 = cpu.registers.get_r8(r8);
    mem.write(hl as usize, r8);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD [HL], n8
/// Copy the value n8 into the byte pointed to by HL.
pub fn ld_n8_hl(n8: u8, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    mem.write(hl as usize, n8);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD r8, [HL]
/// Copy the value pointed to by HL into register r8.
pub fn ld_hl_r8(r8: R8, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let byte = mem.read(hl as usize);
    cpu.registers.set_r8(r8, byte);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 2,
        cycles: 3,
    })
}

/// LD [r16],A
/// Copy the value in register A into the byte pointed to by r16.
pub fn ld_a_immed_r16(r16: R16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let a = cpu.registers.a;
    let r16 = cpu.registers.get_r16(r16);
    mem.write(r16 as usize, a as u8);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD A, [n16]
/// Copy the byte at address n16 into register A.
pub fn ld_immed_n16_a(n16: u16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let byte = mem.read(n16 as usize);
    cpu.registers.set_r8(R8::A, byte);
    cpu.registers.pc += 3;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 4,
    })
}

/// LD [n16], A
/// Copy the value in register A into the byte at address n16.
pub fn ld_a_immed_n16(n16: u16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let a = cpu.registers.a;
    mem.write(n16 as usize, a as u8);
    cpu.registers.pc += 3;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 3,
        cycles: 4,
    })
}

/// LDH A, [n16]
/// Copy the byte at address n16 into register A, provided the address is between $FF00 and $FFFF.
pub fn ldh_a_immed_n16(
    n16: u16,
    cpu: &mut Cpu,
    mem: &mut Memory,
) -> InstructionResult<Instruction> {
    let byte = mem.read(n16 as usize);
    if (0xff00..=0xfff).contains(&n16) {
        cpu.registers.set_r8(R8::A, byte);
        cpu.registers.a = byte;
    }
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::LDH,
        bytes: 2,
        cycles: 3,
    })
}

/// LDH [n16], A
/// Copy the value in register A into the byte at address n16, provided the address is between $FF00 and $FFFF.
pub fn ldh_immed_n16_a(
    n16: u16,
    cpu: &mut Cpu,
    mem: &mut Memory,
) -> InstructionResult<Instruction> {
    let a = cpu.registers.a;
    if (0xff00..=0xfff).contains(&n16) {
        mem.write(n16 as usize, a);
    }
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::LDH,
        bytes: 2,
        cycles: 3,
    })
}

/// LDH A, [C]
/// Copy the byte at address $FF00+C into register A.
pub fn ldh_a_c(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let c = cpu.registers.c;
    let byte = mem.read(0xff00 + c as usize);
    cpu.registers.set_r8(R8::A, byte);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LDH,
        bytes: 1,
        cycles: 2,
    })
}

/// LDH [C],A
/// Copy the value in register A into the byte at address $FF00+C.
pub fn ldh_c_a(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let a = cpu.registers.a;
    let c = cpu.registers.c;
    mem.write(0xff00 + c as usize, a);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LDH,
        bytes: 1,
        cycles: 2,
    })
}

/// LD A,[r16]
/// Copy the byte pointed to by r16 into register A.
pub fn ld_immed_r16_a(r16: R16, cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let r16 = cpu.registers.get_r16(r16);
    let immed = mem.read(r16 as usize);
    cpu.registers.set_r8(R8::A, immed);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD [HLI],A
/// Copy the value in register A into the byte pointed by HL and increment HL afterwards.
pub fn ld_a_hli(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let a = cpu.registers.a;
    let lcdc = mem.lcd_control();
    println!("\nloading register a {a:0x} into 0x{hl:0x}\n{lcdc}");
    mem.write(hl as usize, a);
    cpu.registers.set_r16(R16::HL, hl + 1);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD [HLD],A
/// Copy the value in register A into the byte pointed by HL and decrement HL afterwards.
pub fn ld_a_hld(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let a = cpu.registers.a;
    mem.write(hl as usize, a);
    cpu.registers.set_r16(R16::HL, hl - 1);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}
/// LD A,[HLD]
/// Copy the byte pointed to by HL into register A, and decrement HL afterwards.
pub fn ld_hld_a(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let byte = mem.read(hl as usize);
    cpu.registers.set_r8(R8::A, byte);
    cpu.registers.set_r16(R16::HL, hl - 1);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

/// LD A,[HLI]
/// Copy the byte pointed to by HL into register A, and increment HL afterwards.
pub fn ld_hli_a(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let byte = mem.read(hl as usize);
    // println!("loading 0x{byte:0x} into 0x{hl:0x}");
    cpu.registers.set_r8(R8::A, byte);
    cpu.registers.set_r16(R16::HL, hl + 1);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::LD,
        bytes: 1,
        cycles: 2,
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_ld_r8_r8() {
        let mut cpu = Cpu::default();
        ld_r8_r8(R8::A, R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.bc, 0x0113);
        assert_eq!(cpu.registers.b, 0x01);
        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn test_ld_r8_n8() {
        let mut cpu = Cpu::default();
        ld_r8_n8(R8::B, 0x69, &mut cpu).unwrap();
        assert_eq!(cpu.registers.bc, 0x6913);
        assert_eq!(cpu.registers.b, 0x69);
        assert_eq!(cpu.registers.c, 0x13);
    }

    #[test]
    fn test_ld_r16_n16() {
        let mut cpu = Cpu::default();
        let n16 = 0x0420;
        ld_r16_n16(R16::BC, n16, &mut cpu).unwrap();
        assert_eq!(cpu.registers.bc, 0x0420);
        assert_eq!(cpu.registers.b, 0x04);
        assert_eq!(cpu.registers.c, 0x20);
    }
}
