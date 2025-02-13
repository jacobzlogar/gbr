use crate::{
    Mnemonic,
    cpu::{Cpu, Register8, Register16},
    memory::MemoryMap,
};

use super::{Instruction, InstructionResult};

#[derive(Debug)]
pub struct Arith8Bit {
    pub sum: u8,
    pub flags: u8,
}

pub fn add_8bit(a: u8, b: u8, carry_flag: Option<bool>) -> Arith8Bit {
    let carry = match carry_flag {
        Some(num) => num as u8,
        None => 0,
    };
    // https://stackoverflow.com/a/57822729 thanks
    let b = b + carry;
    let half_carry = ((a & 0x0f) + (b & 0x0f) & 0x10) == 0x10;
    let (sum, carry) = a.overflowing_add(b);
    let mut flags: u8 = 0;
    // set the zero flag if sum == 0
    flags |= ((sum == 0) as u8) << 7;
    // set the subtraction flag to false
    flags |= 0 << 6;
    // set the half carry flag
    flags |= (half_carry as u8) << 5;
    // set the carry flag
    flags |= (carry as u8) << 4;
    Arith8Bit { sum, flags }
}

pub fn sub_8bit(a: u8, b: u8, carry_flag: Option<bool>) -> Arith8Bit {
    let carry = match carry_flag {
        Some(num) => num as u8,
        None => 0,
    };
    let a_mask = a as i16 & 0x0f;
    let b_mask = b as i16 & 0x0f;
    let half_carry = a_mask - b_mask < 0;
    let (sum, _) = a.overflowing_sub(b - carry);
    let carry = b >= sum;
    let mut flags: u8 = 0;
    flags |= ((sum == 0) as u8) << 7;
    flags |= 1 << 6;
    flags |= (half_carry as u8) << 5;
    flags |= (carry as u8) << 4;
    Arith8Bit { sum, flags }
}

/// ADC A,r8
/// Add the value in r8 plus the carry flag to A.
pub fn adc_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = add_8bit(a, r8, Some(cpu.flags.carry));
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADC,
        bytes: 1,
        cycles: 1,
    })
}

/// ADC A,[HL]
/// Add the byte pointed to by HL plus the carry flag to A.
pub fn adc_a_immed_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let a = cpu.get_r8(Register8::A);
    let mem = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = add_8bit(a, mem, Some(cpu.flags.carry));
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADC,
        bytes: 1,
        cycles: 2,
    })
}

/// ADC A,n8
/// Add the value n8 plus the carry flag to A.
pub fn adc_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let carry_flag = cpu.flags.carry;
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = add_8bit(a, n8, Some(carry_flag));
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADC,
        bytes: 2,
        cycles: 2,
    })
}

/// ADD A,r8
/// Add the value in r8 to A.
pub fn add_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r8 = cpu.get_r8(r8);
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = add_8bit(a, r8, None);
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 1,
    })
}

/// ADD A,[HL]
/// Add the byte pointed to by HL to A.
pub fn add_a_immed_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let a = cpu.get_r8(Register8::A);
    let mem = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = add_8bit(a, mem, None);
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}

/// ADD A,n8
/// Add the value n8 to A.
pub fn add_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = add_8bit(a, n8, None);
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}

/// CP A, r8
/// ComPare the value in A with the value in r8.
/// This subtracts the value in r8 from A and sets flags accordingly, but discards the result.
pub fn cp_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r8 = cpu.get_r8(r8);
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum: _, flags } = sub_8bit(a, r8, None);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::CP,
        bytes: 1,
        cycles: 1,
    })
}

/// CP A, [HL]
/// ComPare the value in A with the byte pointed to by HL.
/// This subtracts the value in r8 from A and sets flags accordingly, but discards the result.
pub fn cp_a_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let hl = cpu.registers[Register16::HL];
    let b = mem.read(hl as usize);
    let Arith8Bit { sum: _, flags } = sub_8bit(a, b, None);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::CP,
        bytes: 1,
        cycles: 1,
    })
}

/// CP A, n8
/// ComPare the value in A with the value n8.
/// This subtracts the value n8 from A and sets flags accordingly, but discards the result.
pub fn cp_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum: _, flags } = sub_8bit(a, n8, None);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::CP,
        bytes: 2,
        cycles: 2,
    })
}

/// DEC r8
/// Decrement the value in register r8 by 1.
pub fn dec_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let reg = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = sub_8bit(reg, 1, None);
    cpu.set_r8(r8, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::DEC,
        bytes: 1,
        cycles: 1,
    })
}

/// DEC [HL]
/// Decrement the byte pointed to by HL by 1.
pub fn dec_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = sub_8bit(byte, 1, None);
    mem.write(hl as usize, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::DEC,
        bytes: 1,
        cycles: 1,
    })
}

/// INC r8
/// Increment the value in register r8 by 1.
pub fn inc_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let reg = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = add_8bit(reg, 1, None);
    cpu.set_r8(r8, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::INC,
        bytes: 1,
        cycles: 1,
    })
}

/// INC [HL]
/// Increment the byte pointed to by HL by 1.
pub fn inc_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = add_8bit(byte, 1, None);
    mem.write(hl as usize, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::INC,
        bytes: 1,
        cycles: 1,
    })
}

/// SBC A, r8
/// Subtract the value in r8 and the carry flag from A.
pub fn sbc_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let carry_flag = cpu.flags.carry;
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = sub_8bit(a, r8, Some(carry_flag));
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::SBC,
        bytes: 1,
        cycles: 1,
    })
}

/// SBC A, [HL]
/// Subtract the byte pointed to by HL and the carry flag from A.
pub fn sbc_a_immed_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let carry_flag = cpu.flags.carry;
    let a = cpu.get_r8(Register8::A);
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = sub_8bit(a, byte, Some(carry_flag));
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::SBC,
        bytes: 1,
        cycles: 2,
    })
}

/// SBC A, n8
/// Subtract the value n8 and the carry flag from A.
pub fn sbc_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let carry_flag = cpu.flags.carry;
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = sub_8bit(a, n8, Some(carry_flag));
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::SBC,
        bytes: 2,
        cycles: 2,
    })
}

/// SUB A, r8
/// Subtract the value in r8 from A.
pub fn sub_a_r8(r8: Register8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let r8 = cpu.get_r8(r8);
    let Arith8Bit { sum, flags } = sub_8bit(a, r8, None);
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::SUB,
        bytes: 1,
        cycles: 1,
    })
}

/// SUB A, [HL]
/// Subtract the byte pointed to by HL from A.
pub fn sub_a_immed_hl(cpu: &mut Cpu, mem: &mut MemoryMap) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let hl = cpu.registers[Register16::HL];
    let byte = mem.read(hl as usize);
    let Arith8Bit { sum, flags } = sub_8bit(a, byte, None);
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::SUB,
        bytes: 1,
        cycles: 2,
    })
}

/// SUB A,n8
/// Subtract the value n8 from A.
pub fn sub_a_n8(n8: u8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.get_r8(Register8::A);
    let Arith8Bit { sum, flags } = sub_8bit(a, n8, None);
    cpu.set_r8(Register8::A, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::SUB,
        bytes: 2,
        cycles: 2,
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_adc_a_r8() {}

    #[test]
    fn test_sbc_a_r8() {}
    #[test]
    fn test_cp_a_to_r8() {}

    #[test]
    fn test_dec_r8() {}
}
