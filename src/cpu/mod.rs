mod instruction;
mod register;

use crate::traits::Memory;
use instruction::{AddressingMode, Instruction, InstructionKind};

const PROGRAM_POINTER: u16 = 0xFFFC;
const STACK_ADDR_HI: register::StackPointer = 0x01;
pub const STACK_TOP: register::StackPointer = 0xFF;
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
            stack_pointer: STACK_TOP,
            a: 0,
            x: 0,
            y: 0,
            status: register::Status::INITIAL_STATE,
            memory,
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[rustfmt::skip]
    pub unsafe fn run(&mut self) {
        self.counter = (*self.memory).mem_read_u16(PROGRAM_POINTER);
        loop {
            let opcode = (*self.memory).mem_read_u8(self.counter);
            let instruction = Instruction::from_opcode(opcode);
            if instruction.kind() == InstructionKind::Brk {
                self.brk();
                break
            }
            self.counter += 1;
            let previous_position = self.counter;
            let addr = self.get_operand_address(&instruction.addressing_mode());
            let operand = if addr != IMPLICIT_MODE_ADDR {
                (*self.memory).mem_read_u8(addr)
            } else {
                0
            };
            match instruction.kind() {
                InstructionKind::Adc => self.adc(operand),//tested
                InstructionKind::And => self.and(operand),//tested
                InstructionKind::Asl if instruction.addressing_mode() == AddressingMode::Accumulator
                                       => self.asl_a(),
                InstructionKind::Asl => self.asl(operand, addr),
                InstructionKind::Bit => self.bit(operand),
                InstructionKind::Bcc => self.bcc(addr),//tested
                InstructionKind::Bcs => self.bcs(addr),//tested
                InstructionKind::Beq => self.beq(addr),//tested
                InstructionKind::Bmi => self.bmi(addr),//tested
                InstructionKind::Bne => self.bne(addr),//tested
                InstructionKind::Bpl => self.bpl(addr),//tested
                InstructionKind::Bvc => self.bvc(addr),//tested
                InstructionKind::Bvs => self.bvs(addr),//tested
                InstructionKind::Clc => self.clc(),//tested
                InstructionKind::Cld => self.cld(),
                InstructionKind::Cli => self.cli(),
                InstructionKind::Clv => self.clv(),//tested
                InstructionKind::Cmp => self.cmp(operand),//tested
                InstructionKind::Cpx => self.cpx(operand),//tested 
                InstructionKind::Cpy => self.cpy(operand),//tested
                InstructionKind::Dec => self.dec(operand, addr),//tested
                InstructionKind::Dex => self.dex(),//tested
                InstructionKind::Dey => self.dey(),//tested
                InstructionKind::Eor => self.eor(operand), //tested
                InstructionKind::Inc => self.inc(operand, addr), //tested
                InstructionKind::Inx => self.inx(),//tested
                InstructionKind::Iny => self.iny(),//tested
                InstructionKind::Jmp => self.jmp(addr),
                InstructionKind::Jsr => self.jsr(addr),
                InstructionKind::Lda => self.lda(operand),//tested
                InstructionKind::Ldx => self.ldx(operand),//tested
                InstructionKind::Ldy => self.ldy(operand),//tested
                InstructionKind::Lsr if instruction.addressing_mode() == AddressingMode::Accumulator
                                       => self.lsr_a(),//tested
                InstructionKind::Lsr => self.lsr(operand, addr),//tested
                InstructionKind::Nop => self.nop(),
                InstructionKind::Ora => self.ora(operand),//tested
                InstructionKind::Pha => self.pha(), //tested
                InstructionKind::Php => self.php(),
                InstructionKind::Pla => self.pla(), //tested
                InstructionKind::Plp => self.plp(),
                InstructionKind::Rol if instruction.addressing_mode() == AddressingMode::Accumulator
                                       => self.rol_a(), //tested
                InstructionKind::Rol => self.rol(operand, addr),//tested
                InstructionKind::Ror if instruction.addressing_mode() == AddressingMode::Accumulator
                                       => self.ror_a(),//tested
                InstructionKind::Ror => self.ror(operand, addr),//tested
                InstructionKind::Rti => self.rti(),
                InstructionKind::Rts => self.rts(),
                InstructionKind::Sbc => self.sbc(operand),
                InstructionKind::Sec => self.sec(),//tested
                InstructionKind::Sed => self.sed(),
                InstructionKind::Sei => self.sei(),
                InstructionKind::Sta => self.sta(addr),//tested
                InstructionKind::Stx => self.stx(addr),//tested
                InstructionKind::Sty => self.sty(addr),//testes,
                InstructionKind::Tax => self.tax(),//tested
                InstructionKind::Tay => self.tay(),//tested
                InstructionKind::Tsx => self.tsx(),
                InstructionKind::Txa => self.txa(),//tested
                InstructionKind::Txs => self.txs(),
                InstructionKind::Tya => self.tya(),//tested
                _ => todo!()
            }
            if !self.has_branched(previous_position) {
                self.counter += instruction.addressing_mode().length_in_bytes();
            }
        }
    }

    unsafe fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute => (*self.memory).mem_read_u16(self.counter),
            AddressingMode::AbsoluteX => (*self.memory)
                .mem_read_u16(self.counter)
                .wrapping_add(self.x as u16),
            AddressingMode::AbsoluteY => (*self.memory)
                .mem_read_u16(self.counter)
                .wrapping_add(self.y as u16),
            AddressingMode::Indirect => {
                let addr = (*self.memory).mem_read_u16(self.counter);
                (*self.memory).mem_read_u16(addr)
            }
            AddressingMode::IndirectX => {
                let addr = (*self.memory)
                    .mem_read_u8(self.counter)
                    .wrapping_add(self.x);
                (*self.memory).mem_read_u16(addr as u16)
            }
            AddressingMode::IndirectY => {
                let addr =
                    (*self.memory).mem_read_u16((*self.memory).mem_read_u8(self.counter) as u16);
                addr.wrapping_add(self.y as u16)
            }
            AddressingMode::ZeroPage => (*self.memory).mem_read_u8(self.counter) as u16,
            AddressingMode::ZeroPageX => (*self.memory)
                .mem_read_u8(self.counter)
                .wrapping_add(self.x) as u16,
            AddressingMode::ZeroPageY => (*self.memory)
                .mem_read_u8(self.counter)
                .wrapping_add(self.y) as u16,
            AddressingMode::Immediate => self.counter,
            AddressingMode::Relative => {
                let offset = (*self.memory).mem_read_u8(self.counter) as i8;
                self.counter.wrapping_add(1).wrapping_add(offset as u16)
            }
            AddressingMode::Implicit => IMPLICIT_MODE_ADDR,
            AddressingMode::Accumulator => IMPLICIT_MODE_ADDR,
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

    unsafe fn brk(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.push_u16_on_stack(self.counter);
        self.push_u8_on_stack((self.status | register::Status::BREAK).bits());
        // IRQ interrput vector loaded in program counter
        // self.counter += 1;
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

    fn cmp(&mut self, operand: u8) {
        self.compare(self.a, operand);
    }

    fn cpx(&mut self, operand: u8) {
        self.compare(self.x, operand);
    }

    fn cpy(&mut self, operand: u8) {
        self.compare(self.y, operand);
    }

    unsafe fn dec(&mut self, operand: u8, addr: u16) {
        let val = operand.wrapping_sub(1);
        (*self.memory).mem_write_u8(addr, val);
        self.set_negative_and_zero_flags(val);
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1)
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1)
    }

    fn eor(&mut self, operand: u8) {
        self.a ^= operand;
        self.set_negative_and_zero_flags(self.a);
    }

    unsafe fn inc(&mut self, operand: u8, addr: u16) {
        let val = operand.wrapping_add(1);
        (*self.memory).mem_write_u8(addr, val);
        self.set_negative_and_zero_flags(val);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
    }

    fn jmp(&mut self, addr: u16) {
        self.branch(addr);
    }

    unsafe fn jsr(&mut self, addr: u16) {
        self.push_u16_on_stack(self.counter);
        self.branch(addr);
    }

    fn lda(&mut self, operand: u8) {
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

    unsafe fn lsr(&mut self, operand: u8, addr: u16) {
        self.status
            .set_or_unset_if(register::Status::CARRY, || operand & 1 == 1);
        let res = operand >> 1;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res);
    }

    fn lsr_a(&mut self) {
        self.status
            .set_or_unset_if(register::Status::CARRY, || self.a & 1 == 1);
        self.a >>= 1;
        self.set_negative_and_zero_flags(self.a);
    }

    fn nop(&mut self) {}

    fn ora(&mut self, operand: u8) {
        self.a |= operand;
        self.set_negative_and_zero_flags(self.a);
    }

    unsafe fn pha(&mut self) {
        self.push_u8_on_stack(self.a);
    }

    unsafe fn php(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.push_u8_on_stack((self.status | register::Status::BREAK).bits());
    }

    unsafe fn pla(&mut self) {
        self.a = self.pull_u8_from_stack();
    }

    unsafe fn plp(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.status = register::Status::from_bits(self.pull_u8_from_stack()).unwrap()
            & !register::Status::BREAK;
    }

    unsafe fn rol(&mut self, operand: u8, addr: u16) {
        let carry = if self.status.is_set(register::Status::CARRY) {
            1
        } else {
            0
        };
        self.status
            .set_or_unset_if(register::Status::CARRY, || operand >> 7 == 1);
        let res = (operand << 1) | carry;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res);
    }

    fn rol_a(&mut self) {
        let carry = if self.status.is_set(register::Status::CARRY) {
            1
        } else {
            0
        };
        self.status
            .set_or_unset_if(register::Status::CARRY, || self.a >> 7 == 1);
        self.a = (self.a << 1) | carry;
        self.set_negative_and_zero_flags(self.a)
    }

    unsafe fn ror(&mut self, operand: u8, addr: u16) {
        let carry = if self.status.is_set(register::Status::CARRY) {
            1
        } else {
            0
        };
        self.status
            .set_or_unset_if(register::Status::CARRY, || operand & 1 == 1);
        let res = operand >> 1 | carry << 7;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res)
    }

    fn ror_a(&mut self) {
        let carry = if self.status.is_set(register::Status::CARRY) {
            1
        } else {
            0
        };
        self.status
            .set_or_unset_if(register::Status::CARRY, || self.a & 1 == 1);
        self.a = self.a >> 1 | carry << 7;
        self.set_negative_and_zero_flags(self.a)
    }

    unsafe fn rti(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.status = register::Status::from_bits(self.pull_u8_from_stack()).unwrap()
            & !register::Status::BREAK;
        let addr = self.pull_u16_from_stack();
        self.branch(addr)
    }

    unsafe fn rts(&mut self) {
        let addr = self.pull_u16_from_stack().wrapping_add(1);
        self.branch(addr)
    }

    fn sbc(&mut self, operand: u8) {
        self.adc(operand.wrapping_neg().wrapping_sub(1)); // ? I kindof get the why But i don't really understand it ... oO?
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

    fn tax(&mut self) {
        self.x = self.a;
    }

    fn tay(&mut self) {
        self.y = self.a;
    }

    unsafe fn tsx(&mut self) {
        self.x = self.stack_pointer;
        self.set_negative_and_zero_flags(self.x)
    }

    fn txa(&mut self) {
        self.a = self.x;
    }

    unsafe fn txs(&mut self) {
        self.stack_pointer = self.x;
    }

    fn tya(&mut self) {
        self.a = self.y;
    }

    fn set_negative_and_zero_flags(&mut self, operation_res: u8) {
        self.status
            .set_or_unset_if(register::Status::NEGATIVE, || (operation_res as i8) < 0);
        self.status
            .set_or_unset_if(register::Status::ZERO, || operation_res == 0);
    }

    fn branch_if(&mut self, addr: u16, predicate: impl Fn(&register::Status) -> bool) {
        if predicate(&self.status) {
            self.branch(addr)
        }
    }

    fn branch(&mut self, addr: u16) {
        self.counter = addr;
    }

    fn has_branched(&self, previous_addr: u16) -> bool {
        previous_addr != self.counter
    }

    fn compare(&mut self, lhs: u8, rhs: u8) {
        let val = lhs.wrapping_sub(rhs);
        self.status
            .set_or_unset_if(register::Status::CARRY, || val as i8 >= 0);
        self.set_negative_and_zero_flags(val);
    }

    unsafe fn push_u8_on_stack(&mut self, byte: u8) {
        (*self.memory).mem_write_u8(self.get_stack_addr(), byte);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    unsafe fn push_u16_on_stack(&mut self, addr: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        (*self.memory).mem_write_u16(self.get_stack_addr(), addr);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    unsafe fn pull_u8_from_stack(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let res = (*self.memory).mem_read_u8(self.get_stack_addr());
        res
    }

    unsafe fn pull_u16_from_stack(&mut self) -> u16 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let res = (*self.memory).mem_read_u16(self.get_stack_addr());
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        res
    }

    pub fn get_stack_addr(&self) -> u16 {
        u16::from_le_bytes([self.stack_pointer, STACK_ADDR_HI])
    }
}

#[cfg(test)]
mod test;
