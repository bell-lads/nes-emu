use crate::traits::{Device, Memory};

const RAM_MIRRORING_MASK: u16 = 0b0000_0111_1111_1111;
const PPU_REGISTERS_MIRRORING_MASK: u16 = 0b0010_0000_0000_0111;

enum RunMode {
    Read,
    Write,
}

pub struct Bus {
    memory: *mut [u8; 0xFFFF],
    devices: Vec<(std::ops::Range<usize>, *mut dyn Device)>,
}

impl Bus {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            memory: std::ptr::null_mut(),
            devices: Vec::new(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn map(&mut self, memory: &mut [u8; 0xFFFF], devices: *const [*mut dyn Device]) {
        self.memory = memory;
        for device in (*devices).iter() {
            let mapping = (**device).mapping_def();
            (**device).map(&mut (*self.memory)[mapping.clone()]);
            self.devices.push((mapping, *device));
        }
    }

    fn run_device(&mut self, address: u16, run_mode: RunMode) {
        for (range, device) in self.devices.iter() {
            if range.contains(&usize::from(address)) {
                unsafe {
                    match run_mode {
                        RunMode::Read => (**device).mem_read(),
                        RunMode::Write => (**device).mem_write(),
                    }
                }
                break;
            }
        }
    }
}

impl Memory for Bus {
    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_read_u8(&mut self, addr: u16) -> u8 {
        let addr = mirror_address(addr);
        self.run_device(addr, RunMode::Write);
        (*self.memory)[usize::from(addr)]
    }

    #[allow(clippy::missing_safety_doc)]
    unsafe fn mem_write_u8(&mut self, addr: u16, data: u8) {
        let addr = mirror_address(addr);
        (*self.memory)[usize::from(addr)] = data;
        self.run_device(addr, RunMode::Read);
    }
}

fn mirror_address(addr: u16) -> u16 {
    match addr {
        0x0000..=0x1FFF => addr & RAM_MIRRORING_MASK,
        0x2000..=0x3FFF => addr & PPU_REGISTERS_MIRRORING_MASK,
        _ => addr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDevice {
        start: usize,
        memory: [*mut u8; 2],
    }

    impl MockDevice {
        fn new(start: usize) -> Self {
            Self {
                start,
                memory: [std::ptr::null_mut(); 2],
            }
        }
    }

    impl Device for MockDevice {
        fn mapping_def(&self) -> std::ops::Range<usize> {
            self.start..self.start + 2
        }

        fn map(&mut self, memory: &mut [u8]) {
            for (count, pointer) in self.memory.iter_mut().enumerate() {
                *pointer = &mut memory[count];
            }
        }

        #[allow(clippy::missing_safety_doc)]
        unsafe fn mem_read(&mut self) {
            for (count, i) in (self.start..self.start + 2).into_iter().enumerate() {
                (*self.memory[count]) = (i + 1) as u8;
            }
        }

        #[allow(clippy::missing_safety_doc)]
        unsafe fn mem_write(&mut self) {
            for (count, i) in (self.start..self.start + 2).into_iter().enumerate() {
                let val = (2 * i + 1) as u8;
                (*self.memory[count]) = val;
            }
        }
    }

    #[test]
    fn test_bus_mapping_read() {
        let mut memory = [0; 0xFFFF];
        let mut expected = [0; 0xFFFF];
        memory[0] = 99;
        memory[1] = 99;
        memory[10] = 99;
        memory[11] = 99;
        expected[0] = 1;
        expected[1] = 3;
        expected[10] = 21;
        expected[11] = 23;

        for i in 0..=1u8 {
            let mut device_1 = MockDevice::new(0);
            let mut device_2 = MockDevice::new(10);
            let mut bus = Bus::new();
            unsafe {
                memory[0] = 10;
                bus.map(&mut memory, &[&mut device_1, &mut device_2]);
                bus.mem_read_u8(i as u16);
                bus.mem_read_u8((i + 10) as u16);
            }
        }
        assert_eq!(expected, memory);
    }

    #[test]
    fn test_bus_mapping_write() {
        let mut memory = [0; 0xFFFF];
        let mut expected = [0; 0xFFFF];
        memory[5] = 99;
        memory[6] = 99;
        memory[15] = 99;
        memory[16] = 99;
        expected[5] = 6;
        expected[6] = 7;
        expected[15] = 16;
        expected[16] = 17;

        for i in 0..=1u8 {
            let mut device_1 = MockDevice::new(5);
            let mut device_2 = MockDevice::new(15);
            let mut bus = Bus::new();
            unsafe {
                bus.map(&mut memory, &[&mut device_1, &mut device_2]);
                bus.mem_write_u8((i + 5) as u16, 0);
                bus.mem_write_u8((i + 15) as u16, 0);
            }
        }
        assert_eq!(expected, memory);
    }
}
