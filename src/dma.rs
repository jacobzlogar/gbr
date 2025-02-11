pub struct DmaBuffer {
    data: Vec<u8>,
}

pub struct DmaChannel {
    address: u16,
}

pub struct DmaController {
    source: DmaBuffer,
    dest: DmaBuffer,
    transfer_size: usize,
}

impl Default for DmaController {
    fn default() -> Self {
        Self {
            source: DmaBuffer { data: vec![] },
            dest: DmaBuffer { data: vec![] },
            transfer_size: 0,
        }
    }
}

impl DmaController {
    pub fn transfer(&mut self) {
        // Do DMA work here
    }
}
