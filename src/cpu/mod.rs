mod instruction;
mod registers;

use crate::traits::Memory;
use instruction::{AddressingMode, Instruction, InstructionKind};
use registers::{Registers, StatusFlags};

const PROGRAM_POINTER: u16 = 0xFFFC;
const STACK_ADDR_HI: u8 = 0x01;
pub const STACK_TOP: u8 = 0xFF;
const IMPLICIT_MODE_ADDR: u16 = u16::MAX;

pub struct Cpu {
    registers: Registers,
    memory: *mut dyn Memory,
}

impl Cpu {
    pub fn new(memory: *mut dyn Memory) -> Self {
        Self {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                program_counter: 0,
                stack_pointer: STACK_TOP,
                status_flags: StatusFlags::INITIAL_STATE,
            },
            memory,
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[rustfmt::skip]
    pub unsafe fn run(&mut self) {
        self.registers.program_counter = (*self.memory).mem_read_u16(PROGRAM_POINTER);
        loop {
            let opcode = (*self.memory).mem_read_u8(self.registers.program_counter);
            let instruction = Instruction::from_opcode(opcode);
            if instruction.kind() == InstructionKind::Brk {
                self.brk();
                break
            }
            self.registers.program_counter += 1;
            let previous_position = self.registers.program_counter;
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
                self.registers.program_counter += instruction.addressing_mode().length_in_bytes();
            }
        }
    }

    unsafe fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute => (*self.memory).mem_read_u16(self.registers.program_counter),
            AddressingMode::AbsoluteX => (*self.memory)
                .mem_read_u16(self.registers.program_counter)
                .wrapping_add(self.registers.x as u16),
            AddressingMode::AbsoluteY => (*self.memory)
                .mem_read_u16(self.registers.program_counter)
                .wrapping_add(self.registers.y as u16),
            AddressingMode::Indirect => {
                let addr = (*self.memory).mem_read_u16(self.registers.program_counter);
                (*self.memory).mem_read_u16(addr)
            }
            AddressingMode::IndirectX => {
                let addr = (*self.memory)
                    .mem_read_u8(self.registers.program_counter)
                    .wrapping_add(self.registers.x);
                (*self.memory).mem_read_u16(addr as u16)
            }
            AddressingMode::IndirectY => {
                let addr =
                    (*self.memory).mem_read_u16(
                        (*self.memory).mem_read_u8(self.registers.program_counter) as u16,
                    );
                addr.wrapping_add(self.registers.y as u16)
            }
            AddressingMode::ZeroPage => {
                (*self.memory).mem_read_u8(self.registers.program_counter) as u16
            }
            AddressingMode::ZeroPageX => (*self.memory)
                .mem_read_u8(self.registers.program_counter)
                .wrapping_add(self.registers.x) as u16,
            AddressingMode::ZeroPageY => (*self.memory)
                .mem_read_u8(self.registers.program_counter)
                .wrapping_add(self.registers.y) as u16,
            AddressingMode::Immediate => self.registers.program_counter,
            AddressingMode::Relative => {
                let offset = (*self.memory).mem_read_u8(self.registers.program_counter) as i8;
                self.registers
                    .program_counter
                    .wrapping_add(1)
                    .wrapping_add(offset as u16)
            }
            AddressingMode::Implicit => IMPLICIT_MODE_ADDR,
            AddressingMode::Accumulator => IMPLICIT_MODE_ADDR,
        }
    }

    fn adc(&mut self, operand: u8) {
        // no decimal mode.
        let sum = self.registers.a as u16
            + operand as u16
            + if self.registers.status_flags.contains(StatusFlags::CARRY) {
                1
            } else {
                0
            };
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, sum > 0xFF);
        let res = sum as u8;
        //https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.registers.status_flags.set(
            StatusFlags::OVERFLOW,
            (self.registers.a ^ res) & (operand ^ res) & 0x80 != 0,
        );
        self.registers.a = res;
        self.set_negative_and_zero_flags(res)
    }

    fn and(&mut self, value: u8) {
        self.registers.a &= value;
        self.set_negative_and_zero_flags(self.registers.a);
    }

    unsafe fn asl(&mut self, operand: u8, addr: u16) {
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, operand >> 7 == 1);
        let res = operand << 1;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res)
    }

    fn asl_a(&mut self) {
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, self.registers.a >> 7 == 1);
        self.registers.a <<= 1;
        self.set_negative_and_zero_flags(self.registers.a);
    }

    #[allow(clippy::bad_bit_mask)]
    fn bit(&mut self, operand: u8) {
        self.registers
            .status_flags
            .set(StatusFlags::ZERO, self.registers.a & operand == 0);
        self.registers
            .status_flags
            .set(StatusFlags::NEGATIVE, operand & 0b1000_0000 == 1);
        self.registers
            .status_flags
            .set(StatusFlags::OVERFLOW, operand & 0b0100_0000 == 1);
    }

    fn bcc(&mut self, addr: u16) {
        self.branch_if(addr, |status| !status.contains(StatusFlags::CARRY))
    }

    fn bcs(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.contains(StatusFlags::CARRY))
    }

    fn beq(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.contains(StatusFlags::ZERO))
    }

    fn bmi(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.contains(StatusFlags::NEGATIVE))
    }

    fn bne(&mut self, addr: u16) {
        self.branch_if(addr, |status| !status.contains(StatusFlags::ZERO))
    }

    fn bpl(&mut self, addr: u16) {
        self.branch_if(addr, |status| !status.contains(StatusFlags::NEGATIVE))
    }

    unsafe fn brk(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.push_u16_on_stack(self.registers.program_counter);
        self.push_u8_on_stack((self.registers.status_flags | StatusFlags::BREAK).bits());
        // IRQ interrput vector loaded in program counter
        // self.registers.program_counter += 1;
    }

    fn bvc(&mut self, addr: u16) {
        self.branch_if(addr, |status| !status.contains(StatusFlags::OVERFLOW))
    }

    fn bvs(&mut self, addr: u16) {
        self.branch_if(addr, |status| status.contains(StatusFlags::OVERFLOW))
    }

    fn clc(&mut self) {
        self.registers.status_flags.remove(StatusFlags::CARRY)
    }

    fn cld(&mut self) {
        self.registers.status_flags.remove(StatusFlags::DECIMAL)
    }

    fn cli(&mut self) {
        self.registers
            .status_flags
            .remove(StatusFlags::INTERRUPT_DISABLE)
    }

    fn clv(&mut self) {
        self.registers.status_flags.remove(StatusFlags::OVERFLOW)
    }

    fn cmp(&mut self, operand: u8) {
        self.compare(self.registers.a, operand);
    }

    fn cpx(&mut self, operand: u8) {
        self.compare(self.registers.x, operand);
    }

    fn cpy(&mut self, operand: u8) {
        self.compare(self.registers.y, operand);
    }

    unsafe fn dec(&mut self, operand: u8, addr: u16) {
        let val = operand.wrapping_sub(1);
        (*self.memory).mem_write_u8(addr, val);
        self.set_negative_and_zero_flags(val);
    }

    fn dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1)
    }

    fn dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1)
    }

    fn eor(&mut self, operand: u8) {
        self.registers.a ^= operand;
        self.set_negative_and_zero_flags(self.registers.a);
    }

    unsafe fn inc(&mut self, operand: u8, addr: u16) {
        let val = operand.wrapping_add(1);
        (*self.memory).mem_write_u8(addr, val);
        self.set_negative_and_zero_flags(val);
    }

    fn inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
    }

    fn iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
    }

    fn jmp(&mut self, addr: u16) {
        self.branch(addr);
    }

    unsafe fn jsr(&mut self, addr: u16) {
        self.push_u16_on_stack(self.registers.program_counter);
        self.branch(addr);
    }

    fn lda(&mut self, operand: u8) {
        self.registers.a = operand;
        self.set_negative_and_zero_flags(self.registers.a);
    }

    fn ldx(&mut self, operand: u8) {
        self.registers.x = operand;
        self.set_negative_and_zero_flags(self.registers.x);
    }

    fn ldy(&mut self, operand: u8) {
        self.registers.y = operand;
        self.set_negative_and_zero_flags(self.registers.y);
    }

    unsafe fn lsr(&mut self, operand: u8, addr: u16) {
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, operand & 1 == 1);
        let res = operand >> 1;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res);
    }

    fn lsr_a(&mut self) {
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, self.registers.a & 1 == 1);
        self.registers.a >>= 1;
        self.set_negative_and_zero_flags(self.registers.a);
    }

    fn nop(&mut self) {}

    fn ora(&mut self, operand: u8) {
        self.registers.a |= operand;
        self.set_negative_and_zero_flags(self.registers.a);
    }

    unsafe fn pha(&mut self) {
        self.push_u8_on_stack(self.registers.a);
    }

    unsafe fn php(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.push_u8_on_stack((self.registers.status_flags | StatusFlags::BREAK).bits());
    }

    unsafe fn pla(&mut self) {
        self.registers.a = self.pull_u8_from_stack();
    }

    unsafe fn plp(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.registers.status_flags =
            StatusFlags::from_bits(self.pull_u8_from_stack()).unwrap() & !StatusFlags::BREAK;
    }

    unsafe fn rol(&mut self, operand: u8, addr: u16) {
        let carry = if self.registers.status_flags.contains(StatusFlags::CARRY) {
            1
        } else {
            0
        };
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, operand >> 7 == 1);
        let res = (operand << 1) | carry;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res);
    }

    fn rol_a(&mut self) {
        let carry = if self.registers.status_flags.contains(StatusFlags::CARRY) {
            1
        } else {
            0
        };
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, self.registers.a >> 7 == 1);
        self.registers.a = (self.registers.a << 1) | carry;
        self.set_negative_and_zero_flags(self.registers.a)
    }

    unsafe fn ror(&mut self, operand: u8, addr: u16) {
        let carry = if self.registers.status_flags.contains(StatusFlags::CARRY) {
            1
        } else {
            0
        };
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, operand & 1 == 1);
        let res = operand >> 1 | carry << 7;
        self.set_negative_and_zero_flags(res);
        (*self.memory).mem_write_u8(addr, res)
    }

    fn ror_a(&mut self) {
        let carry = if self.registers.status_flags.contains(StatusFlags::CARRY) {
            1
        } else {
            0
        };
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, self.registers.a & 1 == 1);
        self.registers.a = self.registers.a >> 1 | carry << 7;
        self.set_negative_and_zero_flags(self.registers.a)
    }

    unsafe fn rti(&mut self) {
        //https://www.nesdev.org/wiki/Status_flags
        self.registers.status_flags =
            StatusFlags::from_bits(self.pull_u8_from_stack()).unwrap() & !StatusFlags::BREAK;
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
        self.registers.status_flags.insert(StatusFlags::CARRY)
    }

    fn sed(&mut self) {
        self.registers.status_flags.insert(StatusFlags::DECIMAL)
    }

    fn sei(&mut self) {
        self.registers
            .status_flags
            .insert(StatusFlags::INTERRUPT_DISABLE)
    }

    unsafe fn sta(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.registers.a)
    }

    unsafe fn stx(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.registers.x)
    }

    unsafe fn sty(&mut self, addr: u16) {
        (*self.memory).mem_write_u8(addr, self.registers.y)
    }

    fn tax(&mut self) {
        self.registers.x = self.registers.a;
    }

    fn tay(&mut self) {
        self.registers.y = self.registers.a;
    }

    unsafe fn tsx(&mut self) {
        self.registers.x = self.registers.stack_pointer;
        self.set_negative_and_zero_flags(self.registers.x)
    }

    fn txa(&mut self) {
        self.registers.a = self.registers.x;
    }

    unsafe fn txs(&mut self) {
        self.registers.stack_pointer = self.registers.x;
    }

    fn tya(&mut self) {
        self.registers.a = self.registers.y;
    }

    fn set_negative_and_zero_flags(&mut self, operation_res: u8) {
        self.registers
            .status_flags
            .set(StatusFlags::NEGATIVE, (operation_res as i8) < 0);
        self.registers
            .status_flags
            .set(StatusFlags::ZERO, operation_res == 0);
    }

    fn branch_if(&mut self, addr: u16, predicate: impl Fn(&StatusFlags) -> bool) {
        if predicate(&self.registers.status_flags) {
            self.branch(addr)
        }
    }

    fn branch(&mut self, addr: u16) {
        self.registers.program_counter = addr;
    }

    fn has_branched(&self, previous_addr: u16) -> bool {
        previous_addr != self.registers.program_counter
    }

    fn compare(&mut self, lhs: u8, rhs: u8) {
        let val = lhs.wrapping_sub(rhs);
        self.registers
            .status_flags
            .set(StatusFlags::CARRY, val as i8 >= 0);
        self.set_negative_and_zero_flags(val);
    }

    unsafe fn push_u8_on_stack(&mut self, byte: u8) {
        (*self.memory).mem_write_u8(self.get_stack_addr(), byte);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    unsafe fn push_u16_on_stack(&mut self, addr: u16) {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
        (*self.memory).mem_write_u16(self.get_stack_addr(), addr);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    unsafe fn pull_u8_from_stack(&mut self) -> u8 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        let res = (*self.memory).mem_read_u8(self.get_stack_addr());
        res
    }

    unsafe fn pull_u16_from_stack(&mut self) -> u16 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        let res = (*self.memory).mem_read_u16(self.get_stack_addr());
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        res
    }

    pub fn get_stack_addr(&self) -> u16 {
        u16::from_le_bytes([self.registers.stack_pointer, STACK_ADDR_HI])
    }
}

#[cfg(test)]
mod test;
