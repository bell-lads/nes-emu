use bitflags::bitflags;

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub status_flags: StatusFlags,
}

bitflags! {
    pub struct StatusFlags: u8 {
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
