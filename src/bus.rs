use crate::joypad::Joypad;

pub struct Bus<'a> {
    memory: &'a mut [u8; 0xFFFF],
    joypad_1: &'a mut Joypad,
    joypad_2: &'a mut Joypad,
}

impl<'a> Bus<'a> {
    pub fn new(
        memory: &'a mut [u8; 0xFFFF],
        joypad_1: &'a mut Joypad,
        joypad_2: &'a mut Joypad,
    ) -> Self {
        Self {
            memory,
            joypad_1,
            joypad_2,
        }
    }

    pub fn mem_read_u8(&mut self, addr: u16) -> u8 {
        match addr {
            0x4016 => unsafe { self.joypad_1.read() },
            0x4017 => unsafe { self.joypad_2.read() },
            _ => todo!(),
        }
        self.memory[usize::from(addr)]
    }

    pub fn mem_write_u8(&mut self, addr: u16, data: u8) {
        self.memory[usize::from(addr)] = data;

        match addr {
            0x4016 => unsafe { self.joypad_1.write() },
            0x4017 => unsafe { self.joypad_2.write() },
            _ => todo!(),
        }
    }
}
