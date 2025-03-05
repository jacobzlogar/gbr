pub mod joypad;

// I/O ranges for peripherals;
pub const JOYPAD_INPUT: u16 = 0xff00;
pub const SERIAL_TRANSFER_START: u16 = 0xff01;
pub const SERIAL_TRANSFER_END: u16 = 0xff02;
pub const TIMER_DIVIDER_START: u16 = 0xff04;
pub const TIMER_DIVIDER_END: u16 = 0xff07;
pub const INTERRUPTS: u16 = 0xff0f;
pub const AUDIO_START: u16 = 0xff10;
pub const AUDIO_END: u16 = 0xff26;
pub const WAVE_PATTERN_START: u16 = 0xff30;
pub const WAVE_PATTERN_END: u16 = 0xff3f;
pub const LCD_CONTROL_START: u16 = 0xff40;
pub const LCD_CONTROL_END: u16 = 0xff4b;
pub const VRAM_BANK_SELECT: u16 = 0xff4f;
pub const DISABLE_BOOT_ROM: u16 = 0xff50;
pub const VRAM_DMA_START: u16 = 0xff51;
pub const VRAM_DMA_END: u16 = 0xff55;
pub const BG_OBJ_PALETTE_START: u16 = 0xff68;
pub const BG_OBJ_PALETTE_END: u16 = 0xff6b;
pub const WRAM_BANK_SELECT: u16 = 0xff70;

#[derive(Debug)]
pub struct LcdControl {
    pub lcd_ppu_enable: bool,
    pub window_tile_map_area: [usize; 2],
    pub bg_tile_map_area: [usize; 2],
    pub window_enable: bool,
    pub tile_data_area: [[usize; 2]; 2],
    // pub bg_tile_map_area: [usize; 2],
    obj_size: u8,
    obj_enable: bool,
    pub bg_window_enable: bool,
}

impl std::fmt::Display for LcdControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let window_tile_map_area = format!(
            "window tile map area: 0x{:0x} - 0x{:0x}",
            self.window_tile_map_area[0], self.window_tile_map_area[1]
        );
        let bg_tile_map_area = format!(
            "bg tile map area: 0x{:0x} - 0x{:0x}",
            self.bg_tile_map_area[0], self.bg_tile_map_area[1]
        );
        let tile_data_area = format!(
            "tile data area block_0: 0x{:0x}-{:0x} - block_1: 0x{:0x}-{:0x}",
            self.tile_data_area[0][0],
            self.tile_data_area[0][1],
            self.tile_data_area[1][0],
            self.tile_data_area[1][1]
        );
        write!(
            f,
            "\n{window_tile_map_area}\n{bg_tile_map_area}\n{tile_data_area}\nlcd & ppu enabled: {}\nbg window enabled: {}",
            self.lcd_ppu_enable, self.bg_window_enable
        )
    }
}

impl From<u8> for LcdControl {
    fn from(value: u8) -> Self {
        let window_tile_map_area = match (value & 0x40) >> 6 {
            0 => [0x9800, 0x9bff],
            _ => [0x9c00, 0x9fff],
        };
        let tile_data_area = match (value & 0x10) >> 4 {
            0 => [[0x8800, 0x8fff], [0x9000, 0x97ff]],
            _ => [[0x8000, 0x87ff], [0x8800, 0x8fff]],
        };
        let bg_tile_map_area = match (value & 0x08) >> 3 {
            0 => [0x9800, 0x9bff],
            _ => [0x9c00, 0x9fff],
        };
        Self {
            lcd_ppu_enable: (value & 0x80) >> 7 == 1,
            window_tile_map_area,
            window_enable: (value & 0x20) >> 5 == 1,
            tile_data_area,
            bg_tile_map_area,
            obj_size: (value & 0x04) >> 2,
            obj_enable: (value & 0x02) >> 1 == 1,
            bg_window_enable: (value & 0x01) == 1,
        }
    }
}

#[derive(Debug)]
pub struct LcdStatus {
    pub lyc_int_select: bool,
    pub mode_2_int_select: bool,
    pub mode_1_int_select: bool,
    pub mode_0_int_select: bool,
    pub lyu_lc: bool,
    pub ppu_mode: bool,
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
            _ => Err(crate::errors::SystemError::TimerControlError),
        }
    }
}
