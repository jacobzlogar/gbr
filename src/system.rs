use std::{fmt::Debug, io::Read};

use sdl3::{
    event::Event,
    iostream::IOStream,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    rect::Rect,
    render::{FPoint, FRect, TextureAccess},
    surface::Surface,
};

use crate::{
    DecodeContext,
    apu::Apu,
    cartridge::Cartridge,
    clock::Clock,
    cpu::Cpu,
    display::Ppu,
    errors::{CartridgeError, SystemError},
    memory::{
        Memory,
        registers::{DIV, IE, TMA},
    },
};

pub struct System {
    pub cpu: Cpu,
    pub apu: Apu,
    pub ppu: Ppu,
    pub mem: Memory,
    pub clock: Clock,
    pub cartridge: Cartridge,
}

#[allow(dead_code)]
impl System {
    pub fn new(rom: Vec<u8>) -> Result<Self, SystemError> {
        let mut mem = Memory::default();
        let cartridge =
            Cartridge::new(rom.clone(), &mut mem).map_err(|_| SystemError::CartridgeError)?;
        let ppu = Ppu::new(&cartridge.title);
        let cpu = Cpu::default();
        Ok(Self {
            cpu,
            apu: Apu::default(),
            clock: Clock::new(&mut mem),
            mem,
            ppu,
            cartridge,
        })
    }

    fn handle_interrupts(&mut self) {
        if let Some(interrupt) = self.mem.interrupt_enable() {
            println!("{:?}", interrupt);
        }
    }

    pub fn execute(&mut self) -> ! {
        loop {
            self.clock.tick(&mut self.mem);
            // execute instructions
            let cpu_cycles = self.cpu.execute(&mut self.mem).unwrap() as usize;
            self.clock.m_cycles += cpu_cycles;
            // self.clock.master_clock += cpu_cycles as u64;
            // process audio
            self.apu.process();
            // handle interrupts
            self.handle_interrupts();
            // self.ppu.canvas.clear();
        }
        // println!("{:?}", self.cartridge);
        // self.ppu.canvas.set_draw_color(Color::WHITE);
        // self.ppu.canvas.clear();
        // self.ppu.canvas.present();
        // 'running: loop {
        //     self.clock.tick(&mut self.mem);
        //     // execute instructions
        //     let _cpu_cycles = self.cpu.execute(&mut self.mem).unwrap();
        //     // process audio
        //     self.apu.process();
        //     // handle interrupts
        //     self.handle_interrupts();
        //     // self.ppu.canvas.clear();
        //     for event in self.ppu.event_pump.poll_iter() {
        //         match event {
        //             Event::Quit { .. }
        //             | Event::KeyDown {
        //                 keycode: Some(Keycode::Escape),
        //                 ..
        //             } => break 'running,
        //             _ => {}
        //         }
        //     }
        //     // draw the canvas
        //     // self.ppu.canvas.present();
        // }
    }
}
