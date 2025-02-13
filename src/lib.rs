use cpu::Cpu;
use instructions::{Instruction, InstructionResult};
use memory::MemoryMap;

pub mod clock;
pub mod cpu;
pub mod display;
pub mod dma;
pub mod errors;
pub mod instructions;
pub mod io;
pub mod memory;
pub mod registers;
pub mod system;

/// Holds the necessary context for instruction decoding.
pub struct DecodeContext<'a> {
    pub iter: &'a mut std::slice::Iter<'a, u8>,
    pub cpu: &'a mut Cpu,
    pub memory: &'a mut MemoryMap,
}

/// `InstructionFn` defines the function signature for decoding an instruction.
/// Implementors of `InstructionFn` expect `DecodeContext` as a paramter, which holds:
/// - A mutable iterator over a byte slice (`&mut std::slice::Iter<u8>`) to read instruction bytes.
/// - A mutable reference to the `Cpu`, allowing modifications to registers, flags, etc.
/// - A mutable reference to the `MemoryMap`, providing access to system memory.
/// 
/// The function returns a `Result<Instruction, DecodeError>`, where:
/// - `Instruction` represents a successfully decoded instruction.
/// - `DecodeError` indicates a failure in decoding.
/// 
/// # Example
/// 
/// ```rust
/// pub fn load_r8_r8(
///     source: Register8,
///     dest: Register8,
///     cpu: &mut Cpu,
/// ) -> InstructionResult<Instruction> {
///     let source = cpu.get_r8(source);
///     let dest = cpu.get_r8(dest);
///     cpu.set_r8(dest, source);
///     Ok(Instruction {
///         mnemonic: Mnemonic::LD,
///         bytes: 1,
///         cycles: 1,
///     })
/// }
/// ```
pub type InstructionFn = fn(
    &mut DecodeContext
) -> InstructionResult<Instruction>;

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
