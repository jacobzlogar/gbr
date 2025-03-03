#![allow(warnings)]
use std::io::Write;

use crate::errors::DecodeError;
use cpu::Cpu;
use instructions::{Instruction, InstructionResult};
use memory::Memory;

pub mod apu;
pub mod cartridge;
pub mod clock;
pub mod cpu;
pub mod display;
pub mod errors;
pub mod instructions;
pub mod interrupts;
pub mod io;
pub mod memory;
pub mod system;

/// Holds the necessary context for instruction decoding.
pub struct DecodeContext<'a> {
    pub iter: std::slice::Iter<'a, u8>,
    pub cpu: &'a mut Cpu,
    pub memory: &'a mut Memory,
}

/// `InstructionFn` defines the function signature for decoding an instruction.
/// Implementors of `InstructionFn` expect `DecodeContext` as a paramter, which holds:
/// - A mutable iterator over a byte slice (`&mut std::slice::Iter<u8>`) to read instruction bytes.
/// - A mutable reference to the `Cpu`, allowing modifications to registers, flags, etc.
/// - A mutable reference to the `Memory`, providing access to system memory.
///
/// The function returns a `Result<Instruction, DecodeError>`, where:
/// - `Instruction` represents a successfully decoded instruction.
/// - `DecodeError` indicates a failure in decoding.
///
/// # Example
///
/// ```rust
/// use gbr::{
///     Mnemonic,
///     cpu::{R8, Cpu},
///     instructions::{Instruction, InstructionResult}
/// };
/// pub fn load_r8_r8(
///     source: R8,
///     dest: R8,
///     cpu: &mut Cpu,
/// ) -> InstructionResult<Instruction> {
///     let r8 = cpu.registers.get_r8(source);
///     cpu.registers.set_r8(dest, r8);
///     Ok(Instruction {
///         mnemonic: Mnemonic::LD,
///         bytes: 1,
///         cycles: 1,
///     })
/// }
/// ```

// maybe i should define a contract for functions that decode instructions,
// i.e: functions in the dispatch table take different parts of `ctx` as parameters, i think they should always take all of `DecodeContext`
pub type DecodeFn = fn(&mut DecodeContext) -> InstructionResult<Instruction>;

#[derive(Debug, PartialEq, Eq)]
pub enum Mnemonic {
    PREFIX,
    LD,
    LDH,
    ADC,
    ADD,
    CP,
    DEC,
    INC,
    SBC,
    SUB,
    AND,
    CPL,
    OR,
    XOR,
    BIT,
    RES,
    SET,
    RL,
    RLA,
    RLC,
    RLCA,
    RR,
    RRA,
    RRC,
    RRCA,
    SLA,
    SRA,
    SRL,
    SWAP,
    CALL,
    JP,
    JR,
    RET,
    RETI,
    RST,
    CCF,
    SCF,
    POP,
    PUSH,
    DI,
    EI,
    HALT,
    DAA,
    NOP,
    STOP,
}

pub fn extract_bytes(value: u16) -> (u8, u8) {
    let lsb = (value & 0xff) as u8;
    let msb = (value >> 8) as u8;
    (msb, lsb)
}

pub fn get_i8(iter: &mut std::slice::Iter<u8>) -> InstructionResult<i8> {
    let num = *iter.next().ok_or(DecodeError::MissingDataByte)?;
    Ok(num as i8)
}

pub fn get_u8(iter: &mut std::slice::Iter<u8>) -> InstructionResult<u8> {
    Ok(*iter.next().ok_or(DecodeError::MissingDataByte)?)
}

pub fn get_i16(iter: &mut std::slice::Iter<u8>) -> InstructionResult<i16> {
    let n16 = i16::from_le_bytes([
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
    ]);
    Ok(n16)
}

pub fn get_u16(iter: &mut std::slice::Iter<u8>) -> InstructionResult<u16> {
    let n16 = u16::from_le_bytes([
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
        *iter.next().ok_or(DecodeError::MissingDataByte)?,
    ]);
    Ok(n16)
}

/// Helper that creates .ppm images to help debug tile/tile maps
pub fn dump_tiles(tiles: &[u8], width: u16, height: u16) {
    let mut file = std::fs::File::create(format!("{}/test.ppm", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let header = format!("P3\n{} {}\n255\n", &width, &height);
    let header = header.as_bytes();
    file.write(header);
    for i in 0..height {
        for j in 0..width {
            let pixel = tiles[(i * j) as usize];
            let pixel = format!("{} {} {}\n", pixel, pixel, pixel);
            file.write(pixel.as_bytes());
        }
    }
}
