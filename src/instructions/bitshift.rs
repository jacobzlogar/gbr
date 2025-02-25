use crate::{
    Cpu, Mnemonic,
    cpu::{Flags, R8, R16},
    memory::Memory,
};

use super::{Instruction, InstructionResult};

/// RL r8
/// Rotate bits in register r8 left, through the carry flag.
pub fn rl_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let reg = cpu.registers.get_r8(r8);
    let new_carry = (reg >> 7) & 1;
    let old_carry = cpu.registers.flags.carry as u8 & 1;
    let shifted = ((reg << 1) & 0xff) + old_carry;
    cpu.registers.flags.zero = shifted == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = new_carry == 1;
    cpu.registers.set_r8(r8, shifted);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RL,
        bytes: 2,
        cycles: 2,
    })
}

/// RL [HL]
/// Rotate the byte pointed to by HL left, through the carry flag.
pub fn rl_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let byte = mem.read(hl as usize);
    let new_carry = (byte >> 7) & 1;
    let old_carry = cpu.registers.flags.carry as u8 & 1;
    let shifted = ((byte << 1) & 0xff) + old_carry;
    cpu.registers.flags.zero = shifted == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = new_carry == 1;
    mem.write(hl as usize, shifted);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RL,
        bytes: 2,
        cycles: 4,
    })
}

/// RLA
/// Rotate register A left, through the carry flag.
pub fn rla(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let a = cpu.registers.a;
    let new_carry = (a >> 7) & 1;
    let old_carry = cpu.registers.flags.carry as u8 & 1;
    let shifted = ((a << 1) & 0xff) + old_carry;
    cpu.registers.flags.clear();
    cpu.registers.flags.carry = new_carry == 1;
    cpu.registers.a = shifted;
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::RLA,
        bytes: 1,
        cycles: 1,
    })
}

/// RLC r8
/// Rotate register r8 left.
/// MSB of r8 becomes carry flag
/// MSB becomes LSB of r8
/// the rest of the bits in r8 are shifted left
///
/// ┏━ Flags ━┓   ┏━━━━━━━ r8 ━━━━━━┓
/// ┃    C  ←╂─┬─╂─ b7 ← ... ← b0←╂
/// ┗━━━━━━━━━┛ │ ┗━━━━━━━━━━━━━━━━━┛ │
///             └─────────────────────┘
pub fn rlc_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    // extract MSB of r8
    let msb = (reg & 0x80) >> 7;
    // shift r8
    reg <<= 1;
    // swap LSB with MSB
    reg |= msb << 0;
    cpu.registers.flags.zero = reg == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    // carry flag is set to MSB of r8
    cpu.registers.flags.carry = msb == 1;
    cpu.registers.set_r8(r8, reg);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RLC,
        bytes: 2,
        cycles: 2,
    })
}

/// RLC [HL]
/// Rotate the byte pointed to by HL left
/// ┏━ Flags ━┓   ┏━━━━━━ [HL] ━━━━━┓
/// ┃    C  ←╂─┬─╂─ b7 ← ... ←b0<--╂
/// ┗━━━━━━━━━┛ │ ┗━━━━━━━━━━━━━━━━━┛ │
///             └─────────────────────┘
pub fn rlc_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    let msb = (byte & 0x80) >> 7;
    // shift byte
    byte <<= 1;
    // swap LSB with MSB
    byte |= msb << 0;
    cpu.registers.flags.zero = byte == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    // carry flag is updated to MSB of r8
    cpu.registers.flags.carry = msb == 1;
    mem.write(hl as usize, byte);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RLC,
        bytes: 2,
        cycles: 4,
    })
}

/// RLCA
/// Rotate register A left.
pub fn rlca(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut a = cpu.registers.a;
    // extract MSB of A
    let msb = (a & 0x80) >> 7;
    // shift A
    a <<= 1;
    // swap LSB with MSB
    a |= msb << 0;
    cpu.registers.flags.zero = false;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    // carry flag is set to MSB of r8
    cpu.registers.flags.carry = msb == 1;
    cpu.registers.set_r8(R8::A, a);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::RLCA,
        bytes: 1,
        cycles: 1,
    })
}

/// Rotate register r8 right, through the carry flag.
///
///   ┏━━━━━━━ r8 ━━━━━━┓ ┏━ Flags ━┓
/// ┌─╂→ b7 .... → b0─╂─╂→   C  ─╂─┐
/// │ ┗━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━┛ │
/// └─────────────────────────────────┘
pub fn rr_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    let carry = cpu.registers.flags.carry as u8;
    // extract LSB
    let lsb = reg & 1;
    // shift r8
    reg >>= 1;
    // put the carry flag in r8 MSB
    reg |= carry << 7;
    cpu.registers.flags.zero = reg == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    // put r8 LSB into carry flag
    cpu.registers.flags.carry = lsb == 1;
    cpu.registers.set_r8(r8, reg);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RR,
        bytes: 2,
        cycles: 2,
    })
}

/// Rotate the byte pointed to by HL right, through the carry flag.
/// Flags are updated the same way as RR, R8
pub fn rr_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    let carry = cpu.registers.flags.carry as u8;
    let lsb = byte & 1;
    byte >>= 1;
    byte |= carry << 7;
    cpu.registers.flags.zero = byte == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    mem.write(hl as usize, byte);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RR,
        bytes: 2,
        cycles: 4,
    })
}

/// RRA
/// Rotate register A right, through the carry flag.
pub fn rra(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut a = cpu.registers.a;
    let carry = cpu.registers.flags.carry as u8;
    let lsb = a & 1;
    a >>= 1;
    a |= carry << 7;
    cpu.registers.flags.zero = false;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    cpu.registers.set_r8(R8::A, a);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::RRA,
        bytes: 1,
        cycles: 1,
    })
}

/// RRC r8
/// Rotate register r8 right.
pub fn rrc_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    let lsb = reg & 1;
    reg >>= 1;
    // LSB becomes MSB
    reg |= lsb << 7;
    cpu.registers.flags.zero = reg == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    cpu.registers.set_r8(r8, reg);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RRC,
        bytes: 2,
        cycles: 2,
    })
}

/// RRC [HL]
/// Rotate the byte pointed to by HL right.
pub fn rrc_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    // Extract LSB
    let lsb = byte & 1;
    // rotate right
    byte >>= 1;
    // LSB becomes MSB
    byte |= lsb << 7;
    cpu.registers.flags.zero = byte == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    mem.write(hl as usize, byte);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::RRC,
        bytes: 2,
        cycles: 4,
    })
}

/// RRCA
/// Rotate register A right.
pub fn rrca(cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut a = cpu.registers.a;
    let lsb = a & 1;
    a >>= 1;
    a |= lsb << 7;
    cpu.registers.flags.zero = false;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    cpu.registers.set_r8(R8::A, a);
    cpu.registers.pc += 1;
    Ok(Instruction {
        mnemonic: Mnemonic::RRCA,
        bytes: 1,
        cycles: 1,
    })
}

/// SLA r8
/// Shift Left Arithmetically register r8.
pub fn sla_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    reg <<= 1;
    let msb = (reg & 0x80) >> 7;
    cpu.registers.flags.zero = reg == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = msb == 1;
    cpu.registers.set_r8(r8, reg);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SLA,
        bytes: 2,
        cycles: 2,
    })
}

/// SLA [HL]
/// Shift Left Arithmetically the byte pointed to by HL.
pub fn sla_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    byte <<= 1;
    let msb = (byte & 0x80) >> 7;
    cpu.registers.flags.zero = byte == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = msb == 1;
    mem.write(hl as usize, byte);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SLA,
        bytes: 2,
        cycles: 4,
    })
}

/// SRA r8
/// Shift Right Arithmetically register r8 (bit 7 of r8 is unchanged).
pub fn sra_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    let msb = (reg & 0x80) >> 7;
    let lsb = reg & 1;
    reg >>= 1;
    // put MSB back into MSB(lol)
    reg |= msb << 7;
    cpu.registers.flags.zero = reg == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    cpu.registers.set_r8(r8, reg);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SRA,
        bytes: 2,
        cycles: 2,
    })
}

/// SRA [HL]
/// Shift Right Arithmetically the byte pointed to by HL (bit 7 of the byte pointed to by HL is unchanged)
pub fn sra_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    let msb = (byte & 0x80) >> 7;
    let lsb = byte & 1;
    byte >>= 1;
    byte |= msb << 7;
    cpu.registers.flags.zero = byte == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    mem.write(hl as usize, byte);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SRA,
        bytes: 2,
        cycles: 4,
    })
}

/// SRL r8
/// Shift Right Logically register r8.
pub fn srl_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    let lsb = reg & 1;
    reg >>= 1;
    cpu.registers.flags.zero = reg == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    cpu.registers.set_r8(r8, reg);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SRL,
        bytes: 2,
        cycles: 2,
    })
}

/// SRL [HL]
/// Shift Right Logically the byte pointed to by HL.
pub fn srl_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    byte >>= 1;
    let lsb = byte & 1;
    cpu.registers.flags.zero = byte == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = lsb == 1;
    mem.write(hl as usize, byte);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SRL,
        bytes: 2,
        cycles: 4,
    })
}

/// SWAP r8
/// Swap the upper 4 bits in register r8 and the lower 4 ones.
pub fn swap_r8(r8: R8, cpu: &mut Cpu) -> InstructionResult<Instruction> {
    let mut reg = cpu.registers.get_r8(r8);
    reg = (reg << 4) | (reg >> 4);
    cpu.registers.flags.zero = reg == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = false;
    cpu.registers.set_r8(r8, reg & 0xff);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SWAP,
        bytes: 2,
        cycles: 2,
    })
}

/// SWAP [HL]
/// Swap the upper 4 bits in the byte pointed by HL and the lower 4 ones.
pub fn swap_hl(cpu: &mut Cpu, mem: &mut Memory) -> InstructionResult<Instruction> {
    let hl = cpu.registers.hl;
    let mut byte = mem.read(hl as usize);
    byte = (byte << 4) | (byte >> 4);
    cpu.registers.flags.zero = byte == 0;
    cpu.registers.flags.subtraction = false;
    cpu.registers.flags.half_carry = false;
    cpu.registers.flags.carry = false;
    mem.write(hl as usize, byte & 0xff);
    cpu.registers.pc += 2;
    Ok(Instruction {
        mnemonic: Mnemonic::SWAP,
        bytes: 2,
        cycles: 2,
    })
}

mod tests {
    use cartridge::Cartridge;
    use cpu::Flags;

    use super::*;
    use crate::*;

    #[test]
    fn test_rl_r8() {
        let mut cpu = Cpu::default();
        cpu.registers.b = 254;
        rl_r8(R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.b, 0xfd);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }

    #[test]
    fn test_rl_hl() {
        let mut mem = Memory::new(Cartridge::new(vec![0; 0xffff]).unwrap());
        let mut cpu = Cpu::default();
        cpu.registers.hl = 0x420;
        let hl = cpu.registers.hl;
        mem.write(hl as usize, 0xfe);
        rl_hl(&mut cpu, &mut mem).unwrap();
        assert_eq!(mem.read(hl as usize), 0xfd);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }

    #[test]
    fn test_rla() {
        let mut cpu = Cpu::default();
        cpu.registers.set_r8(R8::A, 254);
        rla(&mut cpu).unwrap();
        assert_eq!(cpu.registers.a, 0xfd);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }

    #[test]
    fn test_rlc_r8() {
        let mut cpu = Cpu::default();
        cpu.registers.set_r8(R8::B, 0x7f);
        rlc_r8(R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.get_r8(R8::B), 0xfe);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }

    #[test]
    fn test_rlc_hl() {
        let mut mem = Memory::new(Cartridge::new(vec![0; 0xffff]).unwrap());
        let mut cpu = Cpu::default();
        cpu.registers.hl = 0x420;
        let hl = cpu.registers.hl;
        mem.write(hl as usize, 0x7f);
        rlc_hl(&mut cpu, &mut mem).unwrap();
        let byte = mem.read(hl as usize);
        assert_eq!(byte, 0xfe);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });

        mem.write(hl as usize, 0xff);
        rlc_hl(&mut cpu, &mut mem).unwrap();
        let byte = mem.read(hl as usize);
        assert_eq!(byte, 0xff);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }

    #[test]
    fn test_rlca() {
        let mut cpu = Cpu::default();
        cpu.registers.a = 0x7f;
        rlca(&mut cpu).unwrap();
        assert_eq!(cpu.registers.a, 0xfe);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }

    #[test]
    fn test_rr_r8() {
        let mut cpu = Cpu::default();
        cpu.registers.b = 0x3f;
        rr_r8(R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.b, 0x9f);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }

    #[test]
    fn test_rr_hl() {
        let mut mem = Memory::new(Cartridge::new(vec![0; 0xffff]).unwrap());
        let mut cpu = Cpu::default();
        cpu.registers.hl = 0x420;
        let hl = cpu.registers.hl;
        mem.write(hl as usize, 0x3f);
        rr_hl(&mut cpu, &mut mem).unwrap();
        let byte = mem.read(hl as usize);
        assert_eq!(byte, 0x9f);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }

    #[test]
    fn test_rra() {
        let mut cpu = Cpu::default();
        cpu.registers.a = 0x3f;
        rra(&mut cpu).unwrap();
        assert_eq!(cpu.registers.a, 0x9f);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }
    #[test]
    fn test_sla_r8() {
        let mut cpu = Cpu::default();
        cpu.registers.b = 0x3f;
        sla_r8(R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.b, 0x7e);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }
    #[test]
    fn test_sla_hl() {
        let mut mem = Memory::new(Cartridge::new(vec![0; 0xffff]).unwrap());
        let mut cpu = Cpu::default();
        cpu.registers.hl = 0x420;
        let hl = cpu.registers.hl;
        mem.write(hl as usize, 0x3f);
        sla_hl(&mut cpu, &mut mem).unwrap();
        let byte = mem.read(hl as usize);
        assert_eq!(byte, 0x7e);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }
    #[test]
    fn test_sra_r8() {
        let mut cpu = Cpu::default();
        cpu.registers.b = 0x81;
        sra_r8(R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.b, 0xc0);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }
    #[test]
    fn test_sra_hl() {
        let mut mem = Memory::new(Cartridge::new(vec![0; 0xffff]).unwrap());
        let mut cpu = Cpu::default();
        cpu.registers.hl = 0x420;
        let hl = cpu.registers.hl;
        mem.write(hl as usize, 0x81);
        sra_hl(&mut cpu, &mut mem).unwrap();
        assert_eq!(mem.read(hl as usize), 0xc0);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: true
        });
    }
    #[test]
    fn test_srl_r8() {
        let mut cpu = Cpu::default();
        cpu.registers.b = 0x80;
        srl_r8(R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.b, 0x40);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }
    #[test]
    fn test_srl_hl() {
        let mut mem = Memory::new(Cartridge::new(vec![0; 0xffff]).unwrap());
        let mut cpu = Cpu::default();
        cpu.registers.hl = 0x420;
        let hl = cpu.registers.hl;
        mem.write(hl as usize, 0x80);
        srl_hl(&mut cpu, &mut mem).unwrap();
        assert_eq!(mem.read(hl as usize), 0x40);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }
    #[test]
    fn test_swap_r8() {
        let mut cpu = Cpu::default();
        cpu.registers.b = 0xf0;
        swap_r8(R8::B, &mut cpu).unwrap();
        assert_eq!(cpu.registers.b, 0xf);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }

    #[test]
    fn test_swap_hl() {
        let mut mem = Memory::new(Cartridge::new(vec![0; 0xffff]).unwrap());
        let mut cpu = Cpu::default();
        cpu.registers.hl = 0x420;
        let hl = cpu.registers.hl;
        mem.write(hl as usize, 0xf0);
        swap_hl(&mut cpu, &mut mem).unwrap();
        assert_eq!(mem.read(hl as usize), 0xf);
        assert_eq!(cpu.registers.flags, Flags {
            zero: false,
            subtraction: false,
            half_carry: false,
            carry: false
        });
    }
}
