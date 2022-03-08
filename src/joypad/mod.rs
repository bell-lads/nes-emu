mod button;

use button::Button;
// Order of reported buttons A, B, Select, Start, Up, Down, Left, Right
// read cylce of a joypad for CPU :
// Write 0x01 to joypad::ADDRESS (strobe mode on -> reset pointer to A)
// Write 0x00 to joypad::ADDRESS (strobe mode off, will cycle again through all buttons)
// Read from joypad::ADDRESS 8 times
// Repeat

pub struct Joypad {
    strobe_mode_on: bool,
    button_reporter: button::Reporter,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            strobe_mode_on: false,
            button_reporter: button::Reporter::new(),
        }
    }

    pub fn write(&mut self, byte: &u8) {
        let first_bit_mask = 0b0000_0001;
        self.strobe_mode_on = (byte & first_bit_mask) == 1;
        if self.strobe_mode_on {
            self.button_reporter.reset_pointer();
        }
    }

    pub fn read(&mut self, byte: &mut u8) {
        self.button_reporter.read(byte);
        if !self.strobe_mode_on {
            self.button_reporter.roll();
        }
    }

    pub fn press(&mut self, key: Button) {
        self.button_reporter.press(key)
    }

    pub fn release(&mut self, key: Button) {
        self.button_reporter.release(key)
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
