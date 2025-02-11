use crate::{
    clock::{CPU_CYCLES_PER_CLOCK, CYCLES_PER_FRAME, Clock, PPU_CYCLES_PER_SCANLINE},
    cpu::Cpu,
    display::Ppu,
    errors::SystemError,
    memory::{MemoryMap, registers::*},
};

pub struct System {
    pub cpu: Cpu,
    pub ppu: Ppu,
    pub mem: MemoryMap,
    pub clock: Clock,
}

#[allow(dead_code)]
impl System {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(rom),
            ppu: Ppu::default(),
            clock: Clock::default(),
            mem: MemoryMap::default(),
        }
    }

    pub fn execute(mut self) -> ! {
        loop {
            self.clock.master_clock += 1;
            self.clock.cpu_cycles += 1;
            // 4 cpu cycles per clock
            if self.clock.cpu_cycles >= CPU_CYCLES_PER_CLOCK {
                self.clock.cpu_cycles = 0;
                //self.clock.cpu_cycles += self.cpu.execute(&mut self.mem.block).unwrap() as u64;
            }
            // 114 ppu cycles per clock, i need to figure out where i read this number
            self.clock.ppu_cycles += 1;
            if self.clock.ppu_cycles >= PPU_CYCLES_PER_SCANLINE {
                self.clock.ppu_cycles = 0;
                self.ppu.render_scanline(&self.mem.block);
            }
            if self.clock.master_clock % CYCLES_PER_FRAME == 0 {
                // v-blank
            }
        }
    }
}
