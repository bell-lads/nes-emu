use crate::joypad::Joypad;

const RAM_MIRRORING_MASK: u16 = 0b0000_0111_1111_1111;
const PPU_REGISTERS_MIRRORING_MASK: u16 = 0b0010_0000_0000_0111;

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

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn mem_read_u8(&mut self, addr: u16) -> u8 {
        let addr = mirror_address(addr);
        match addr {
            0x4016 => self.joypad_1.read(),
            0x4017 => self.joypad_2.read(),
            _ => todo!(),
        }
        self.memory[usize::from(addr)]
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn mem_write_u8(&mut self, addr: u16, data: u8) {
        let addr = mirror_address(addr);
        self.memory[usize::from(addr)] = data;
        match addr {
            0x4016 => self.joypad_1.write(),
            0x4017 => self.joypad_2.write(),
            _ => todo!(),
        }
    }
}

fn mirror_address(addr: u16) -> u16 {
    match addr {
        0x0000..=0x1FFF => addr & RAM_MIRRORING_MASK,
        0x2000..=0x3FFF => addr & PPU_REGISTERS_MIRRORING_MASK,
        _ => addr
    }
}