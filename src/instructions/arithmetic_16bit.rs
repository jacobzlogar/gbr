use crate::{
    Mnemonic,
    cpu::{Cpu, Register8, Register16},
};

use super::{Instruction, InstructionResult};

#[derive(Debug)]
pub struct Arith16Bit {
    pub sum: u16,
    pub flags: u8,
}

pub fn add_16bit(a: u16, b: u16, carry_flag: Option<bool>) -> Arith16Bit {
    let carry = match carry_flag {
        Some(num) => num as u16,
        None => 0,
    };
    let half_carry = ((a & 0x00ff) + (b & 0x00ff) & 0x0100) == 0x0100;
    let (sum, carry) = a.overflowing_add(b + carry);
    let mut flags: u8 = 0;
    flags |= ((sum == 0) as u8) << 7;
    flags |= 0 << 6;
    flags |= (half_carry as u8) << 5;
    flags |= (carry as u8) << 4;
    Arith16Bit { sum, flags }
}

pub fn sub_16bit(a: u16, b: u16, carry_flag: Option<bool>) -> Arith16Bit {
    let carry = match carry_flag {
        Some(num) => num as u16,
        None => 0,
    };
    let a_mask = a as i32 & 0x00ff;
    let b_mask = b as i32 & 0x00ff;
    let half_carry = a_mask - b_mask > 0;
    let (sum, carry) = a.overflowing_sub(b - carry);
    let mut flags: u8 = 0;
    flags |= ((sum == 0) as u8) << 7;
    flags |= 0 << 6;
    flags |= (half_carry as u8) << 5;
    flags |= (carry as u8) << 4;
    Arith16Bit { sum, flags }
}

/// ADD HL, r16
/// Add the value in r16 to HL
pub fn add_r16_hl(r16: Register16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r16 = cpu.registers[r16];
    let hl = cpu.registers[Register16::HL];
    let Arith16Bit { sum, flags } = add_16bit(r16, hl, None);
    cpu.set_r16(Register16::HL, sum);
    cpu.flags.set(flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}

/// DEC r16
/// Decrement the value in register r16 by 1.
pub fn dec_r16(r16: Register16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let reg = cpu.registers[r16];
    let Arith16Bit { sum, flags: _ } = sub_16bit(reg, 1, None);
    cpu.set_r16(r16, sum);
    Ok(Instruction {
        mnemonic: Mnemonic::DEC,
        bytes: 1,
        cycles: 2,
    })
}

/// INC r16
/// Increment the value in register r16 by 1.
pub fn inc_r16(r16: Register16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let reg = cpu.registers[r16];
    let Arith16Bit { sum, flags: _ } = add_16bit(reg, 1, None);
    cpu.set_r16(r16, sum);
    Ok(Instruction {
        mnemonic: Mnemonic::INC,
        bytes: 1,
        cycles: 2,
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_add_r16_hl() {
        let mut cpu = Cpu::new(vec![]);
        cpu.set_r16(Register16::BC, 0xfffe);
        cpu.set_r16(Register16::HL, 0x0002);
        let _ = add_r16_hl(Register16::BC, &mut cpu);
        assert_eq!(cpu.registers[Register16::HL], 0);
        // assert_eq!(cpu.get_r8(Register8::Flags), 0xb0);
    }
}
