use sdl3::{event::Event, keyboard::Keycode, pixels::Color};

use crate::{
    apu::Apu,
    cartridge::Cartridge,
    clock::Clock,
    cpu::Cpu,
    display::{Ppu, PpuMode},
    errors::SystemError,
    instructions::jumps::call_n16,
    interrupts::Interrupt,
    memory::Memory,
};

pub struct System {
    pub cpu: Cpu,
    pub apu: Apu,
    pub ppu: Ppu,
    pub mem: Memory,
    pub clock: Clock,
    pub cartridge: Cartridge,
}

pub const VBLANK_INT_HANDLER: u16 = 0x40;

impl System {
    pub fn new(game: Vec<u8>) -> Result<Self, SystemError> {
        let mut mem = Memory::default();
        let cartridge =
            Cartridge::new(game.clone(), &mut mem).map_err(|_| SystemError::CartridgeError)?;
        Ok(Self {
            cpu: Cpu::default(),
            apu: Apu::default(),
            ppu: Ppu::new(),
            mem,
            clock: Clock::new(&mut mem),
            cartridge,
        })
    }

    fn handle_interrupts(&mut self) {
        if self.ppu.mode == PpuMode::Drawing {
            println!("drawing");
        }
        if let Some(interrupt) = Interrupt::get_interrupt(self.mem.get_interrupt_registers()) {
            match interrupt {
                Interrupt::VBlank => {
                    let _ = call_n16(VBLANK_INT_HANDLER, &mut self.cpu, &mut self.mem);
                }
                _ => (),
            }
        }
    }

    pub fn execute(&mut self) {
        self.ppu.canvas.set_draw_color(Color::WHITE);
        'running: loop {
            self.clock.tick(&mut self.mem);
            // execute instructions
            self.clock.m_cycles += self.cpu.execute(&mut self.mem).unwrap() as usize;
            // process audio
            self.apu.process();
            // handle interrupts
            self.handle_interrupts();
            // render
            self.ppu.render_scanline(&mut self.mem, &self.clock);
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
            // draw the canvas
            self.ppu.canvas.present();
        }
    }
}
