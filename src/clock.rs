use crate::memory::{Memory, TimerControl};

pub const T_CYCLES_PER_FRAME: usize = 70224;

#[derive(Debug)]
pub struct Clock {
    pub t_cycles: usize,
    pub dots: usize,
    pub timer_control: TimerControl,
}

impl Clock {
    pub fn new(mem: &mut Memory) -> Self {
        Self {
            t_cycles: 0,
            dots: 0,
            timer_control: mem.timer_control(),
        }
    }
    pub fn tick(&mut self, mem: &mut Memory) {
        self.dots = self.t_cycles * 4;
        if self.dots >= 456 {
            self.dots = 0;
            mem.inc_scanline();
        }
        if self.t_cycles > 153 {
            self.t_cycles = 0;
        }
    }
}
