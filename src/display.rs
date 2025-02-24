extern crate sdl3;

use std::collections::hash_map::Drain;

use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::{Error, EventPump};

use crate::clock::Clock;
use crate::memory::Memory;
use crate::memory::registers::LY;
/// ```ignore
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
/// ```
/// Read more: https://gbdev.io/pandocs/Rendering.html
#[derive(PartialEq, Eq)]
pub enum PpuMode {
    HorizontalBlank, // waiting until the end of the scanline
    VerticalBlank,   // waiting until the next frame
    OAMScan,         // searching for OBJS which overlap this line
    Drawing,         // sending pixels to the LCD
}

pub struct Ppu {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub obj_penalty: usize,
    pub scanline: u16,
    pub mode: PpuMode,
    pub frame_buffer: Vec<u16>,
}

impl Ppu {
    pub fn new() -> Self {
        let (canvas, event_pump) = setup_ctx().unwrap();
        Self {
            canvas,
            event_pump,
            obj_penalty: 0,
            scanline: 0,
            mode: PpuMode::OAMScan,
            frame_buffer: vec![],
        }
    }
    pub fn oam_scan(&mut self, mem: &mut Memory, scanline: u8) {
        let oam = mem.get_oam();
        for chunk in oam.chunks_exact(4) {
            if chunk[0] == scanline {
            }
        }
    }
    pub fn render_scanline(&mut self, mem: &mut Memory, clock: &Clock) {
        let scanline = mem.read(LY);
        let lcdc = mem.lcd_control();
        for i in 0x8000..0x87ff {
            
        }
        match scanline {
            0 => {
                mem.oam_accessible = false;
                self.mode = PpuMode::OAMScan;
            }
            143 => self.mode = PpuMode::VerticalBlank,
            _ => (),
        };

        match clock.dots {
            0..=80 => {
                self.oam_scan(mem, scanline);
                self.mode = PpuMode::OAMScan;
            }
            81 => {
                self.mode = PpuMode::Drawing;
                mem.oam_accessible = false;
                mem.vram_accessible = false;
            }
            _ => (),
        }
    }
}

pub fn setup_ctx() -> Result<(Canvas<Window>, EventPump), Error> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("test", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    Ok((window.into_canvas(), sdl_context.event_pump()?))
}
