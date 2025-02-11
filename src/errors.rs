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
    Contention,
}

impl std::error::Error for SystemError {}

impl std::fmt::Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
