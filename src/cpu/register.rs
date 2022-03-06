use bitflags::bitflags;

pub type ProgramCounter = u16;
pub type A = u8;
pub type X = u8;
pub type Y = u8;
pub type StackPointer = u8;

const PROGRAM_COUNTER_START: ProgramCounter = 0xFFFC;

bitflags! {
    pub struct Status: u8 {
        const NEGATIVE =            0b1000_0000;
        const OVERFLOW =            0b0100_0000;
        const UNUSED =              0b0010_0000;
        const BREAK =               0b0001_0000;
        const DECIMAL =             0b0000_1000;
        const INTERRUPT_DISABLE =   0b0000_0100;
        const ZERO =                0b0000_0010;
        const CARRY =               0b0000_0001;
        const INITIAL_STATE = Self::INTERRUPT_DISABLE.bits() | Self::UNUSED.bits();
    }
}

impl Status {
    pub fn is_set(&self, flag: Status) -> bool {
        self.contains(flag)
    }

    pub fn is_unset(&self, flag: Status) -> bool {
        !self.is_set(flag)
    }
}
