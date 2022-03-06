mod instruction;
mod register;

pub struct Cpu {
    counter: register::ProgramCounter,
    stack_pointer: register::StackPointer,
    a: register::A,
    x: register::X,
    y: register::Y,
    status: register::Status,
    memory: [u8; 0xFFFF],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            counter: 0,
            stack_pointer: 0,
            a: 0,
            x: 0,
            y: 0,
            status: register::Status::INITIAL_STATE,
            memory: [0; 0xFFFF],
        }
    }
}
