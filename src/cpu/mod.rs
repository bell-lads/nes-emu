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
            let operand = if addr != IMPLICIT_MODE_ADDR { 
                (*self.memory).mem_read_u8(addr)
            } else {
                0
            };
            match instruct.name {
                instruction::Name::Adc => self.adc(operand),//tested
                instruction::Name::And => self.and(operand),//tested
                instruction::Name::Asl if instruct.mode == instruction::Mode::Accumulator => self.asl_a(),
                instruction::Name::Asl => self.asl(operand, addr),
                instruction::Name::Bit => self.bit(operand),
                instruction::Name::Bcc => self.bcc(addr),//tested
                instruction::Name::Bcs => self.bcs(addr),//tested
                instruction::Name::Beq => self.beq(addr),//tested
                instruction::Name::Bmi => self.bmi(addr),//tested
                instruction::Name::Bne => self.bne(addr),//tested
                instruction::Name::Bpl => self.bpl(addr),//tested
                instruction::Name::Bvc => self.bvc(addr),//tested
                instruction::Name::Bvs => self.bvs(addr),//tested
                instruction::Name::Clc => self.clc(),//tested
                instruction::Name::Cld => self.cld(),
                instruction::Name::Cli => self.cli(),
                instruction::Name::Clv => self.clv(),//tested
                instruction::Name::Lda => self.lda(operand),//tested
                instruction::Name::Ldx => self.ldx(operand),//tested
                instruction::Name::Ldy => self.ldy(operand),//tested
                instruction::Name::Sec => self.sec(),//tested
                instruction::Name::Sed => self.sed(),
                instruction::Name::Sei => self.sei(),
                instruction::Name::Sta => self.sta(addr),//tested
                instruction::Name::Stx => self.stx(addr),//tested
                instruction::Name::Sty => self.sty(addr),//testes,
                _ => todo!()
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
            instruction::Mode::Accumulator => u16::MAX,
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

    fn and(&mut self, value: u8) {
        self.a &= value;
        self.set_negative_and_zero_flags(self.a);
    }

    unsafe fn asl(&mut self, operand: u8, addr: u16) {
        self.status
            .set_or_unset_if(register::Status::CARRY, || operand >> 7 == 1);
        let res = operand << 1;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res)
    }

    fn asl_a(&mut self) {
        self.status
            .set_or_unset_if(register::Status::CARRY, || self.a >> 7 == 1);
        self.a <<= 1;
        self.set_negative_and_zero_flags(self.a);
    }

    #[allow(clippy::bad_bit_mask)]
    fn bit(&mut self, operand: u8) {
        self.status
            .set_or_unset_if(register::Status::ZERO, || self.a & operand == 0);
        self.status
            .set_or_unset_if(register::Status::NEGATIVE, || operand & 0b1000_0000 == 1);
        self.status
            .set_or_unset_if(register::Status::OVERFLOW, || operand & 0b0100_0000 == 1);
    }

    fn bcc(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_unset(register::Status::CARRY))
    }

    fn bcs(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_set(register::Status::CARRY))
    }

    fn beq(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_set(register::Status::ZERO))
    }

    fn bmi(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_set(register::Status::NEGATIVE))
    }

    fn bne(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_unset(register::Status::ZERO))
    }

    fn bpl(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_unset(register::Status::NEGATIVE))
    }

    fn brk(&mut self){
        //handle non maskable interrupt ??
        self.counter += 1;
    }

    fn bvc(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_unset(register::Status::OVERFLOW))
    }

    fn bvs(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.is_set(register::Status::OVERFLOW))
    }

    fn clc(&mut self) {
        self.status.remove(register::Status::CARRY)
    }

    fn cld(&mut self) {
        self.status.remove(register::Status::DECIMAL)
    }

    fn cli(&mut self) {
        self.status.remove(register::Status::INTERRUPT_DISABLE)
    }

    fn clv(&mut self) {
        self.status.remove(register::Status::OVERFLOW)
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

    fn ldy(&mut self, operand: u8) {
        self.y = operand;
        self.set_negative_and_zero_flags(self.y);
    }

    fn sec(&mut self) {
        self.status.insert(register::Status::CARRY)
    }

    fn sed(&mut self) {
        self.status.insert(register::Status::DECIMAL)
    }

    fn sei(&mut self) {
        self.status.insert(register::Status::INTERRUPT_DISABLE)
    }

    unsafe fn sta(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.a)
    }

    unsafe fn stx(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.x)
    }

    unsafe fn sty(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.y)
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

#[cfg(test)]
mod test;
