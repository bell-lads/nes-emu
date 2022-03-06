use bitflags::bitflags;

pub type Address = u16; //this might be moved to memory module once we have it

pub const ADDRESS: Address = 0x4016;

bitflags! {
    pub struct Button: u8 {
        const RIGHT     = 0b1000_0000;
        const LEFT      = 0b0100_0000;
        const DOWN      = 0b0010_0000;
        const UP        = 0b0001_0000;
        const START     = 0b0000_1000;
        const SELECT    = 0b0000_0100;
        const B         = 0b0000_0010;
        const A         = 0b0000_0001;
    }
}

// Order of reported buttons A, B, Select, Start, Up, Down, Left, Right
// read cylce of a joypad for CPU :
// Write 0x01 to joypad::ADDRESS (strobe mode on -> reset pointer to A)
// Write 0x00 to joypad::ADDRESS (strobe mode off, will cycle again through all buttons)
// Read from joypad::ADDRESS 8 times
// Repeat

pub struct Joypad {
    strobe_mode_on: bool,
    next_to_read: Button,
    return_1: bool,
    button_status: Button,
}

impl<'a> Joypad {
    pub fn new() -> Joypad {
        Joypad {
            strobe_mode_on: false,
            next_to_read: Button::A,
            return_1: false,
            button_status: Button::empty(),
        }
    }

    pub fn write(&mut self, byte: &u8) {
        //0X4016
        let first_bit_mask = 0b0000_0001;
        self.strobe_mode_on = (byte & first_bit_mask) == first_bit_mask;
        self.return_1 = false;
        if self.strobe_mode_on {
            self.next_to_read = Button::A
        }
    }

    pub fn read(&mut self, byte: &mut u8) {
        if self.return_1 {
            *byte = 1;
        }
        let res = self.button_status.contains(self.next_to_read) as u8;
        if !self.strobe_mode_on {
            if self.next_to_read != Button::RIGHT {
                self.roll_button();
            } else {
                self.return_1 = true;
            }
        }
        *byte = res;
    }

    pub fn press(&mut self, key: Button) {
        self.button_status |= key;
    }

    pub fn release(&mut self, key: Button) {
        self.button_status &= !key;
    }

    fn roll_button(&mut self) {
        self.next_to_read = Button::from_bits(self.next_to_read.bits() << 1).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn press_button() {
        let mut joypad_byte: u8 = 1;
        let mut joypad = Joypad::new();
        joypad.write(&joypad_byte);
        joypad.press(Button::A);
        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 1); //CPU get result
    }

    #[test]
    fn release_button() {
        let mut joypad_byte: u8 = 0;
        let mut joypad = Joypad::new();
        joypad.write(&joypad_byte);
        joypad.press(Button::A);
        joypad.release(Button::A);
        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 0);
    }

    #[test]
    fn test_strobe_mode() {
        let mut joypad_byte: u8 = 1;
        let mut joypad = Joypad::new();
        joypad.write(&joypad_byte);
        joypad.press(Button::A);
        for _x in 0..10 {
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 1);
        }
    }

    #[test]
    fn test_strobe_mode_on_off() {
        let mut joypad_byte: u8 = 0;
        let mut joypad = Joypad::new();
        joypad.write(&joypad_byte);
        joypad.press(Button::RIGHT);
        joypad.press(Button::LEFT);
        joypad.press(Button::SELECT);
        joypad.press(Button::B);

        for _ in 0..=1 {
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 0);
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 1);
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 1);
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 0);
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 0);
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 0);
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 1);
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 1);

            for _x in 0..10 {
                joypad.read(&mut joypad_byte);
                assert_eq!(joypad_byte, 1);
            }
            joypad_byte = 1;
            joypad.write(&joypad_byte);
            joypad_byte = 0;
            joypad.write(&joypad_byte);
        }
    }
}
