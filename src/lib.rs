mod bus;
pub mod cpu;
mod joypad;
pub mod traits;
use bus::Bus;
use cpu::Cpu;
use joypad::Joypad;
use std::{marker::PhantomPinned, pin::Pin, ptr};

pub struct Nes {
    memory: [u8; 0xFFFF],
    joypad_1: Joypad,
    joypad_2: Joypad,
    bus: Bus,
    cpu: Cpu,
    _pin: PhantomPinned,
}

impl Nes {
    pub fn new() -> Pin<Box<Self>> {
        let nes = Self {
            memory: [0; 0xFFFF],
            joypad_1: Joypad::new(0x4016),
            joypad_2: Joypad::new(0x4017),
            bus: Bus::new(),
            cpu: Cpu::new(ptr::null_mut::<Bus>()),
            _pin: PhantomPinned,
        };

        let mut pinned_boxed_nes = Box::pin(nes);
        unsafe {
            pinned_boxed_nes.map_devices();
        }
        pinned_boxed_nes
    }

    unsafe fn map_devices(self: &mut Pin<Box<Self>>) {
        let pinned_nes_ref: Pin<&mut Self> = Pin::as_mut(self);
        let nes_ref: &mut Self = Pin::get_unchecked_mut(pinned_nes_ref);
        nes_ref.bus.map(
            &mut nes_ref.memory,
            &[&mut nes_ref.joypad_1, &mut nes_ref.joypad_2],
        )
    }
}
