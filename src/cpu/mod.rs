mod instruction;
mod register;

use crate::bus::Bus;

pub struct Cpu<'a> {
    counter: register::ProgramCounter,
    stack_pointer: register::StackPointer,
    a: register::A,
    x: register::X,
    y: register::Y,
    status: register::Status,
    bus: &'a mut Bus<'a>,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a mut Bus<'a>) -> Self {
        Self {
            counter: 0,
            stack_pointer: 0,
            a: 0,
            x: 0,
            y: 0,
            status: register::Status::INITIAL_STATE,
            bus,
        }
    }
}
