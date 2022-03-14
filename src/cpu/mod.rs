mod instruction;
mod register;

use instruction::INSTRUCTION_MAP;

use crate::traits::Memory;

const PROGRAM_POINTER: u16 = 0xFFFC;

const IMPLICIT_MODE_ADDR: u16 = u16::MAX;

pub struct Cpu {
    counter: register::ProgramCounter,
    stack_pointer: register::StackPointer,
    a: register::A,
    x: register::X,
    y: register::Y,
    status: register::Status,
    memory: *mut dyn Memory,
}

impl Cpu {
    pub fn new(memory: *mut dyn Memory) -> Self {
        Self {
            counter: 0,
            stack_pointer: 0,
            a: 0,
            x: 0,
            y: 0,
            status: register::Status::INITIAL_STATE,
            memory,
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[rustfmt::skip]
    pub unsafe fn run(&mut self)
    {
        self.counter = (*self.memory).mem_read_u16(PROGRAM_POINTER);
        loop {
            let instruct = INSTRUCTION_MAP.get(&(*self.memory).mem_read_u8(self.counter)).unwrap();
            if instruct.opcode == 0 {
                self.brk();
                break
            }
            self.counter += 1;
            let previous_position = self.counter;
            println!("{:?}", instruct);
            let addr = self.get_operand_address(&instruct.mode);
            let  operand = if addr != IMPLICIT_MODE_ADDR { 
                (*self.memory).mem_read_u8(addr)
            } else {
                0
            };
            match instruct.opcode {
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71   => self.adc(operand),//tested
                0x06 | 0x16 | 0x0E | 0x1E                               => self.asl(operand, addr),
                0x90                                                    => self.bcc(addr),//tested
                0x18                                                    => self.clc(),//tested
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 |0xB1    => self.lda(operand),//tested
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE                        => self.ldx(operand),//tested
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91          => self.sta(addr),//tested
                0x86 | 0x96 | 0x8E                                      => self.stx(addr),//tested
                _ => panic!("Cpu :: Unexpected opcode : {}", instruct.opcode)
            }
            if !self.has_branched(previous_position) {
                self.counter += u16::from(instruct.len - 1);
            }
        }
    }

    unsafe fn get_operand_address(&mut self, mode: &instruction::Mode) -> u16 {
        match mode {
            instruction::Mode::Absolute => (*self.memory).mem_read_u16(self.counter),
            instruction::Mode::AbsoluteX => (*self.memory)
                .mem_read_u16(self.counter)
                .wrapping_add(self.x as u16),
            instruction::Mode::AbsoluteY => (*self.memory)
                .mem_read_u16(self.counter)
                .wrapping_add(self.y as u16),
            instruction::Mode::Indirect => {
                let addr = (*self.memory).mem_read_u16(self.counter);
                (*self.memory).mem_read_u16(addr)
            }
            instruction::Mode::IndirectX => {
                let addr = (*self.memory)
                    .mem_read_u8(self.counter)
                    .wrapping_add(self.x);
                (*self.memory).mem_read_u16(addr as u16)
            }
            instruction::Mode::IndirectY => {
                let addr =
                    (*self.memory).mem_read_u16((*self.memory).mem_read_u8(self.counter) as u16);
                addr.wrapping_add(self.y as u16)
            }
            instruction::Mode::ZeroPage => (*self.memory).mem_read_u8(self.counter) as u16,
            instruction::Mode::ZeroPageX => (*self.memory)
                .mem_read_u8(self.counter)
                .wrapping_add(self.x) as u16,
            instruction::Mode::ZeroPageY => (*self.memory)
                .mem_read_u8(self.counter)
                .wrapping_add(self.y) as u16,
            instruction::Mode::Immediate => self.counter,
            instruction::Mode::Relative => {
                let offset = (*self.memory).mem_read_u8(self.counter) as i8;
                self.counter.wrapping_add(1).wrapping_add(offset as u16)
            }
            instruction::Mode::Implicit => u16::MAX,
            _ => panic!("Cpu :: Unexpected mode : {:?}", mode),
        }
    }

    fn adc(&mut self, operand: u8) {
        // no decimal mode.
        let sum = self.a as u16
            + operand as u16
            + if self.status.is_set(register::Status::CARRY) {
                1
            } else {
                0
            };
        self.status
            .set_or_unset_if(register::Status::CARRY, || sum > 0xFF);
        let res = sum as u8;
        //https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.status.set_or_unset_if(register::Status::OVERFLOW, || {
            (self.a ^ res) & (operand ^ res) & 0x80 != 0
        });
        self.a = res;
        self.set_negative_and_zero_flags(res)
    }

    unsafe fn asl(&mut self, operand: u8, addr: u16) {
        self.status
            .set_or_unset_if(register::Status::CARRY, || operand >> 7 == 1);
        let res = operand << 1;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res)
    }

    fn bcc(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_unset(register::Status::CARRY))
    }

    fn beq(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_set(register::Status::ZERO))
    }

    fn brk(&mut self){
        //handle non maskable interrupt ??
        self.counter += 1;
    }

    fn clc(&mut self) {
        self.status.remove(register::Status::CARRY)
    }

    fn lda(&mut self, operand: u8) {
        println!("lda : operand : {operand}");
        self.a = operand;
        self.set_negative_and_zero_flags(self.a);
    }

    fn ldx(&mut self, operand: u8) {
        self.x = operand;
        self.set_negative_and_zero_flags(self.x);
    }

    unsafe fn sta(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.a)
    }

    unsafe fn stx(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.x)
    }

    fn set_negative_and_zero_flags(&mut self, operation_res: u8) {
        self.status
            .set_or_unset_if(register::Status::NEGATIVE, || (operation_res as i8) < 0);
        self.status
            .set_or_unset_if(register::Status::ZERO, || operation_res == 0);
    }

    fn branch_if(&mut self, addr: u16, predicate: impl Fn(&register::Status) -> bool) {
        if predicate(&self.status) {
            self.counter = addr
        }
    }

    fn has_branched(&self, previous_addr: u16) -> bool {
        previous_addr != self.counter
    }
}
