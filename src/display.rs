use std::sync::{Arc, atomic::AtomicU8, mpmc::Receiver};

use crate::clock::Clock;

/// These modes represent the modes the PPU cycles between during a frame
///
/// A frame consists of 154 scan lines, during the first 144 the screen is drawn top to bottom, left to right
/// A “dot” = one 222 Hz (≅ 4.194 MHz) time unit.
///
///      |OAMScan |    Drawing     |    HorizontalBlank   |
///      | 80 dots| 172-289 dots   | 87-204 dots
///               |----------------| VRAM (8000-9FFF) accessible
///      |-------------------------| OAM inaccessible
/// LY=0 |        |                |                      |
///  144 |-------------- Vertical Blank ------------------|
///  ... |             Everything Accessible              |
///  153 |-------------- Vertical Blank ------------------|
///
/// Read more: https://gbdev.io/pandocs/Rendering.html
pub enum PpuMode {
    HorizontalBlank, // waiting until the end of the scanline
    VerticalBlank,   // waiting until the next frame
    OAMScan,         // searching for OBJS which overlap this line
    Drawing,         // sending pixels to the LCD
}

pub struct Ppu {
    scanline: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Self { scanline: 0 }
    }
}

impl Ppu {
    pub fn render_scanline(&self, memory: &[u8]) -> () {}
}
