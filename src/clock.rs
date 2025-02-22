use crate::{
    memory::Memory,
    io::TimerControl
};

pub const T_CYCLES_PER_FRAME: usize = 70224;

#[derive(Debug)]
pub struct Clock {
    pub m_cycles: usize,
    pub dots: usize,
    // pub timer_control: TimerControl, // use this eventually
}

impl Clock {
    pub fn new(mem: &mut Memory) -> Self {
        Self {
            m_cycles: 0,
            dots: 0,
            // timer_control: mem.timer_control(),
        }
    }
    pub fn tick(&mut self, mem: &mut Memory) {
        self.dots = self.m_cycles * 4;
        // a scanline has been completed
        if self.dots % 456 == 0 {
            mem.inc_scanline();
        }
        // a second should have elapsed, 70224 dots * 59.7 fps
        if self.dots % 70224 == 0 {
            // println!("1 second");
        }
        // request vblank int
        if self.m_cycles > 143 {
            let vblank = mem.get_interrupt_registers() | 1;
            mem.set_interrupt_registers(vblank);
        }
        // reset scan lines, 
        if self.m_cycles > 153 {
            self.m_cycles = 0;
        }
    }
}
