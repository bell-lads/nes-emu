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
    memory: *mut u8,
}

impl Joypad {
    pub fn new(memory: *mut u8) -> Self {
        Self {
            is_strobe_on: false,
            current_button_mask: Button::A,
            button_status: Button::from_bits_truncate(0),
            memory,
        }
    }

    pub fn map_memory(&mut self, memory: *mut u8) {
        self.memory = memory;
    }

    /// # Safety
    /// Make sure that `memory` ptr is valid
    pub unsafe fn read_mem(&mut self) {
        self.is_strobe_on = *self.memory & 1 == 1;
        if self.is_strobe_on {
            self.current_button_mask = Button::A
        }
    }

    /// # Safety
    /// Make sure that `memory` ptr is valid
    pub unsafe fn write_mem(&mut self) {
        if self.current_button_mask.is_empty() {
            *self.memory = 1;
            return;
        }
        *self.memory = u8::from(self.button_status.contains(self.current_button_mask));
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
        let mut joypad_byte = 0;
        let mut joypad = Joypad::new(&mut joypad_byte);
        joypad.press(Button::A);
        unsafe {
            joypad.write_mem();
        }
        assert_eq!(joypad_byte, 1);
    }

    #[test]
    fn test_release_button_a() {
        let mut joypad_byte = 0;
        let mut joypad = Joypad::new(&mut joypad_byte);
        joypad.press(Button::A);
        joypad.release(Button::A);
        unsafe {
            joypad.write_mem();
        }
        assert_eq!(joypad_byte, 0);
    }

    #[test]
    fn test_button_index_reset() {
        let mut joypad_byte = 0;
        let mut joypad = Joypad::new(&mut joypad_byte);
        joypad.press(Button::A);
        unsafe {
            joypad.write_mem();
            assert_eq!(joypad_byte, 1);
            joypad.write_mem();
            assert_eq!(joypad_byte, 0);

            std::ptr::write_volatile(&mut joypad_byte, 1);
            joypad.read_mem();
            std::ptr::write_volatile(&mut joypad_byte, 0);
            joypad.read_mem();

            joypad.write_mem();
            assert_eq!(joypad_byte, 1);
            joypad.write_mem();
            assert_eq!(joypad_byte, 0);
        }
    }

    #[test]
    fn test_reading_when_strobe_off() {
        let mut joypad_byte = 0;
        let mut joypad = Joypad::new(&mut joypad_byte);
        joypad.press(Button::A);
        joypad.press(Button::SELECT);
        joypad.press(Button::UP);

        let expected_results = [1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1];
        for result in expected_results {
            unsafe {
                joypad.write_mem();
            }
            assert_eq!(joypad_byte, result);
        }
    }

    #[test]
    fn test_reading_when_strobe_on() {
        let mut joypad_byte = 0;
        let mut joypad = Joypad::new(&mut joypad_byte);
        unsafe {
            std::ptr::write_volatile(&mut joypad_byte, 1);
            joypad.read_mem();
        }
        joypad.press(Button::A);

        for _ in 0..3 {
            unsafe {
                joypad.write_mem();
            }
            assert_eq!(joypad_byte, 1);
        }
    }
}
