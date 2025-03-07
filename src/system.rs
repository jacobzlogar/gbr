use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    render::{FRect, TextureCreator},
    sys::pixels::SDL_PIXELFORMAT_RGB24,
};

use crate::{
    apu::Apu,
    cartridge::Cartridge,
    clock::Clock,
    cpu::Cpu,
    display::{Ppu, PpuMode},
    errors::SystemError,
    instructions::jumps::call_n16,
    interrupts::Interrupt,
    memory::{Memory, registers::LY},
};

pub struct System {
    pub cpu: Cpu,
    pub apu: Apu,
    pub ppu: Ppu,
    pub clock: Clock,
    pub mem: Memory,
}

impl System {
    pub fn new(game: Vec<u8>) -> Result<Self, SystemError> {
        let cartridge = Cartridge::new(game.clone()).map_err(|_| SystemError::CartridgeError)?;
        let mut mem = Memory::new(cartridge);
        Ok(Self {
            cpu: Cpu::default(),
            apu: Apu::default(),
            ppu: Ppu::new(),
            clock: Clock::new(),
            mem,
        })
    }
    /// The following interrupt service routine is executed when control is being transferred to an interrupt handler:
    /// Two wait states are executed (2 M-cycles pass while nothing happens; presumably the CPU is executing nops during this time).
    /// The current value of the PC register is pushed onto the stack, consuming 2 more M-cycles.
    /// The PC register is set to the address of the handler (one of: $40, $48, $50, $58, $60). This consumes one last M-cycle.
    /// Read more: https://gbdev.io/pandocs/Interrupts.html
    fn handle_interrupt(&mut self) -> Result<(), SystemError> {
        if let Some(interrupt) = Interrupt::get_interrupt(self.mem.get_interrupt_registers()) {
            // https://gbdev.io/pandocs/Interrupt_Sources.html
            let handler = match interrupt {
                Interrupt::VBlank => 0x40,
                Interrupt::Stat => 0x48,
                Interrupt::Timer => 0x50,
                Interrupt::Serial => 0x58,
                Interrupt::Joypad => 0x60,
            };
            call_n16(handler, &mut self.cpu, &mut self.mem)
                .map_err(|_| SystemError::InterruptHandlerError(interrupt, handler))?;
        }
        Ok(())
    }

    pub fn run(&mut self) {
        let mut texture_creator = self.ppu.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormat::try_from(SDL_PIXELFORMAT_RGB24).unwrap(),
                160,
                144,
            )
            .unwrap();
        self.ppu.canvas.set_draw_color(Color::WHITE);
        self.ppu.canvas.clear();
        'running: loop {
            // execute instructions
            self.clock.m_cycles += self.cpu.execute(&mut self.mem).unwrap() as usize;
            // advance the clock
            self.clock.tick(&mut self.mem);
            // process audio
            self.apu.process();
            // handle interrupts
            if self.cpu.ime {
                self.handle_interrupt();
            }
            let scanline = self.mem.read(LY);
            let lcdc = self.mem.lcd_control();
            // scanline 144 is the beginning of vblank
            if scanline <= 143 && lcdc.lcd_ppu_enable {
                let pixels = self.ppu.update_scanline(&mut self.mem, &self.clock, &lcdc, scanline);
                texture.with_lock(None, |buffer: &mut [u8], _: usize| {
                    let start = (scanline as usize * 480) as usize;
                    let end = start + 480;
                    buffer[start..end].copy_from_slice(&pixels);
                });
                self.ppu.canvas
                    .copy(&texture, None, Some(FRect::new(0.0, 0.0, 160.0, 144.0)))
                    .unwrap();
                self.clock.dots += 4;
            }

            match scanline {
                143 => self.ppu.mode = PpuMode::VerticalBlank,
                _ => (),
            };
            match self.clock.dots {
                0..=80 => {
                    // self.oam_scan(mem, scanline);
                    self.ppu.mode = PpuMode::OAMScan;
                }
                81..=252 => {
                    self.ppu.mode = PpuMode::Drawing;
                    self.mem.oam_accessible = false;
                    self.mem.vram_accessible = false;
                    if lcdc.window_enable {}
                    // TODO: add obj penalty variable mode length algorithm
                    if lcdc.bg_window_enable {}
                }
                _ => {
                    self.mem.oam_accessible = true;
                    self.mem.vram_accessible = true;
                }
            }
            for event in self.ppu.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            self.ppu.canvas.present();
        }
    }
}
