extern crate sdl3;

use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::{Error, EventPump};

use crate::clock::Clock;
use crate::memory::Memory;
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
pub enum PpuMode {
    HorizontalBlank, // waiting until the end of the scanline
    VerticalBlank,   // waiting until the next frame
    OAMScan,         // searching for OBJS which overlap this line
    Drawing,         // sending pixels to the LCD
}

#[allow(dead_code)]
pub struct Ppu {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    scanline: u16,
    mode: PpuMode,
}

impl Ppu {
    pub fn new() -> Self {
        let (canvas, event_pump) = setup_ctx().unwrap();
        Self {
            scanline: 0,
            mode: PpuMode::OAMScan,
            canvas,
            event_pump,
        }
    }
    pub fn render_scanline(&mut self, mem: &mut Memory, clock: &Clock) {
        // match clock.dots {
        //     81.. => {
        //         self.mode = PpuMode::Drawing
        //     },
        //     _ => {}
        // }
    }
}

pub fn setup_ctx() -> Result<(Canvas<Window>, EventPump), Error> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "test",
            640,
            480,
        )
        .position_centered()
        .build()
        .unwrap();

    Ok((window.into_canvas(), sdl_context.event_pump()?))
}
