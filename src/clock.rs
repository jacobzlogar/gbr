use crate::memory::Memory;

#[derive(Debug)]
pub struct Clock {
    pub master_clock: usize,
    pub m_cycles: usize,
    pub dots: usize,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            master_clock: 0,
            m_cycles: 0,
            dots: 0,
        }
    }
    pub fn tick(&mut self, mem: &mut Memory) {
        self.master_clock += 1;
        self.dots = self.m_cycles * 4;
        // a scanline has been completed, 456 dots per scanline
        if self.dots % 456 == 0 {
            mem.inc_scanline();
        }
        // a second should have elapsed
        // 70224 dots * 59.7 fps = ~4190000 (the clock speed of the system)
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
