pub trait Device {
    fn mapping_def(&self) -> std::ops::Range<usize>;

    fn map(&mut self, memory: &mut [u8]);

    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_read(&mut self);

    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_write(&mut self);
}

pub trait Memory {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn load(&mut self, data: &[u8], address: u16);

    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_read_u8(&mut self, address: u16) -> u8;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_write_u8(&mut self, address: u16, byte: u8);

    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_read_u16(&mut self, address: u16) -> u16 {
        let bytes = [self.mem_read_u8(address), self.mem_read_u8(address + 1)];
        u16::from_le_bytes(bytes)
    }

    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_write_u16(&mut self, address: u16, word: u16) {
        let [lo, hi] = word.to_le_bytes();
        self.mem_write_u8(address, lo);
        self.mem_write_u8(address + 1, hi)
    }
}
