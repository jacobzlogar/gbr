#![feature(mpmc_channel)]
use cpu::Cpu;
use errors::DecodeError;
use instructions::Instruction;
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

/// A thunk takes a reference to an iterable over the slice of bytes representing an instruction stream,
/// a reference to the Cpu struct so it can read register values, and a reference to the backing memory
pub type Thunk = fn(
    &mut std::slice::Iter<u8>,
    &mut Cpu,
    &mut MemoryMap,
) -> std::result::Result<Instruction, DecodeError>;
//pub type Thunk = fn(&mut std::slice::Iter<u8>) -> std::result::Result<Instruction, DecodeError>;

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
