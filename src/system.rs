use crate::{
    apu::Apu, cartridge::Cartridge, clock::Clock, cpu::Cpu, display::Ppu, memory::{registers::DIV, Memory}
};

pub struct System {
    pub cpu: Cpu,
    pub apu: Apu,
    pub ppu: Ppu,
    pub mem: Memory,
    pub clock: Clock,
    // cartridge: Cartridge
}

#[allow(dead_code)]
impl System {
    pub fn new(rom: Vec<u8>) -> Self {
        //let cartridge = Cartridge::read(rom);
        Self {
            cpu: Cpu::new(rom),
            apu: Apu::default(),
            ppu: Ppu::default(),
            clock: Clock::default(),
            mem: Memory::default(),
        }
    }

    fn handle_interrupts(&mut self) {
        if self.mem.read(DIV) == 255 {
            self.mem.write(DIV, 0);
            println!("div {}", self.clock.master_clock);
        }
    }

    pub fn execute(&mut self) -> ! {
        loop {
            self.clock.tick(&mut self.mem);
            // execute instructions
            let _ = self.cpu.execute(&mut self.mem);
            // // process audio
            self.apu.process();
            // // render scanlines
            self.ppu.render_scanline();
            // // handle interrupts
            self.handle_interrupts()
        }
    }
}
