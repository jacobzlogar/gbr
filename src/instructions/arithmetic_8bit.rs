use crate::{
    Mnemonic,
    cpu::{Cpu, Flag, Register8, Register16},
    memory::MemoryMap,
};

use super::{add_8bit, sub_8bit, Arith8Bit, Instruction, InstructionResult};

/// ADC A,r8
/// Add the value in r8 plus the carry flag to A.
pub fn adc_r8_a(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let carry_flag = cpu.get_flag(Flag::C) as u8;
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = add_8bit(a, r8, Some(carry_flag));
    cpu.set_r8(Register8::A, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADC,
        bytes: 1,
        cycles: 1,
    })
}

/// ADC A,[HL]
/// Add the byte pointed to by HL plus the carry flag to A.
pub fn adc_immed_hl_a(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let carry_flag = cpu.get_flag(Flag::C) as u8;
    let hl = cpu.registers[Register16::HL];
    let a = cpu.get_r8(Register8::A);
    let mem = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = add_8bit(a, mem, Some(carry_flag));
    cpu.set_r8(Register8::A, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADC,
        bytes: 1,
        cycles: 2,
    })
}

/// ADC A,n8
/// Add the value n8 plus the carry flag to A.
pub fn adc_n8_a(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let carry_flag = cpu.get_flag(Flag::C) as u8;
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = add_8bit(a, n8, Some(carry_flag));
    cpu.set_r8(Register8::A, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADC,
        bytes: 2,
        cycles: 2,
    })
}

/// ADD A,r8
/// Add the value in r8 to A.
pub fn add_r8_a(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r8 = cpu.get_r8(r8);
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = add_8bit(a, r8, None);
    cpu.set_r8(Register8::A, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 1,
    })
}

/// ADD A,[HL]
/// Add the byte pointed to by HL to A.
pub fn add_immed_hl_a(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let a = cpu.get_r8(Register8::A);
    let mem = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = add_8bit(a, mem, None);
    cpu.set_r8(Register8::A, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}

/// ADD A,n8
/// Add the value n8 to A.
pub fn add_n8_a(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = add_8bit(a, n8, None);
    cpu.set_r8(Register8::A, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}


/// CP A, r8
/// ComPare the value in A with the value in r8.
/// This subtracts the value in r8 from A and sets flags accordingly, but discards the result.
pub fn cp_a_r8(
    r8: Register8,
    cpu: &mut Cpu
) -> InstructionResult<Instruction> {
    let r8 = cpu.get_r8(r8);
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum: _, flags } = sub_8bit(a, r8, None);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::CP,
        bytes: 1,
        cycles: 1
    })
}

/// CP A, [HL]
/// ComPare the value in A with the byte pointed to by HL.
/// This subtracts the value in r8 from A and sets flags accordingly, but discards the result.
pub fn cp_a_hl(
    cpu: &mut Cpu,
    mem: &mut MemoryMap
) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let hl = cpu.registers[Register16::HL];
    let b = mem.read(hl as usize);
    let Arith8Bit { sum: _, flags } = sub_8bit(a, b, None);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::CP,
        bytes: 1,
        cycles: 1
    })
}

/// CP A, n8
/// ComPare the value in A with the value n8.
/// This subtracts the value n8 from A and sets flags accordingly, but discards the result.
pub fn cp_a_n8(
    n8: u8,
    cpu: &mut Cpu
) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum: _, flags } = sub_8bit(a, n8, None);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::CP,
        bytes: 2,
        cycles: 2
    })
}

/// DEC r8
/// Decrement the value in register r8 by 1.
pub fn dec_r8(
    r8: Register8,
    cpu: &mut Cpu
) -> InstructionResult<Instruction> {
    let reg = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = sub_8bit(reg, 1, None);
    cpu.set_r8(Register8::Flags, flags);
    cpu.set_r8(r8, sum);
    Ok(Instruction {
        mnemonic: Mnemonic::DEC,
        bytes: 1,
        cycles: 1
    })
}

/// DEC [HL]
/// Decrement the byte pointed to by HL by 1.
pub fn dec_hl(
    cpu: &mut Cpu,
    mem: &mut MemoryMap
) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = sub_8bit(byte, 1, None);
    cpu.set_r8(Register8::Flags, flags);
    mem.write(hl as usize, sum);
    Ok(Instruction {
        mnemonic: Mnemonic::DEC,
        bytes: 1,
        cycles: 1
    })
}

/// INC r8
/// Increment the value in register r8 by 1.
pub fn inc_r8(
    r8: Register8,
    cpu: &mut Cpu
) -> InstructionResult<Instruction> {
    let reg = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = add_8bit(reg, 1, None);
    cpu.set_r8(r8, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::INC,
        bytes: 1,
        cycles: 1
    })
}

/// INC [HL]
/// Increment the byte pointed to by HL by 1.
pub fn inc_hl(
    cpu: &mut Cpu,
    mem: &mut MemoryMap
) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = add_8bit(byte, 1, None);
    cpu.set_r8(Register8::Flags, flags);
    mem.write(hl as usize, sum);
    Ok(Instruction {
        mnemonic: Mnemonic::INC,
        bytes: 1,
        cycles: 1
    })
}

/// SBC A, r8
/// Subtract the value in r8 and the carry flag from A.
pub fn sbc_a_r8(
    r8: Register8,
    cpu: &mut Cpu,
) -> InstructionResult<Instruction> {
    let carry_flag = cpu.get_flag(Flag::C) as u8;
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = sub_8bit(a, r8, Some(carry_flag));
    cpu.set_r8(Register8::Flags, flags);
    cpu.set_r8(Register8::A, sum);
    Ok(Instruction {
        mnemonic: Mnemonic::SBC,
        bytes: 1,
        cycles: 1
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_sbc_a_r8() {
        let mut cpu = Cpu::new(vec![]);
        cpu.set_flag(Flag::C, true);
        cpu.set_r8(Register8::A, 1);
        cpu.set_r8(Register8::B, 14);
        let _ = sbc_a_r8(Register8::B, &mut cpu);
        assert_eq!(cpu.get_flag(Flag::H), true);
    }
    #[test]
    fn test_adc_a_r8() {
        let mut cpu = Cpu::new(vec![]);
        cpu.set_flag(Flag::C, true);
        cpu.set_r8(Register8::A, 1);
        cpu.set_r8(Register8::B, 14);
        let _ = adc_r8_a(Register8::B, &mut cpu);
        assert_eq!(cpu.get_flag(Flag::H), true);
    }

    #[test]
    fn test_cp_a_to_r8() {
        let mut cpu = Cpu::new(vec![]);
        cpu.set_r8(Register8::A, 0);
        cpu.set_r8(Register8::B, 255);
        let _ = cp_a_r8(Register8::B, &mut cpu);
        assert_eq!(cpu.get_r8(Register8::Flags), 0x70);
    }

    #[test]
    fn test_dec_r8() {
        let mut cpu = Cpu::new(vec![]);
        cpu.set_r8(Register8::B, 2);
        let _ = dec_r8(Register8::B, &mut cpu);
        println!("{:08b} {:08b}", cpu.get_r8(Register8::B), cpu.get_r8(Register8::Flags));
    }
}
