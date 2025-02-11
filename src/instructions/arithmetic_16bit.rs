use crate::{
    Mnemonic,
    cpu::{Cpu, Register8, Register16},
};

use super::{Arith16Bit, Instruction, InstructionResult, add_16bit};

/// ADD HL, r16
/// Add the value in r16 to HL
pub fn add_r16_to_hl(r16: Register16, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let r16 = cpu.registers[r16];
    let hl = cpu.registers[Register16::HL];
    let Arith16Bit { sum, flags } = add_16bit(r16, hl, None);
    cpu.set_r16(Register16::HL, sum);
    cpu.set_r8(Register8::Flags, flags);
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}

pub fn dec_r16(r16: Register16, cpu: &mut  Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
        bytes: 1,
        cycles: 2,
    })
}
pub fn inc_r16(r16: Register16, cpu: &mut  Cpu) -> InstructionResult<Instruction> {
    Ok(Instruction {
        mnemonic: Mnemonic::ADD,
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
        let _ = add_r16_to_hl(Register16::BC, &mut cpu);
        assert_eq!(cpu.registers[Register16::HL], 0);
        assert_eq!(cpu.get_r8(Register8::Flags), 0xb0);
    }
}
