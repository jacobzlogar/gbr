pub const CPU_CYCLES_PER_CLOCK: u64 = 4;
pub const PPU_CYCLES_PER_SCANLINE: u64 = 114;
pub const CYCLES_PER_FRAME: u64 = 70224;
pub const MASTER_CLOCK_FREQUENCY: u64 = 4190000;

#[derive(Debug, Copy, Clone)]
pub struct Clock {
    pub master_clock: u64,
    pub cpu_cycles: u64,
    pub ppu_cycles: u64,
    pub ppu_interrupt: u64,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            master_clock: 0,
            cpu_cycles: 0,
            ppu_cycles: 0,
            ppu_interrupt: 0,
        }
    }
}
