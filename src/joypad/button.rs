use bitflags::bitflags;

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

pub struct State {
    button_to_read: Option<Button>,
    buttons_pressed: Button,
}

impl State {
    pub fn new() -> Self {
        Self {
            button_to_read: Some(Button::A),
            buttons_pressed: Button::empty(),
        }
    }

    pub fn roll(&mut self) {
        if let Some(b_ref) = self.button_to_read.as_mut() {
            if *b_ref != Button::RIGHT {
                self.button_to_read = Some(Button::from_bits(b_ref.bits() << 1).unwrap())
            } else {
                self.button_to_read = None
            }
        }
    }

    pub fn reset_pointer(&mut self) {
        self.button_to_read = Some(Button::A)
    }

    pub fn read(&self, byte: &mut u8) {
        if let Some(current) = self.button_to_read {
            *byte = self.buttons_pressed.contains(current) as u8
        } else {
            *byte = 1
        }
    }

    pub fn press(&mut self, key: Button) {
        self.buttons_pressed |= key;
    }

    pub fn release(&mut self, key: Button) {
        self.buttons_pressed &= !key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_button() {
        let mut b = State::new();
        let mut byte: u8 = 0;
        b.press(Button::A);
        b.read(&mut byte);
        assert_eq!(byte, 1);
        b.roll();
        b.read(&mut byte);
        assert_eq!(byte, 0);
        b.reset_pointer();
        b.read(&mut byte);
        assert_eq!(byte, 1);
    }

    #[test]
    fn roll_button() {
        let mut b = State::new();
        b.press(Button::A);
        b.press(Button::SELECT);
        b.press(Button::UP);
        b.press(Button::LEFT);
        for _ in 0..=1 {
            let mut byte: u8 = 0;
            b.reset_pointer();
            b.read(&mut byte);
            assert_eq!(byte, 1);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 0);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 1);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 0);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 1);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 0);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 1);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 0);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 1);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 1);
            b.roll();
            b.read(&mut byte);
            assert_eq!(byte, 1);
        }
    }
}
