use crate::interrupts::Interrupt;

#[derive(Debug)]
pub enum DecodeError {
    InvalidOpcodeByte(u8),
    MissingDataByte,
    MissingOffsetByte,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDataByte => write!(f, "Missing Data Byte"),
            Self::MissingOffsetByte => write!(f, "Missing Offset Byte"),
            Self::InvalidOpcodeByte(byte) => write!(f, "No Opcode byte at: {byte}"),
        }
    }
}

pub enum JoypadError {
    InvalidRegisterValue(u8, String),
}

impl std::fmt::Display for JoypadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRegisterValue(reg, bank) => {
                write!(f, "Invalid value {reg} for bank: {bank}")
            }
        }
    }
}

#[derive(Debug)]
pub enum SystemError {
    InterruptHandlerError(Interrupt, u16),
    TimerControlError,
    CartridgeError,
}

impl std::error::Error for SystemError {}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InterruptHandlerError(interrupt, handler) => {
                write!(f, "Int handler for {interrupt:?} at 0x{handler:0x} failed")
            }
            Self::TimerControlError => {
                write!(f, "Failed to read cartridge")
            }
            Self::CartridgeError => {
                write!(f, "Failed to read cartridge")
            }
        }
    }
}

#[derive(Debug)]
pub enum CpuError {
    MissingOpcodeByte,
    NoCycles,
}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub enum CartridgeError {
    InvalidHardware(u8),
    InvalidRamSize(u8),
    InvalidRomSize(u8),
}

impl std::error::Error for CartridgeError {}

impl std::fmt::Display for CartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHardware(byte) => {
                write!(f, "No hardware mapping for: {byte}")
            }
            Self::InvalidRamSize(byte) => {
                write!(f, "No ram size mapping for: {byte}")
            }
            Self::InvalidRomSize(byte) => {
                write!(f, "No rom size mapping for: {byte}")
            }
        }
    }
}
