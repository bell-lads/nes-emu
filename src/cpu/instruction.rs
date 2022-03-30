use lazy_static::lazy_static;

pub type Opcode = u8;

#[derive(Debug, PartialEq, Eq)]
pub enum Name {
    Adc,
    And,
    Asl,
    Bit,
    Bpl,
    Bmi,
    Bvc,
    Bvs,
    Bcc,
    Bcs,
    Bne,
    Beq,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Eor,
    Clc,
    Sec,
    Cli,
    Sei,
    Clv,
    Cld,
    Sed,
    Inc,
    Jmp,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Ora,
    Tax,
    Txa,
    Dex,
    Inx,
    Tay,
    Tya,
    Dey,
    Iny,
    Rol,
    Ror,
    Sbc,
    Sta,
    Txs,
    Tsx,
    Pha,
    Pla,
    Php,
    Plp,
    Stx,
    Sty,
    Brk,
    Jsr,
    Nop,
    Rti,
    Rts,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Implicit,
    Accumulator,
    Relative,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: Name,
    pub opcode: Opcode,
    pub mode: Mode,
    pub len: u8,
}

impl Instruction {
    const fn new(name: Name, opcode: Opcode, mode: Mode, len: u8) -> Instruction {
        Instruction {
            name,
            opcode,
            mode,
            len,
        }
    }
}

#[rustfmt::skip] 
const INSTRUCTIONS: [Instruction; 151] = [
    Instruction::new(Name::Adc, 0x69, Mode::Immediate, 2),
    Instruction::new(Name::Adc, 0x65, Mode::ZeroPage, 2),
    Instruction::new(Name::Adc, 0x75, Mode::ZeroPageX, 2),
    Instruction::new(Name::Adc, 0x6D, Mode::Absolute, 3),
    Instruction::new(Name::Adc, 0x7D, Mode::AbsoluteX, 3),
    Instruction::new(Name::Adc, 0x79, Mode::AbsoluteY, 3),
    Instruction::new(Name::Adc, 0x61, Mode::IndirectX, 2),
    Instruction::new(Name::Adc, 0x71, Mode::IndirectY, 2),
    //AND_SET
    Instruction::new(Name::And, 0x29, Mode::Immediate, 2),
    Instruction::new(Name::And, 0x25, Mode::ZeroPage, 2),
    Instruction::new(Name::And, 0x35, Mode::ZeroPageX, 2),
    Instruction::new(Name::And, 0x2D, Mode::Absolute, 3),
    Instruction::new(Name::And, 0x3D, Mode::AbsoluteX, 3),
    Instruction::new(Name::And, 0x39, Mode::AbsoluteY, 3),
    Instruction::new(Name::And, 0x21, Mode::IndirectX, 2),
    Instruction::new(Name::And, 0x31, Mode::IndirectY, 2),
    //ASL_SET
    Instruction::new(Name::Asl, 0x0A, Mode::Accumulator, 1),
    Instruction::new(Name::Asl, 0x06, Mode::ZeroPage, 2),
    Instruction::new(Name::Asl, 0x16, Mode::ZeroPageX, 2),
    Instruction::new(Name::Asl, 0x0E, Mode::Absolute, 3),
    Instruction::new(Name::Asl, 0x1E, Mode::AbsoluteX, 3),
    //BIT_SET
    Instruction::new(Name::Bit, 0x24, Mode::ZeroPage, 2),
    Instruction::new(Name::Bit, 0x2C, Mode::Absolute, 3),
    //BRANCHES_SET
    Instruction::new(Name::Bpl, 0x10, Mode::Relative, 2),
    Instruction::new(Name::Bmi, 0x30, Mode::Relative, 2),
    Instruction::new(Name::Bvc, 0x50, Mode::Relative, 2),
    Instruction::new(Name::Bvs, 0x70, Mode::Relative, 2),
    Instruction::new(Name::Bcc, 0x90, Mode::Relative, 2),
    Instruction::new(Name::Bcs, 0xB0, Mode::Relative, 2),
    Instruction::new(Name::Bne, 0xD0, Mode::Relative, 2),
    Instruction::new(Name::Beq, 0xF0, Mode::Relative, 2),
    //CMP_SET
    Instruction::new(Name::Cmp, 0xC9, Mode::Immediate, 2),
    Instruction::new(Name::Cmp, 0xC5, Mode::ZeroPage, 2),
    Instruction::new(Name::Cmp, 0xD5, Mode::ZeroPageX, 2),
    Instruction::new(Name::Cmp, 0xCD, Mode::Absolute, 3),
    Instruction::new(Name::Cmp, 0xDD, Mode::AbsoluteX, 3),
    Instruction::new(Name::Cmp, 0xD9, Mode::AbsoluteY, 3),
    Instruction::new(Name::Cmp, 0xC1, Mode::IndirectX, 2),
    Instruction::new(Name::Cmp, 0xD1, Mode::IndirectY, 2),
    //CPX_SET
    Instruction::new(Name::Cpx, 0xE0, Mode::Immediate, 2),
    Instruction::new(Name::Cpx, 0xE4, Mode::ZeroPage, 2),
    Instruction::new(Name::Cpx, 0xEC, Mode::Absolute, 3),
    //CPY_SET
    Instruction::new(Name::Cpy, 0xC0, Mode::Immediate, 2),
    Instruction::new(Name::Cpy, 0xC4, Mode::ZeroPage, 2),
    Instruction::new(Name::Cpy, 0xCC, Mode::Absolute, 3),
    //DEC_SET
    Instruction::new(Name::Dec, 0xC6, Mode::ZeroPage, 2),
    Instruction::new(Name::Dec, 0xD6, Mode::ZeroPageX, 2),
    Instruction::new(Name::Dec, 0xCE, Mode::Absolute, 3),
    Instruction::new(Name::Dec, 0xDE, Mode::AbsoluteX, 3),
    //EOR_SET
    Instruction::new(Name::Eor, 0x49, Mode::Immediate, 2),
    Instruction::new(Name::Eor, 0x45, Mode::ZeroPage, 2),
    Instruction::new(Name::Eor, 0x55, Mode::ZeroPageX, 2),
    Instruction::new(Name::Eor, 0x4D, Mode::Absolute, 3),
    Instruction::new(Name::Eor, 0x5D, Mode::AbsoluteX, 3),
    Instruction::new(Name::Eor, 0x59, Mode::AbsoluteY, 3),
    Instruction::new(Name::Eor, 0x41, Mode::IndirectX, 2),
    Instruction::new(Name::Eor, 0x51, Mode::IndirectY, 2),
    //PROC_STATUS_SET
    Instruction::new(Name::Clc, 0x18, Mode::Implicit, 1),
    Instruction::new(Name::Sec, 0x38, Mode::Implicit, 1),
    Instruction::new(Name::Cli, 0x58, Mode::Implicit, 1),
    Instruction::new(Name::Sei, 0x78, Mode::Implicit, 1),
    Instruction::new(Name::Clv, 0xB8, Mode::Implicit, 1),
    Instruction::new(Name::Cld, 0xD8, Mode::Implicit, 1),
    Instruction::new(Name::Sed, 0xF8, Mode::Implicit, 1),
    //INC_SET
    Instruction::new(Name::Inc, 0xE6, Mode::ZeroPage, 2),
    Instruction::new(Name::Inc, 0xF6, Mode::ZeroPageX, 2),
    Instruction::new(Name::Inc, 0xEE, Mode::Absolute, 3),
    Instruction::new(Name::Inc, 0xFE, Mode::AbsoluteX, 3),
    //JMP_SET
    Instruction::new(Name::Jmp, 0x4C, Mode::Absolute, 3),
    Instruction::new(Name::Jmp, 0x6C, Mode::Indirect, 3),
    //LDA_SET
    Instruction::new(Name::Lda, 0xA9, Mode::Immediate, 2),
    Instruction::new(Name::Lda, 0xA5, Mode::ZeroPage, 2),
    Instruction::new(Name::Lda, 0xB5, Mode::ZeroPageX, 2),
    Instruction::new(Name::Lda, 0xAD, Mode::Absolute, 3),
    Instruction::new(Name::Lda, 0xBD, Mode::AbsoluteX, 3),
    Instruction::new(Name::Lda, 0xB9, Mode::AbsoluteY, 3),
    Instruction::new(Name::Lda, 0xA1, Mode::IndirectX, 2),
    Instruction::new(Name::Lda, 0xB1, Mode::IndirectY, 2),
    //LDX_SET
    Instruction::new(Name::Ldx, 0xA2, Mode::Immediate, 2),
    Instruction::new(Name::Ldx, 0xA6, Mode::ZeroPage, 2),
    Instruction::new(Name::Ldx, 0xB6, Mode::ZeroPageY, 2),
    Instruction::new(Name::Ldx, 0xAE, Mode::Absolute, 3),
    Instruction::new(Name::Ldx, 0xBE, Mode::AbsoluteY, 3),
    //LDY_SET
    Instruction::new(Name::Ldy, 0xA0, Mode::Immediate, 2),
    Instruction::new(Name::Ldy, 0xA4, Mode::ZeroPage, 2),
    Instruction::new(Name::Ldy, 0xB4, Mode::ZeroPageX, 2),
    Instruction::new(Name::Ldy, 0xAC, Mode::Absolute, 3),
    Instruction::new(Name::Ldy, 0xBC, Mode::AbsoluteX, 3),
    //LSR_SET
    Instruction::new(Name::Lsr, 0x4A, Mode::Accumulator, 1),
    Instruction::new(Name::Lsr, 0x46, Mode::ZeroPage, 2),
    Instruction::new(Name::Lsr, 0x56, Mode::ZeroPageX, 2),
    Instruction::new(Name::Lsr, 0x4E, Mode::Absolute, 3),
    Instruction::new(Name::Lsr, 0x5E, Mode::AbsoluteX, 3),
    //ORA_SET
    Instruction::new(Name::Ora, 0x09, Mode::Immediate, 2),
    Instruction::new(Name::Ora, 0x05, Mode::ZeroPage, 2),
    Instruction::new(Name::Ora, 0x15, Mode::ZeroPageX, 2),
    Instruction::new(Name::Ora, 0x0D, Mode::Absolute, 3),
    Instruction::new(Name::Ora, 0x1D, Mode::AbsoluteX, 3),
    Instruction::new(Name::Ora, 0x19, Mode::AbsoluteY, 3),
    Instruction::new(Name::Ora, 0x01, Mode::IndirectX, 2),
    Instruction::new(Name::Ora, 0x11, Mode::IndirectY, 2),
    //REGISTER_SET
    Instruction::new(Name::Tax, 0xAA, Mode::Implicit, 1),
    Instruction::new(Name::Txa, 0x8A, Mode::Implicit, 1),
    Instruction::new(Name::Dex, 0xCA, Mode::Implicit, 1),
    Instruction::new(Name::Inx, 0xE8, Mode::Implicit, 1),
    Instruction::new(Name::Tay, 0xA8, Mode::Implicit, 1),
    Instruction::new(Name::Tya, 0x98, Mode::Implicit, 1),
    Instruction::new(Name::Dey, 0x88, Mode::Implicit, 1),
    Instruction::new(Name::Iny, 0xC8, Mode::Implicit, 1),
    //ROL_SET
    Instruction::new(Name::Rol, 0x2A, Mode::Accumulator, 1),
    Instruction::new(Name::Rol, 0x26, Mode::ZeroPage, 2),
    Instruction::new(Name::Rol, 0x36, Mode::ZeroPageX, 2),
    Instruction::new(Name::Rol, 0x2E, Mode::Absolute, 3),
    Instruction::new(Name::Rol, 0x3E, Mode::AbsoluteX, 3),
    //ROR_SET
    Instruction::new(Name::Ror, 0x6A, Mode::Accumulator, 1),
    Instruction::new(Name::Ror, 0x66, Mode::ZeroPage, 2),
    Instruction::new(Name::Ror, 0x76, Mode::ZeroPageX, 2),
    Instruction::new(Name::Ror, 0x6E, Mode::Absolute, 3),
    Instruction::new(Name::Ror, 0x7E, Mode::AbsoluteX, 3),
    //SBC_SET
    Instruction::new(Name::Sbc, 0xE9, Mode::Immediate, 2),
    Instruction::new(Name::Sbc, 0xE5, Mode::ZeroPage, 2),
    Instruction::new(Name::Sbc, 0xF5, Mode::ZeroPageX, 2),
    Instruction::new(Name::Sbc, 0xED, Mode::Absolute, 3),
    Instruction::new(Name::Sbc, 0xFD, Mode::AbsoluteX, 3),
    Instruction::new(Name::Sbc, 0xF9, Mode::AbsoluteY, 3),
    Instruction::new(Name::Sbc, 0xE1, Mode::IndirectX, 2),
    Instruction::new(Name::Sbc, 0xF1, Mode::IndirectY, 2),
    //STA_SET
    Instruction::new(Name::Sta, 0x85, Mode::ZeroPage, 2),
    Instruction::new(Name::Sta, 0x95, Mode::ZeroPageX, 2),
    Instruction::new(Name::Sta, 0x8D, Mode::Absolute, 3),
    Instruction::new(Name::Sta, 0x9D, Mode::AbsoluteX, 3),
    Instruction::new(Name::Sta, 0x99, Mode::AbsoluteY, 3),
    Instruction::new(Name::Sta, 0x81, Mode::IndirectX, 2),
    Instruction::new(Name::Sta, 0x91, Mode::IndirectY, 2),
    //STACK_SET
    Instruction::new(Name::Txs, 0x9A, Mode::Implicit, 1),
    Instruction::new(Name::Tsx, 0xBA, Mode::Implicit, 1),
    Instruction::new(Name::Pha, 0x48, Mode::Implicit, 1),
    Instruction::new(Name::Pla, 0x68, Mode::Implicit, 1),
    Instruction::new(Name::Php, 0x08, Mode::Implicit, 1),
    Instruction::new(Name::Plp, 0x28, Mode::Implicit, 1),
    //STX_SET
    Instruction::new(Name::Stx, 0x86, Mode::ZeroPage, 2),
    Instruction::new(Name::Stx, 0x96, Mode::ZeroPageY, 2),
    Instruction::new(Name::Stx, 0x8E, Mode::Absolute, 3),
    Instruction::new(Name::Sty, 0x84, Mode::ZeroPage, 2),
    Instruction::new(Name::Sty, 0x94, Mode::ZeroPageX, 2),
    Instruction::new(Name::Sty, 0x8C, Mode::Absolute, 3),
    //OTHER_SET
    Instruction::new(Name::Brk, 0x00, Mode::Implicit, 1),
    Instruction::new(Name::Jsr, 0x20, Mode::Absolute, 3),
    Instruction::new(Name::Nop, 0xEA, Mode::Implicit, 1),
    Instruction::new(Name::Rti, 0x40, Mode::Implicit, 1),
    Instruction::new(Name::Rts, 0x60, Mode::Implicit, 1)
];

lazy_static! {
    pub static ref INSTRUCTION_MAP: std::collections::HashMap<u8, &'static Instruction> = {
        let mut m = std::collections::HashMap::<u8, &'static Instruction>::new();
        for instruction in INSTRUCTIONS.iter() {
            if m.insert(instruction.opcode, instruction).is_some() {
                panic!(
                    "INSTRUCTION_MAP : Error, opcode already defined adding instruction : {:#?}",
                    instruction
                )
            }
        }
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_lenght_from_mode(mode: &Mode) -> u8 {
        match mode {
            Mode::Implicit | Mode::Accumulator => 1,
            Mode::Immediate
            | Mode::ZeroPage
            | Mode::ZeroPageX
            | Mode::ZeroPageY
            | Mode::IndirectX
            | Mode::IndirectY
            | Mode::Relative => 2,
            Mode::Absolute | Mode::AbsoluteX | Mode::AbsoluteY | Mode::Indirect => 3,
        }
    }

    #[test]
    fn all_opcodes_are_different() {
        //The MAP construction will panic and fail the test
        //if all opcodes are not different
        assert_eq!(Name::Brk, INSTRUCTION_MAP.get(&0x00).unwrap().name)
    }

    #[test]
    fn all_modes_and_sizes_are_coherent() {
        let bad_instructions: Vec<&&Instruction> = INSTRUCTION_MAP
            .iter()
            .map(|(_, i)| i)
            .filter(|i| i.len != get_lenght_from_mode(&i.mode))
            .collect();
        assert_eq!(0, bad_instructions.len())
    }
}
