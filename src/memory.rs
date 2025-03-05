use regions::*;
use registers::*;

use crate::{
    cartridge::{Cartridge, CartridgeType},
    decode_tile,
    errors::SystemError,
    io::{LcdControl, LcdStatus, TimerControl},
};

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
    pub const TILE_BLOCK_0_START: usize = 0x8000;
    pub const TILE_BLOCK_0_END: usize = 0x87ff;
    pub const TILE_BLOCK_1_START: usize = 0x8800;
    pub const TILE_BLOCK_1_END: usize = 0x8fff;
    pub const TILE_BLOCK_2_START: usize = 0x9000;
    pub const TILE_BLOCK_2_END: usize = 0x97ff;
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

#[derive(Debug, Clone)]
pub struct Memory {
    pub block: [u8; 65536],
    pub cartridge: Cartridge,
    pub oam_accessible: bool,
    pub vram_accessible: bool,
    pub rom_banks: Vec<[u8; 16383]>,
}

impl Memory {
    /// Fill hardware registers with their default values:
    /// Read more: https://gbdev.io/pandocs/Power_Up_Sequence.html#hardware-registers
    /// Setup memory banks based on cartridge values:
    /// Read more: https://gbdev.io/pandocs/MBCs.html
    pub fn new(cartridge: Cartridge) -> Self {
        let mut mem = Self {
            block: [0u8; 65536],
            cartridge,
            oam_accessible: true,
            vram_accessible: true,
            rom_banks: vec![],
        };
        mem.setup_mbc();
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
    pub fn read(&mut self, addr: usize) -> u8 {
        if addr >= 0x8000 && addr <= 0x97ff {
            // println!("accessing vram: {addr:?}");
        }
        // oam can't be read or written to during ppu mode 2 or mode 3
        if addr >= 0xfe00 && addr <= 0xfe9f && (!self.oam_accessible || !self.vram_accessible) {
            return 0xff;
        }
        // vram can't be read or written to during ppu mode 3
        if addr >= 0x8000 && addr <= 0x9fff && !self.vram_accessible {
            return 0xff;
        }
        self.block[addr]
    }

    // TODO: wire up MBC
    pub fn write(&mut self, addr: usize, value: u8) {
        if addr >= 0x2000 && addr <= 0x3fff {
            println!("switching rom banks");
        }
        if addr >= 0x4000 && addr <= 0x5fff {
            println!("switching rom banks");
        }
        if addr >= 0x6000 && addr <= 0x7fff {
            println!("banking mode select");
        }
        if addr >= 0xfe00 && addr <= 0xfe9f && (!self.oam_accessible || !self.vram_accessible) {
            println!("Attempting to write to hram");
            // return;
        }
        if addr >= 0x8000 && addr <= 0x9fff {
            println!("Attempting to write to vram");
            // return;
        }
        self.block[addr] = value;
    }

    pub fn inc_scanline(&mut self) {
        let ly = self.read(LY);
        if self.read(LY) == 153 {
            self.write(LY, 0);
        } else {
            self.write(LY, ly + 1);
        }
    }

    pub fn get_tile_map(&mut self, tile_map_area: [usize; 2]) -> [u8; 1024] {
        let mut tile_map = [0u8; 1024];
        let slice = &self.block[tile_map_area[0]..=tile_map_area[1]];
        tile_map.copy_from_slice(slice);
        tile_map
    }

    pub fn get_tile_data(
        &mut self,
        tile_data_area: [[usize; 2]; 2],
    ) -> ([[u8; 64]; 128], [[u8; 64]; 128]) {
        let mut tile_block_0 = [[0u8; 64]; 128];
        for (tile, chunk) in tile_block_0
            .iter_mut()
            .zip(self.block[tile_data_area[0][0]..=tile_data_area[0][1]].chunks_exact(16))
        {
            *tile = decode_tile(chunk);
        }
        let mut tile_block_1 = [[0u8; 64]; 128];
        for (tile, chunk) in tile_block_1
            .iter_mut()
            .zip(self.block[tile_data_area[1][0]..=tile_data_area[1][1]].chunks_exact(16))
        {
            *tile = decode_tile(chunk);
        }
        (tile_block_0, tile_block_1)
    }

    pub fn lcd_status(&self) -> LcdStatus {
        LcdStatus::from(self.block[STAT])
    }

    pub fn lcd_control(&self) -> LcdControl {
        LcdControl::from(self.block[LCDC])
    }

    pub fn timer_control(&self) -> TimerControl {
        TimerControl::try_from(self.block[TAC]).unwrap()
    }

    pub fn rom(&self) -> &[u8] {
        &self.block[ROM_BANK_0_START..ROM_BANK_1_END]
    }

    pub fn setup_mbc(&mut self) {
        let chunks: Vec<[u8; 16383]> = self
            .cartridge
            .rom
            .chunks_exact(16383)
            .map(|chunk| <[u8; 16383]>::try_from(chunk).unwrap())
            .collect();
        self.rom_banks = chunks;
        self.block[ROM_BANK_0_START..ROM_BANK_0_END].copy_from_slice(&self.rom_banks[0]);
        self.block[ROM_BANK_1_START..ROM_BANK_1_END].copy_from_slice(&self.rom_banks[1]);
    }

    pub fn inc_tima(&mut self) {
        let tima = self.read(TIMA);
        // This timer is incremented at the clock frequency specified by the TAC register ($FF07).
        // When the value overflows (exceeds $FF) it is reset to the value specified in TMA (FF06) and an interrupt is requested.
        if tima == 0xff {
            self.block[IE] = crate::interrupts::TIMER;
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

    pub fn get_vram(&self) -> &[u8] {
        &self.block[VRAM_START..VRAM_END]
    }

    pub fn get_oam(&self) -> &[u8] {
        &self.block[OAM_START..OAM_END]
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
