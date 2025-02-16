use regions::*;
use registers::*;

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
    pub const INTERRUPT_ENABLE_REGISTER: usize = 0xffff;
}

#[derive(Debug)]
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

    pub fn lcd_status(&self) -> LcdStatus {
        LcdStatus::from(self.block[STAT])
    }

    pub fn timer_control(&self) -> TimerControl {
        TimerControl::from(self.block[TAC])
    }

    pub fn inc_div(&mut self) {
        self.block[DIV] += 1;
    }
    // im not sure i'll actually end up using any of these, lol
    pub fn get_rom_bank_0(&self) -> &[u8] {
        &self.block[ROM_BANK_0_START..ROM_BANK_0_END]
    }

    pub fn get_rom_bank_1(&self) -> &[u8] {
        &self.block[ROM_BANK_1_START..ROM_BANK_1_END]
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
        mem.write(LY, 0x91);
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
pub enum TimerFreq {
    First(u32, u32, u32, u32),
    Second(u32, u32, u32, u32),
    Third(u32, u32, u32, u32),
    Fourth(u32, u32, u32, u32),
}

#[derive(Debug)]
pub struct TimerControl {
    enable: bool,
    clock_select: TimerFreq,
}

impl From<u8> for TimerControl {
    fn from(value: u8) -> Self {
        let freq = value & 0x03;
        let freq = match freq {
            0 => TimerFreq::First(256, 4096, 4194, 8192),
            1 => TimerFreq::Second(4, 262144, 268400, 524288),
            2 => TimerFreq::Third(16, 65536, 67110, 131072),
            _ => TimerFreq::First(64, 16384, 16780, 32768),
        };
        Self {
            enable: value & 0x04 != 0,
            clock_select: freq,
        }
    }
}

#[derive(Debug)]
pub struct LcdStatus {
    lyc_int_select: bool,
    mode_2_int_select: bool,
    mode_1_int_select: bool,
    mode_0_int_select: bool,
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
