#[derive(Debug)]
pub enum Interrupt {
    Joypad,
    Serial,
    Timer,
    Stat,
    VBlank,
}

impl Interrupt {
    pub fn get_interrupt(value: &u8) -> Option<Self> {
        match value {
            0x04 => Some(Interrupt::Joypad),
            0x03 => Some(Interrupt::Serial),
            0x02 => Some(Interrupt::Timer),
            0x01 => Some(Interrupt::Stat),
            0x00 => Some(Interrupt::VBlank),
            _ => None,
        }
    }
}
pub const TIMER: u8 = 0x02;
// pub const VBLANK: u8 = 0x00;
// pub const LCD: u8 = 0x02;
// pub const TIMER: u8 = 0x04;
// pub const SERIAL: u8 = 0x08;
// pub const JOYPAD: u8 = 0x10;
