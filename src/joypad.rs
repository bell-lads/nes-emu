use bitflags::bitflags;

bitflags! {
    pub struct Button: u8 {
        const RIGHT    = 0b10000000;
        const LEFT     = 0b01000000;
        const DOWN     = 0b00100000;
        const UP       = 0b00010000;
        const START    = 0b00001000;
        const SELECT   = 0b00000100;
        const B        = 0b00000010;
        const A        = 0b00000001;
    }
}

pub struct Joypad {
    is_strobe_on: bool,
    current_button_mask: Button,
    button_status: Button,
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            is_strobe_on: false,
            current_button_mask: Button::A,
            button_status: Button::from_bits_truncate(0),
        }
    }

    pub fn write(&mut self, data: &u8) {
        self.is_strobe_on = data & 1 == 1;
        if self.is_strobe_on {
            self.current_button_mask = Button::A
        }
    }

    pub fn read(&mut self, data: &mut u8) {
        if self.current_button_mask.is_empty() {
            *data = 1;
            return;
        }
        *data = u8::from(self.button_status.contains(self.current_button_mask));
        if !self.is_strobe_on {
            self.current_button_mask.bits <<= 1;
        }
    }

    pub fn press(&mut self, button: Button) {
        self.button_status |= button;
    }

    pub fn release(&mut self, button: Button) {
        self.button_status &= !button;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_press_button_a() {
        let mut joypad = Joypad::new();
        joypad.press(Button::A);

        let mut joypad_byte = 0;
        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 1);
    }

    #[test]
    fn test_release_button_a() {
        let mut joypad = Joypad::new();
        joypad.press(Button::A);
        joypad.release(Button::A);
        
        let mut joypad_byte = 0;
        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 0);
    }

    #[test]
    fn test_button_index_reset() {
        let mut joypad = Joypad::new();
        joypad.press(Button::A);
        
        let mut joypad_byte = 0;
        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 1);
        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 0);

        joypad.write(&1);
        joypad.write(&0);

        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 1);
        joypad.read(&mut joypad_byte);
        assert_eq!(joypad_byte, 0);
    }

    #[test]
    fn test_reading_when_strobe_off() {
        let mut joypad = Joypad::new();
        joypad.press(Button::A);
        joypad.press(Button::SELECT);
        joypad.press(Button::UP);

        let mut joypad_byte: u8 = 0;
        let expected_results = [1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1];
        for result in expected_results {
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, result);
        }
    }

    #[test]
    fn test_reading_when_strobe_on() {
        let mut joypad = Joypad::new();
        joypad.write(&1);
        joypad.press(Button::A);

        let mut joypad_byte: u8 = 0;
        for _ in 0..3 {
            joypad.read(&mut joypad_byte);
            assert_eq!(joypad_byte, 1);
        }
    }
}