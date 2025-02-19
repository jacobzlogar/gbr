use crate::memory::{Memory, TimerControl};

pub const CPU_CYCLES_PER_CLOCK: u64 = 4;
pub const PPU_CYCLES_PER_SCANLINE: u64 = 114;
pub const CYCLES_PER_FRAME: u64 = 70224;
pub const MASTER_CLOCK_FREQUENCY: u64 = 4190000;
pub const DIV_INC_RATE: usize = 16384;

#[derive(Debug)]
pub struct Clock {
    pub master_clock: usize,
    pub frame_clock: usize,
    pub m_cycles: usize,
    pub t_cycles: usize, // "dots"
    pub timer_control: TimerControl,
}

impl Clock {
    pub fn new(mem: &mut Memory) -> Self {
        Self {
            master_clock: 0,
            frame_clock: 0,
            m_cycles: 0,
            t_cycles: 0,
            timer_control: mem.timer_control(),
        }
    }
    pub fn tick(&mut self, mem: &mut Memory) {
        self.master_clock += 1;
        self.frame_clock += 1;
        if self.frame_clock == DIV_INC_RATE {
            self.frame_clock = 0;
            mem.inc_div();
        }
        if self.m_cycles >= self.timer_control.increment as usize {
            mem.inc_tima();
            self.m_cycles = 0;
        }
    }
}
