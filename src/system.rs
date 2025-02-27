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
    memory::{registers::LY, Memory},
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
        let lcdc = mem.lcd_control();
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
                Interrupt::Joypad => 0x60
            };
            call_n16(handler, &mut self.cpu, &mut self.mem)
                .map_err(|_| SystemError::InterruptHandlerError(interrupt, handler))?;
        }
        Ok(())
    }

    pub fn run(&mut self) {
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
            // scanline 144 is the beginning of vblank
            if self.mem.read(LY) <= 143 {
                self.ppu.render_scanline(&mut self.mem, &self.clock);
            }
            self.ppu.canvas.present();
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
        }
    }
}
