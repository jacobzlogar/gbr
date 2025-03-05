use std::alloc::System;

use crate::{errors::CartridgeError, memory::Memory};

#[derive(Debug, Clone)]
pub struct Cartridge {
    pub rom: Vec<u8>,
    pub cartridge_type: CartridgeType,
    pub logo: Vec<u8>,
    pub title: String,
    pub cgb_flag: bool,
    pub rom_size: usize,
    pub ram_size: RamSize,
}

pub const ENTRY_POINT_START: usize = 0x0100;
pub const ENTRY_POINT_END: usize = 0x0103;
pub const LOGO_START: usize = 0x0104;
pub const LOGO_END: usize = 0x0133;
pub const TITLE_START: usize = 0x0134;
pub const TITLE_END: usize = 0x0143;
pub const CGB_FLAG: usize = 0x0143; // huh?
pub const CARTRIDGE_TYPE: usize = 0x0147;
pub const ROM_SIZE: usize = 0x0148;
pub const RAM_SIZE: usize = 0x0149;

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Result<Self, CartridgeError> {
        let cartridge_type = CartridgeType::try_from(rom[CARTRIDGE_TYPE])?;
        let title = String::from_utf8_lossy(&rom[TITLE_START..TITLE_END]).to_string();
        let logo = &rom[LOGO_START..LOGO_END].to_vec();
        let cgb_flag = match &rom[CGB_FLAG] {
            0x80 | 0xc0 => true,
            _ => false,
        };
        let rom_size = get_rom_size(rom[ROM_SIZE])?;
        let ram_size = RamSize::try_from(rom[RAM_SIZE])?;
        Ok(Cartridge {
            rom,
            cartridge_type,
            title,
            logo: logo.to_vec(),
            cgb_flag,
            ram_size,
            rom_size,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RamSize {
    Zero,
    Ram8KiB(u8),
    Ram32KiB(u8),
    Ram128KiB(u8),
    Ram64KiB(u8),
}

impl TryFrom<u8> for RamSize {
    type Error = CartridgeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 | 0x01 => Ok(RamSize::Zero),
            0x02 => Ok(Self::Ram8KiB(1)),
            0x03 => Ok(Self::Ram32KiB(4)),
            0x04 => Ok(Self::Ram128KiB(16)),
            0x05 => Ok(Self::Ram64KiB(8)),
            _ => Err(CartridgeError::InvalidRamSize(value)),
        }
    }
}

fn get_rom_size(value: u8) -> Result<usize, CartridgeError> {
    match value {
        0x00 => Ok(2),
        0x01 => Ok(4),
        0x02 => Ok(8),
        0x03 => Ok(16),
        0x04 => Ok(32),
        0x05 => Ok(64),
        0x06 => Ok(128),
        0x07 => Ok(256),
        0x08 => Ok(512),
        0x52 => Ok(72),
        0x53 => Ok(80),
        0x54 => Ok(96),
        _ => Err(CartridgeError::InvalidRomSize(value)),
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum CartridgeType {
    RomOnly,
    MBC1 {
        ram: bool,
        battery: bool,
    },
    MBC2 {
        battery: bool,
    },
    RomRam,
    RomRamBattery,
    MMM01 {
        ram: bool,
        battery: bool,
    },
    MBC3 {
        timer: bool,
        ram: bool,
        battery: bool,
    },
    MBC5 {
        ram: bool,
        rumble: bool,
        battery: bool,
    },
    MBC6,
    MBC7,
    PocketCamera,
    BandaiTama,
    HuC3,
    HuC1,
}

impl TryFrom<u8> for CartridgeType {
    type Error = CartridgeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::RomOnly),
            0x01 => Ok(Self::MBC1 {
                ram: false,
                battery: false,
            }),
            0x02 => Ok(Self::MBC1 {
                ram: true,
                battery: false,
            }),
            0x03 => Ok(Self::MBC1 {
                ram: true,
                battery: true,
            }),
            0x05 => Ok(Self::MBC2 { battery: false }),
            0x06 => Ok(Self::MBC2 { battery: true }),
            0x08 => Ok(Self::RomRam),
            0x09 => Ok(Self::RomRamBattery),
            0x0b => Ok(Self::MMM01 {
                ram: false,
                battery: false,
            }),
            0x0c => Ok(Self::MMM01 {
                ram: true,
                battery: false,
            }),
            0x0d => Ok(Self::MMM01 {
                ram: true,
                battery: true,
            }),
            0x0f => Ok(Self::MBC3 {
                timer: true,
                ram: false,
                battery: true,
            }),
            0x10 => Ok(Self::MBC3 {
                timer: true,
                ram: true,
                battery: true,
            }),
            0x11 => Ok(Self::MBC3 {
                timer: false,
                ram: false,
                battery: false,
            }),
            0x12 => Ok(Self::MBC3 {
                timer: false,
                ram: true,
                battery: false,
            }),
            0x13 => Ok(Self::MBC3 {
                timer: false,
                ram: true,
                battery: true,
            }),
            0x19 => Ok(Self::MBC5 {
                ram: false,
                rumble: false,
                battery: false,
            }),
            0x1a => Ok(Self::MBC5 {
                ram: true,
                rumble: false,
                battery: false,
            }),
            0x1b => Ok(Self::MBC5 {
                ram: false,
                rumble: true,
                battery: true,
            }),
            0x1c => Ok(Self::MBC5 {
                ram: false,
                rumble: true,
                battery: false,
            }),
            0x1d => Ok(Self::MBC5 {
                ram: true,
                rumble: true,
                battery: false,
            }),
            0x1e => Ok(Self::MBC5 {
                ram: true,
                rumble: true,
                battery: true,
            }),
            0x20 => Ok(Self::MBC6),
            0x22 => Ok(Self::MBC7),
            0xfc => Ok(Self::PocketCamera),
            0xfd => Ok(Self::BandaiTama),
            0xfe => Ok(Self::HuC3),
            0xff => Ok(Self::HuC1),
            _ => Err(CartridgeError::InvalidHardware(value)),
        }
    }
}
