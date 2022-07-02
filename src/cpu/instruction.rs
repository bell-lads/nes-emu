#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionKind {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AddressingMode {
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

impl AddressingMode {
    pub fn length_in_bytes(self) -> u16 {
        match self {
            AddressingMode::Implicit | AddressingMode::Accumulator => 0,
            AddressingMode::Immediate
            | AddressingMode::ZeroPage
            | AddressingMode::ZeroPageX
            | AddressingMode::ZeroPageY
            | AddressingMode::IndirectX
            | AddressingMode::IndirectY
            | AddressingMode::Relative => 1,
            AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteY
            | AddressingMode::Indirect => 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Instruction {
    kind: InstructionKind,
    addressing_mode: AddressingMode,
}

impl Instruction {
    #[rustfmt::skip]
    pub const fn from_opcode(opcode: u8) -> Instruction {
        match opcode {
            // ADC
            0x69 => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::Immediate },
            0x65 => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::ZeroPage },
            0x75 => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::ZeroPageX },
            0x6D => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::Absolute },
            0x7D => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::AbsoluteX },
            0x79 => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::AbsoluteY },
            0x61 => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::IndirectX },
            0x71 => Instruction { kind: InstructionKind::Adc, addressing_mode: AddressingMode::IndirectY },
            // AND
            0x29 => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::Immediate },
            0x25 => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::ZeroPage },
            0x35 => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::ZeroPageX },
            0x2D => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::Absolute },
            0x3D => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::AbsoluteX },
            0x39 => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::AbsoluteY },
            0x21 => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::IndirectX },
            0x31 => Instruction { kind: InstructionKind::And, addressing_mode: AddressingMode::IndirectY },
            // ASL
            0x0A => Instruction { kind: InstructionKind::Asl, addressing_mode: AddressingMode::Accumulator },
            0x06 => Instruction { kind: InstructionKind::Asl, addressing_mode: AddressingMode::ZeroPage },
            0x16 => Instruction { kind: InstructionKind::Asl, addressing_mode: AddressingMode::ZeroPageX },
            0x0E => Instruction { kind: InstructionKind::Asl, addressing_mode: AddressingMode::Absolute },
            0x1E => Instruction { kind: InstructionKind::Asl, addressing_mode: AddressingMode::AbsoluteX },
            // BIT
            0x24 => Instruction { kind: InstructionKind::Bit, addressing_mode: AddressingMode::ZeroPage },
            0x2C => Instruction { kind: InstructionKind::Bit, addressing_mode: AddressingMode::Absolute },
            // BRANCHES
            0x10 => Instruction { kind: InstructionKind::Bpl, addressing_mode: AddressingMode::Relative },
            0x30 => Instruction { kind: InstructionKind::Bmi, addressing_mode: AddressingMode::Relative },
            0x50 => Instruction { kind: InstructionKind::Bvc, addressing_mode: AddressingMode::Relative },
            0x70 => Instruction { kind: InstructionKind::Bvs, addressing_mode: AddressingMode::Relative },
            0x90 => Instruction { kind: InstructionKind::Bcc, addressing_mode: AddressingMode::Relative },
            0xB0 => Instruction { kind: InstructionKind::Bcs, addressing_mode: AddressingMode::Relative },
            0xD0 => Instruction { kind: InstructionKind::Bne, addressing_mode: AddressingMode::Relative },
            0xF0 => Instruction { kind: InstructionKind::Beq, addressing_mode: AddressingMode::Relative },
            // CMP
            0xC9 => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::Immediate },
            0xC5 => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::ZeroPage },
            0xD5 => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::ZeroPageX },
            0xCD => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::Absolute },
            0xDD => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::AbsoluteX },
            0xD9 => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::AbsoluteY },
            0xC1 => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::IndirectX },
            0xD1 => Instruction { kind: InstructionKind::Cmp, addressing_mode: AddressingMode::IndirectY },
            // CPX
            0xE0 => Instruction { kind: InstructionKind::Cpx, addressing_mode: AddressingMode::Immediate },
            0xE4 => Instruction { kind: InstructionKind::Cpx, addressing_mode: AddressingMode::ZeroPage },
            0xEC => Instruction { kind: InstructionKind::Cpx, addressing_mode: AddressingMode::Absolute },
            // CPY
            0xC0 => Instruction { kind: InstructionKind::Cpy, addressing_mode: AddressingMode::Immediate },
            0xC4 => Instruction { kind: InstructionKind::Cpy, addressing_mode: AddressingMode::ZeroPage },
            0xCC => Instruction { kind: InstructionKind::Cpy, addressing_mode: AddressingMode::Absolute },
            // DEC
            0xC6 => Instruction { kind: InstructionKind::Dec, addressing_mode: AddressingMode::ZeroPage },
            0xD6 => Instruction { kind: InstructionKind::Dec, addressing_mode: AddressingMode::ZeroPageX },
            0xCE => Instruction { kind: InstructionKind::Dec, addressing_mode: AddressingMode::Absolute },
            0xDE => Instruction { kind: InstructionKind::Dec, addressing_mode: AddressingMode::AbsoluteX },
            //EOR_SET
            0x49 => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::Immediate },
            0x45 => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::ZeroPage },
            0x55 => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::ZeroPageX },
            0x4D => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::Absolute },
            0x5D => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::AbsoluteX },
            0x59 => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::AbsoluteY },
            0x41 => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::IndirectX },
            0x51 => Instruction { kind: InstructionKind::Eor, addressing_mode: AddressingMode::IndirectY },
            // PROC STATUS
            0x18 => Instruction { kind: InstructionKind::Clc, addressing_mode: AddressingMode::Implicit },
            0x38 => Instruction { kind: InstructionKind::Sec, addressing_mode: AddressingMode::Implicit },
            0x58 => Instruction { kind: InstructionKind::Cli, addressing_mode: AddressingMode::Implicit },
            0x78 => Instruction { kind: InstructionKind::Sei, addressing_mode: AddressingMode::Implicit },
            0xB8 => Instruction { kind: InstructionKind::Clv, addressing_mode: AddressingMode::Implicit },
            0xD8 => Instruction { kind: InstructionKind::Cld, addressing_mode: AddressingMode::Implicit },
            0xF8 => Instruction { kind: InstructionKind::Sed, addressing_mode: AddressingMode::Implicit },
            // INC
            0xE6 => Instruction { kind: InstructionKind::Inc, addressing_mode: AddressingMode::ZeroPage },
            0xF6 => Instruction { kind: InstructionKind::Inc, addressing_mode: AddressingMode::ZeroPageX },
            0xEE => Instruction { kind: InstructionKind::Inc, addressing_mode: AddressingMode::Absolute },
            0xFE => Instruction { kind: InstructionKind::Inc, addressing_mode: AddressingMode::AbsoluteX },
            // JMP
            0x4C => Instruction { kind: InstructionKind::Jmp, addressing_mode: AddressingMode::Absolute },
            0x6C => Instruction { kind: InstructionKind::Jmp, addressing_mode: AddressingMode::Indirect },
            // LDA
            0xA9 => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::Immediate },
            0xA5 => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::ZeroPage },
            0xB5 => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::ZeroPageX },
            0xAD => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::Absolute },
            0xBD => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::AbsoluteX },
            0xB9 => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::AbsoluteY },
            0xA1 => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::IndirectX },
            0xB1 => Instruction { kind: InstructionKind::Lda, addressing_mode: AddressingMode::IndirectY },
            // LDX
            0xA2 => Instruction { kind: InstructionKind::Ldx, addressing_mode: AddressingMode::Immediate },
            0xA6 => Instruction { kind: InstructionKind::Ldx, addressing_mode: AddressingMode::ZeroPage },
            0xB6 => Instruction { kind: InstructionKind::Ldx, addressing_mode: AddressingMode::ZeroPageY },
            0xAE => Instruction { kind: InstructionKind::Ldx, addressing_mode: AddressingMode::Absolute },
            0xBE => Instruction { kind: InstructionKind::Ldx, addressing_mode: AddressingMode::AbsoluteY },
            // LDY
            0xA0 => Instruction { kind: InstructionKind::Ldy, addressing_mode: AddressingMode::Immediate },
            0xA4 => Instruction { kind: InstructionKind::Ldy, addressing_mode: AddressingMode::ZeroPage },
            0xB4 => Instruction { kind: InstructionKind::Ldy, addressing_mode: AddressingMode::ZeroPageX },
            0xAC => Instruction { kind: InstructionKind::Ldy, addressing_mode: AddressingMode::Absolute },
            0xBC => Instruction { kind: InstructionKind::Ldy, addressing_mode: AddressingMode::AbsoluteX },
            // LSR
            0x4A => Instruction { kind: InstructionKind::Lsr, addressing_mode: AddressingMode::Accumulator },
            0x46 => Instruction { kind: InstructionKind::Lsr, addressing_mode: AddressingMode::ZeroPage },
            0x56 => Instruction { kind: InstructionKind::Lsr, addressing_mode: AddressingMode::ZeroPageX },
            0x4E => Instruction { kind: InstructionKind::Lsr, addressing_mode: AddressingMode::Absolute },
            0x5E => Instruction { kind: InstructionKind::Lsr, addressing_mode: AddressingMode::AbsoluteX },
            // ORA
            0x09 => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::Immediate },
            0x05 => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::ZeroPage },
            0x15 => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::ZeroPageX },
            0x0D => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::Absolute },
            0x1D => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::AbsoluteX },
            0x19 => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::AbsoluteY },
            0x01 => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::IndirectX },
            0x11 => Instruction { kind: InstructionKind::Ora, addressing_mode: AddressingMode::IndirectY },
            // REGISTER
            0xAA => Instruction { kind: InstructionKind::Tax, addressing_mode: AddressingMode::Implicit },
            0x8A => Instruction { kind: InstructionKind::Txa, addressing_mode: AddressingMode::Implicit },
            0xCA => Instruction { kind: InstructionKind::Dex, addressing_mode: AddressingMode::Implicit },
            0xE8 => Instruction { kind: InstructionKind::Inx, addressing_mode: AddressingMode::Implicit },
            0xA8 => Instruction { kind: InstructionKind::Tay, addressing_mode: AddressingMode::Implicit },
            0x98 => Instruction { kind: InstructionKind::Tya, addressing_mode: AddressingMode::Implicit },
            0x88 => Instruction { kind: InstructionKind::Dey, addressing_mode: AddressingMode::Implicit },
            0xC8 => Instruction { kind: InstructionKind::Iny, addressing_mode: AddressingMode::Implicit },
            // ROL
            0x2A => Instruction { kind: InstructionKind::Rol, addressing_mode: AddressingMode::Accumulator },
            0x26 => Instruction { kind: InstructionKind::Rol, addressing_mode: AddressingMode::ZeroPage },
            0x36 => Instruction { kind: InstructionKind::Rol, addressing_mode: AddressingMode::ZeroPageX },
            0x2E => Instruction { kind: InstructionKind::Rol, addressing_mode: AddressingMode::Absolute },
            0x3E => Instruction { kind: InstructionKind::Rol, addressing_mode: AddressingMode::AbsoluteX },
            // ROR
            0x6A => Instruction { kind: InstructionKind::Ror, addressing_mode: AddressingMode::Accumulator },
            0x66 => Instruction { kind: InstructionKind::Ror, addressing_mode: AddressingMode::ZeroPage },
            0x76 => Instruction { kind: InstructionKind::Ror, addressing_mode: AddressingMode::ZeroPageX },
            0x6E => Instruction { kind: InstructionKind::Ror, addressing_mode: AddressingMode::Absolute },
            0x7E => Instruction { kind: InstructionKind::Ror, addressing_mode: AddressingMode::AbsoluteX },
            // SBC
            0xE9 => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::Immediate },
            0xE5 => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::ZeroPage },
            0xF5 => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::ZeroPageX },
            0xED => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::Absolute },
            0xFD => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::AbsoluteX },
            0xF9 => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::AbsoluteY },
            0xE1 => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::IndirectX },
            0xF1 => Instruction { kind: InstructionKind::Sbc, addressing_mode: AddressingMode::IndirectY },
            // STA
            0x85 => Instruction { kind: InstructionKind::Sta, addressing_mode: AddressingMode::ZeroPage },
            0x95 => Instruction { kind: InstructionKind::Sta, addressing_mode: AddressingMode::ZeroPageX },
            0x8D => Instruction { kind: InstructionKind::Sta, addressing_mode: AddressingMode::Absolute },
            0x9D => Instruction { kind: InstructionKind::Sta, addressing_mode: AddressingMode::AbsoluteX },
            0x99 => Instruction { kind: InstructionKind::Sta, addressing_mode: AddressingMode::AbsoluteY },
            0x81 => Instruction { kind: InstructionKind::Sta, addressing_mode: AddressingMode::IndirectX },
            0x91 => Instruction { kind: InstructionKind::Sta, addressing_mode: AddressingMode::IndirectY },
            // STACK
            0x9A => Instruction { kind: InstructionKind::Txs, addressing_mode: AddressingMode::Implicit },
            0xBA => Instruction { kind: InstructionKind::Tsx, addressing_mode: AddressingMode::Implicit },
            0x48 => Instruction { kind: InstructionKind::Pha, addressing_mode: AddressingMode::Implicit },
            0x68 => Instruction { kind: InstructionKind::Pla, addressing_mode: AddressingMode::Implicit },
            0x08 => Instruction { kind: InstructionKind::Php, addressing_mode: AddressingMode::Implicit },
            0x28 => Instruction { kind: InstructionKind::Plp, addressing_mode: AddressingMode::Implicit },
            // STX
            0x86 => Instruction { kind: InstructionKind::Stx, addressing_mode: AddressingMode::ZeroPage },
            0x96 => Instruction { kind: InstructionKind::Stx, addressing_mode: AddressingMode::ZeroPageY },
            0x8E => Instruction { kind: InstructionKind::Stx, addressing_mode: AddressingMode::Absolute },
            0x84 => Instruction { kind: InstructionKind::Sty, addressing_mode: AddressingMode::ZeroPage },
            0x94 => Instruction { kind: InstructionKind::Sty, addressing_mode: AddressingMode::ZeroPageX },
            0x8C => Instruction { kind: InstructionKind::Sty, addressing_mode: AddressingMode::Absolute },
            // OTHER
            0x00 => Instruction { kind: InstructionKind::Brk, addressing_mode: AddressingMode::Implicit },
            0x20 => Instruction { kind: InstructionKind::Jsr, addressing_mode: AddressingMode::Absolute },
            0xEA => Instruction { kind: InstructionKind::Nop, addressing_mode: AddressingMode::Implicit },
            0x40 => Instruction { kind: InstructionKind::Rti, addressing_mode: AddressingMode::Implicit },
            0x60 => Instruction { kind: InstructionKind::Rts, addressing_mode: AddressingMode::Implicit },
            _ => panic!("Given opcode doesnt map to any instruction")
        }
    }

    pub fn kind(&self) -> InstructionKind {
        self.kind
    }

    pub fn addressing_mode(&self) -> AddressingMode {
        self.addressing_mode
    }
}
