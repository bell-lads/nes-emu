pub mod bus;
pub mod cpu;
pub mod joypad;

use bus::Bus;
use cpu::Cpu;
use joypad::Joypad;
use std::{ptr, pin::Pin, marker::PhantomPinned};

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
            joypad_1: Joypad::new(ptr::null_mut()),
            joypad_2: Joypad::new(ptr::null_mut()),
            bus: Bus::new(ptr::null_mut(), ptr::null_mut(), ptr::null_mut()),
            cpu: Cpu::new(ptr::null_mut()),
            _pin: PhantomPinned
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

        nes_ref.joypad_1.map_memory(&mut nes_ref.memory[0x4016]);
        nes_ref.joypad_2.map_memory(&mut nes_ref.memory[0x4017]);

        nes_ref.bus.joypad_1 = &mut nes_ref.joypad_1;
        nes_ref.bus.joypad_2 = &mut nes_ref.joypad_2;
        
        nes_ref.cpu.bus = &mut nes_ref.bus;
    }
}
