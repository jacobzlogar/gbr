#[derive(Debug)]
pub struct Apu {}

impl Apu {
    pub fn process(&mut self) {}
}

impl Default for Apu {
    fn default() -> Self {
        Self {}
    }
}
