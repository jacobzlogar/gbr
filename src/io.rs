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
