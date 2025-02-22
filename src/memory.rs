use interrupts::Interrupt;
use regions::*;
use registers::*;

use crate::{cartridge::CartridgeType, errors::SystemError};

pub mod interrupts {
    #[derive(Debug)]
    pub enum Interrupt {
        Joypad,
        Serial,
        Timer,
        Stat,
        VBlank,
    }

    impl Interrupt {
        pub fn get_interrupt(value: &u8) -> Option<Self> {
            match value {
                0x04 => Some(Interrupt::Joypad),
                0x03 => Some(Interrupt::Serial),
                0x02 => Some(Interrupt::Timer),
                0x01 => Some(Interrupt::Stat),
                0x00 => Some(Interrupt::VBlank),
                _ => None,
            }
        }
    }
    pub const TIMER: u8 = 0x02;
    // pub const VBLANK: u8 = 0x00;
    // pub const LCD: u8 = 0x02;
    // pub const TIMER: u8 = 0x04;
    // pub const SERIAL: u8 = 0x08;
    // pub const JOYPAD: u8 = 0x10;
}

// Registers
pub mod registers {
    pub const JOYP: usize = 0xff00;
    pub const SB: usize = 0xff01;
    pub const SC: usize = 0xff02;
    pub const DIV: usize = 0xff04;
    pub const TIMA: usize = 0xff05;
    pub const TMA: usize = 0xff06;
    pub const TAC: usize = 0xff07;
    pub const IF: usize = 0xff0f;
    pub const NR10: usize = 0xff10;
    pub const NR11: usize = 0xff11;
    pub const NR12: usize = 0xff12;
    pub const NR13: usize = 0xff13;
    pub const NR14: usize = 0xff14;
    pub const NR21: usize = 0xff16;
    pub const NR22: usize = 0xff17;
    pub const NR23: usize = 0xff18;
    pub const NR24: usize = 0xff19;
    pub const NR30: usize = 0xff1a;
    pub const NR31: usize = 0xff1b;
    pub const NR32: usize = 0xff1c;
    pub const NR33: usize = 0xff1d;
    pub const NR34: usize = 0xff1e;
    pub const NR41: usize = 0xff20;
    pub const NR42: usize = 0xff21;
    pub const NR43: usize = 0xff22;
    pub const NR44: usize = 0xff23;
    pub const NR50: usize = 0xff24;
    pub const NR51: usize = 0xff25;
    pub const NR52: usize = 0xff26;
    pub const WAVE_RAM_START: usize = 0xff30;
    pub const WAVE_RAM_END: usize = 0xff3f;
    pub const LCDC: usize = 0xff40;
    pub const STAT: usize = 0xff41;
    pub const SCY: usize = 0xff42;
    pub const SCX: usize = 0xff43;
    pub const LY: usize = 0xff44;
    pub const LYC: usize = 0xff45;
    pub const DMA: usize = 0xff46;
    pub const BGP: usize = 0xff47;
    pub const OGBP0: usize = 0xff48;
    pub const OGBP1: usize = 0xff49;
    pub const WY: usize = 0xff4a;
    pub const WX: usize = 0xff4b;
    pub const IE: usize = 0xffff;
}

// Memory regions
pub mod regions {
    pub const ROM_BANK_0_START: usize = 0x0000;
    pub const ROM_BANK_0_END: usize = 0x3fff;
    pub const ROM_BANK_1_START: usize = 0x4000;
    pub const ROM_BANK_1_END: usize = 0x7fff;
    pub const VRAM_START: usize = 0x8000;
    pub const VRAM_END: usize = 0x9fff;
    pub const EXTERNAL_RAM_START: usize = 0xa000;
    pub const EXTERNAL_RAM_END: usize = 0xbfff;
    pub const WRAM_1_START: usize = 0xc000;
    pub const WRAM_1_END: usize = 0xcfff;
    pub const WRAM_2_START: usize = 0xd000;
    pub const WRAM_2_END: usize = 0xdfff;
    pub const ECHO_RAM_START: usize = 0xe000;
    pub const ECHO_RAM_END: usize = 0xfdff;
    pub const OAM_START: usize = 0xfe00;
    pub const OAM_END: usize = 0xfe9f;
    pub const IO_REGISTER_START: usize = 0xff00;
    pub const IO_REGISTER_END: usize = 0xff7f;
    pub const HRAM_START: usize = 0xff80;
    pub const HRAM_END: usize = 0xfffe;
    pub const INTERRUPT_FLAG: usize = 0xff0f;
    pub const INTERRUPT_ENABLE_REGISTER: usize = 0xffff;
}

#[derive(Debug, Copy, Clone)]
pub struct Memory {
    // (* 8 65536) == ~.5mb so i guess it's not that bad
    pub block: [u8; 65536],
}

impl Memory {
    pub fn read(&mut self, addr: usize) -> u8 {
        self.block[addr]
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.block[addr] = value;
    }

    pub fn inc_scanline(&mut self) {
        let ly = self.read(LY);
        self.write(LY, ly + 1);
        if self.read(LY) == 153 {
            self.write(LY, 0);
        }
    }

    pub fn lcd_status(&self) -> LcdStatus {
        LcdStatus::from(self.block[STAT])
    }

    pub fn lcd_control(&self) -> LcdControl {
        LcdControl::from(self.block[LCDC])
    }

    pub fn timer_control(&self) -> TimerControl {
        // TODO: this shouldn't unwrap
        TimerControl::try_from(self.block[TAC]).unwrap()
    }

    pub fn rom(&self) -> &[u8] {
        &self.block[ROM_BANK_0_START..ROM_BANK_1_END]
    }

    pub fn setup_rom(&mut self, rom: Vec<u8>, cartridge_type: CartridgeType) {
        match cartridge_type {
            CartridgeType::RomOnly => {
                self.block[ROM_BANK_0_START..ROM_BANK_0_END]
                    .copy_from_slice(&rom.as_slice()[ROM_BANK_0_START..ROM_BANK_0_END]);
                self.block[ROM_BANK_1_START..ROM_BANK_1_END]
                    .copy_from_slice(&rom.as_slice()[ROM_BANK_1_START..ROM_BANK_1_END]);
            }
            _ => (),
        }
    }

    pub fn inc_tima(&mut self) {
        let tima = self.read(TIMA);
        // This timer is incremented at the clock frequency specified by the TAC register ($FF07). When the value overflows (exceeds $FF) it is reset to the value specified in TMA (FF06) and an interrupt is requested.
        if tima == 0xff {
            self.block[IE] = interrupts::TIMER;
            self.block[TIMA] = self.read(TMA);
        }
        self.block[TIMA] += 1;
    }

    pub fn inc_div(&mut self) {
        if self.block[DIV] == 0xff {
            self.block[DIV] = 0;
        }
        self.block[DIV] += 1;
    }

    pub fn get_rom_bank_0(&self) -> &[u8] {
        &self.block[ROM_BANK_0_START..ROM_BANK_0_END]
    }

    pub fn get_rom_bank_1(&self) -> &[u8] {
        &self.block[ROM_BANK_1_START..ROM_BANK_1_END]
    }

    pub fn get_rom(&mut self) -> &[u8] {
        &self.block[ROM_BANK_0_START..ROM_BANK_1_END]
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.block[VRAM_START..VRAM_END]
    }

    pub fn get_external_ram(&self) -> &[u8] {
        &self.block[EXTERNAL_RAM_START..EXTERNAL_RAM_END]
    }

    pub fn get_work_ram_1(&self) -> &[u8] {
        &self.block[WRAM_1_START..WRAM_1_END]
    }

    pub fn get_work_ram_2(&self) -> &[u8] {
        &self.block[WRAM_2_START..WRAM_2_END]
    }

    pub fn get_oam(&self) -> &[u8] {
        &self.block[OAM_START..OAM_END]
    }

    pub fn get_io_registers(&self) -> &[u8] {
        &self.block[IO_REGISTER_START..IO_REGISTER_END]
    }

    pub fn get_hram(&self) -> &[u8] {
        &self.block[HRAM_START..HRAM_END]
    }

    pub fn get_interrupt_flag(&self) -> &u8 {
        &self.block[INTERRUPT_FLAG]
    }

    pub fn set_interrupt_registers(&mut self, value: u8) {
        self.write(INTERRUPT_ENABLE_REGISTER, value);
    }

    pub fn get_interrupt_registers(&self) -> &u8 {
        &self.block[INTERRUPT_ENABLE_REGISTER]
    }
}

impl Default for Memory {
    /// Fill hardware registers with their default values:
    /// Read more: https://gbdev.io/pandocs/Power_Up_Sequence.html#hardware-registers
    fn default() -> Self {
        let mut mem = Self {
            block: [0u8; 65536],
        };
        mem.write(JOYP, 0xcf);
        mem.write(SB, 0x00);
        mem.write(SC, 0x7e);
        mem.write(DIV, 0x18);
        mem.write(TAC, 0xf8);
        mem.write(IF, 0xe1);
        mem.write(NR10, 0x80);
        mem.write(NR11, 0xbf);
        mem.write(NR12, 0xf3);
        mem.write(NR13, 0xff);
        mem.write(NR14, 0xbf);
        mem.write(NR21, 0x3f);
        mem.write(NR22, 0x00);
        mem.write(NR23, 0xff);
        mem.write(NR24, 0xbf);
        mem.write(NR30, 0x7f);
        mem.write(NR31, 0xff);
        mem.write(NR32, 0x9f);
        mem.write(NR33, 0xff);
        mem.write(NR34, 0xbf);
        mem.write(NR41, 0xff);
        mem.write(NR42, 0x00);
        mem.write(NR43, 0xff);
        mem.write(NR44, 0xbf);
        mem.write(NR50, 0x77);
        mem.write(NR51, 0xf3);
        mem.write(NR52, 0xf1);
        mem.write(LCDC, 0x91);
        mem.write(STAT, 0x81);
        mem.write(SCY, 0x00);
        mem.write(SCX, 0x00);
        mem.write(LY, 0x00);
        mem.write(LYC, 0x00);
        mem.write(DMA, 0xff);
        mem.write(BGP, 0xfc);
        mem.write(WY, 0x00);
        mem.write(WX, 0x00);
        mem.write(IE, 0x00);
        mem
    }
}

#[derive(Debug)]
pub struct TimerControl {
    pub enable: bool,
    pub increment: u16,
    pub freq_single_speed: usize,
    pub freq_sgb1: usize,
    pub freq_double_speed: usize,
}

impl TryFrom<u8> for TimerControl {
    type Error = crate::errors::SystemError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let freq = value & 0x03;
        let enable = value & 0x04 != 0;
        match freq {
            0x00 => Ok(Self {
                enable,
                increment: 256,
                freq_single_speed: 4096,
                freq_sgb1: 4194,
                freq_double_speed: 8192,
            }),
            0x01 => Ok(Self {
                enable,
                increment: 4,
                freq_single_speed: 262144,
                freq_sgb1: 268400,
                freq_double_speed: 524288,
            }),
            0x02 => Ok(Self {
                enable,
                increment: 16,
                freq_single_speed: 65536,
                freq_sgb1: 67110,
                freq_double_speed: 131072,
            }),
            0x03 => Ok(Self {
                enable,
                increment: 64,
                freq_single_speed: 16384,
                freq_sgb1: 16780,
                freq_double_speed: 32768,
            }),
            _ => Err(SystemError::TimerControlError),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct LcdControl {
    lcd_ppu_enable: bool,
    window_tile_map: bool,
    window_enable: bool,
    bg_window_tile_data_area: bool,
    bg_tile_map: bool,
    obj_size: bool,
    obj_enable: bool,
    bg_window_enable: bool,
}

impl From<u8> for LcdControl {
    fn from(value: u8) -> Self {
        Self {
            lcd_ppu_enable: (value & 0x80) >> 7 == 1,
            window_tile_map: (value & 0x40) >> 6 == 1,
            window_enable: (value & 0x20) >> 5 == 1,
            bg_window_tile_data_area: (value & 0x10) >> 4 == 1,
            bg_tile_map: (value & 0x08) >> 3 == 1,
            obj_size: (value & 0x04) >> 2 == 1,
            obj_enable: (value & 0x02) >> 1 == 1,
            bg_window_enable: (value & 0x01) == 1,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LcdStatus {
    lyc_int_select: bool,
    pub mode_2_int_select: bool,
    pub mode_1_int_select: bool,
    pub mode_0_int_select: bool,
    lyu_lc: bool,
    ppu_mode: bool,
}

impl From<u8> for LcdStatus {
    fn from(value: u8) -> Self {
        Self {
            lyc_int_select: value & 0x40 != 0,
            mode_2_int_select: value & 0x20 != 0,
            mode_1_int_select: value & 0x10 != 0,
            mode_0_int_select: value & 0x08 != 0,
            lyu_lc: value & 0x04 != 0,
            ppu_mode: value & 0x03 != 0,
        }
    }
}
