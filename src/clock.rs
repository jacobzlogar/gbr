use crate::memory::{Memory, TimerControl};

pub const T_CYCLES_PER_FRAME: usize = 70224;

#[derive(Debug)]
pub struct Clock {
    pub m_cycles: usize,
    pub dots: usize,
    pub timer_control: TimerControl,
}

impl Clock {
    pub fn new(mem: &mut Memory) -> Self {
        Self {
            m_cycles: 0,
            dots: 0,
            timer_control: mem.timer_control(),
        }
    }
    pub fn tick(&mut self, mem: &mut Memory) {
        self.dots = self.m_cycles * 4;
        if self.dots >= 456 {
            self.dots = 0;
            mem.inc_scanline();
        }
        if self.m_cycles > 143 {
            let vblank = mem.get_interrupt_registers() | 1;
            mem.set_interrupt_registers(vblank);
        }
        if self.m_cycles > 153 {
            self.m_cycles = 0;
        }
    }
}
