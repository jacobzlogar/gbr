extern crate sdl3;

use sdl3::pixels::{Color, PixelFormat};
use sdl3::rect::Rect;
use sdl3::render::{Canvas, FRect, Texture, TextureAccess, TextureCreator};
use sdl3::sys::pixels::{
    SDL_PIXELFORMAT_ABGR32, SDL_PIXELFORMAT_RGB24, SDL_PixelFormat, SDL_PixelFormatDetails,
};
use sdl3::sys::rect::SDL_GetRectAndLineIntersectionFloat;
use sdl3::sys::stdinc::SDL_sinf;
use sdl3::video::{Window, WindowContext};
use sdl3::{Error, EventPump};

use crate::clock::Clock;
use crate::io::LcdControl;
use crate::memory::Memory;
use crate::memory::registers::{LCDC, LY};

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
    VerticalBlank,   // waiting until the next frame, all vram sectitons become accessible to cpu
    OAMScan,         // searching for OBJS which overlap the current scanline
    Drawing,         // sending pixels to the LCD
}
pub struct Ppu {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub obj_penalty: usize,
    pub scanline: u16,
    pub mode: PpuMode,
    pub frame_buffer: Vec<u8>,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl Ppu {
    pub fn new() -> Self {
        let (canvas, event_pump) = setup_ctx().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            canvas,
            event_pump,
            obj_penalty: 0,
            scanline: 0,
            mode: PpuMode::OAMScan,
            frame_buffer: vec![],
            texture_creator,
        }
    }
    pub fn oam_scan(&mut self, mem: &mut Memory, scanline: u8) {
        let oam = mem.get_oam();
        for chunk in oam.chunks_exact(4) {
            if chunk[0] == scanline {}
        }
    }

    pub fn render_scanline(
        &mut self,
        mem: &mut Memory,
        clock: &Clock,
        lcdc: &LcdControl,
        texture: &mut Texture,
    ) {
        let scanline = mem.block[LY];
        let window_tile_map = mem.get_tile_map(lcdc.window_tile_map_area);
        let bg_tile_map = mem.get_tile_map(lcdc.bg_tile_map_area);
        let (tile_block_0, tile_block_1) = mem.get_tile_data(lcdc.tile_data_area);
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                println!("{pitch}");
                // tile maps are 32x32
                for y in 0..32 {
                    for x in 0..32 {
                        let tile_map = bg_tile_map[y * 32 + x];
                        // tiles are 8x8
                        for i in 0..8 {
                            for j in 0..8 {
                                let offset = (y * 8 + j) * pitch + (x * 8 + i) * 3;
                                // let offset = (y * 8 + j) * pitch + (x * 8 + i) * 4;
                                let tile_index: usize = i + j * 8;
                                if tile_map <= 127 {
                                    let pixel = tile_block_1[tile_map as usize][tile_index];
                                    buffer[offset] = pixel;
                                    buffer[offset + 1] = pixel;
                                    buffer[offset + 2] = pixel;
                                } else {
                                    let pixel = tile_block_0[tile_map as usize][tile_index];
                                    buffer[offset] = pixel;
                                    buffer[offset + 1] = pixel;
                                    buffer[offset + 2] = pixel;
                                }
                            }
                        }
                        // println!("{:?}", bg_tile_map.len());
                    }
                }
            })
            .unwrap();
        self.canvas
            .copy(&texture, None, Some(FRect::new(0.0, 0.0, 256.0, 256.0)))
            .unwrap();
        match scanline {
            143 => self.mode = PpuMode::VerticalBlank,
            _ => (),
        };
        match clock.dots {
            0..=80 => {
                // self.oam_scan(mem, scanline);
                self.mode = PpuMode::OAMScan;
            }
            81..=252 => {
                self.mode = PpuMode::Drawing;
                mem.oam_accessible = false;
                mem.vram_accessible = false;
                if lcdc.window_enable {
                    // println!("window_tile_map: {:?}", lcdc.window_tile_map_area);
                }
                if lcdc.bg_window_enable {}
                // TODO: add obj penalty variable mode length algorithm
                self.canvas.present();
            }
            _ => {
                mem.oam_accessible = true;
                mem.vram_accessible = true;
            }
        }
    }
}

pub fn setup_ctx() -> Result<(Canvas<Window>, EventPump), Error> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("test", 256, 256)
        .position_centered()
        .build()
        .unwrap();

    Ok((window.into_canvas(), sdl_context.event_pump()?))
}

pub mod tests {
    use crate::cartridge::Cartridge;
    use crate::{dump_tiles, memory::Memory};

    #[test]
    fn test_decode_tile() {}
}
