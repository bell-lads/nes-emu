use lazy_static::lazy_static;

pub type Opcode = u8;

#[derive(Debug)]
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
    pub name: &'static str,
    pub opcode: Opcode,
    pub mode: Mode,
    pub len: u8,
}

impl Instruction {
    const fn new(name: &'static str, opcode: Opcode, mode: Mode, len: u8) -> Instruction {
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
    Instruction::new("ADC", 0x69, Mode::Immediate, 2),
    Instruction::new("ADC", 0x65, Mode::ZeroPage, 2),
    Instruction::new("ADC", 0x75, Mode::ZeroPageX, 2),
    Instruction::new("ADC", 0x6D, Mode::Absolute, 3),
    Instruction::new("ADC", 0x7D, Mode::AbsoluteX, 3),
    Instruction::new("ADC", 0x79, Mode::AbsoluteY, 3),
    Instruction::new("ADC", 0x61, Mode::IndirectX, 2),
    Instruction::new("ADC", 0x71, Mode::IndirectY, 2),
    //AND_SET
    Instruction::new("AND", 0x29, Mode::Immediate, 2),
    Instruction::new("AND", 0x25, Mode::ZeroPage, 2),
    Instruction::new("AND", 0x35, Mode::ZeroPageX, 2),
    Instruction::new("AND", 0x2D, Mode::Absolute, 3),
    Instruction::new("AND", 0x3D, Mode::AbsoluteX, 3),
    Instruction::new("AND", 0x39, Mode::AbsoluteY, 3),
    Instruction::new("AND", 0x21, Mode::IndirectX, 2),
    Instruction::new("AND", 0x31, Mode::IndirectY, 2),
    //ASL_SET
    Instruction::new("ASL", 0x0A, Mode::Accumulator, 1),
    Instruction::new("ASL", 0x06, Mode::ZeroPage, 2),
    Instruction::new("ASL", 0x16, Mode::ZeroPageX, 2),
    Instruction::new("ASL", 0x0E, Mode::Absolute, 3),
    Instruction::new("ASL", 0x1E, Mode::AbsoluteX, 3),
    //BIT_SET
    Instruction::new("BIT", 0x24, Mode::ZeroPage, 2),
    Instruction::new("BIT", 0x2C, Mode::Absolute, 3),
    //BRANCHES_SET
    Instruction::new("BPL", 0x10, Mode::Relative, 2),
    Instruction::new("BMI", 0x30, Mode::Relative, 2),
    Instruction::new("BVC", 0x50, Mode::Relative, 2),
    Instruction::new("BVS", 0x70, Mode::Relative, 2),
    Instruction::new("BCC", 0x90, Mode::Relative, 2),
    Instruction::new("BCS", 0xB0, Mode::Relative, 2),
    Instruction::new("BNE", 0xD0, Mode::Relative, 2),
    Instruction::new("BEQ", 0xF0, Mode::Relative, 2),
    //CMP_SET
    Instruction::new("CMP", 0xC9, Mode::Immediate, 2),
    Instruction::new("CMP", 0xC5, Mode::ZeroPage, 2),
    Instruction::new("CMP", 0xD5, Mode::ZeroPageX, 2),
    Instruction::new("CMP", 0xCD, Mode::Absolute, 3),
    Instruction::new("CMP", 0xDD, Mode::AbsoluteX, 3),
    Instruction::new("CMP", 0xD9, Mode::AbsoluteY, 3),
    Instruction::new("CMP", 0xC1, Mode::IndirectX, 2),
    Instruction::new("CMP", 0xD1, Mode::IndirectY, 2),
    //CPX_SET
    Instruction::new("CPX", 0xE0, Mode::Immediate, 2),
    Instruction::new("CPX", 0xE4, Mode::ZeroPage, 2),
    Instruction::new("CPX", 0xEC, Mode::Absolute, 3),
    //CPY_SET
    Instruction::new("CPY", 0xC0, Mode::Immediate, 2),
    Instruction::new("CPY", 0xC4, Mode::ZeroPage, 2),
    Instruction::new("CPY", 0xCC, Mode::Absolute, 3),
    //DEC_SET
    Instruction::new("DEC", 0xC6, Mode::ZeroPage, 2),
    Instruction::new("DEC", 0xD6, Mode::ZeroPageX, 2),
    Instruction::new("DEC", 0xCE, Mode::Absolute, 3),
    Instruction::new("DEC", 0xDE, Mode::AbsoluteX, 3),
    //EOR_SET
    Instruction::new("EOR", 0x49, Mode::Immediate, 2),
    Instruction::new("EOR", 0x45, Mode::ZeroPage, 2),
    Instruction::new("EOR", 0x55, Mode::ZeroPageX, 2),
    Instruction::new("EOR", 0x4D, Mode::Absolute, 3),
    Instruction::new("EOR", 0x5D, Mode::AbsoluteX, 3),
    Instruction::new("EOR", 0x59, Mode::AbsoluteY, 3),
    Instruction::new("EOR", 0x41, Mode::IndirectX, 2),
    Instruction::new("EOR", 0x51, Mode::IndirectY, 2),
    //PROC_STATUS_SET
    Instruction::new("CLC", 0x18, Mode::Implicit, 1),
    Instruction::new("SEC", 0x38, Mode::Implicit, 1),
    Instruction::new("CLI", 0x58, Mode::Implicit, 1),
    Instruction::new("SEI", 0x78, Mode::Implicit, 1),
    Instruction::new("CLV", 0xB8, Mode::Implicit, 1),
    Instruction::new("CLD", 0xD8, Mode::Implicit, 1),
    Instruction::new("SED", 0xF8, Mode::Implicit, 1),
    //whINC_SET
    Instruction::new("INC", 0xE6, Mode::ZeroPage, 2),
    Instruction::new("INC", 0xF6, Mode::ZeroPageX, 2),
    Instruction::new("INC", 0xEE, Mode::Absolute, 3),
    Instruction::new("INC", 0xFE, Mode::AbsoluteX, 3),
    //JMP_SET
    Instruction::new("JMP", 0x4C, Mode::Absolute, 3),
    Instruction::new("JMP", 0x6C, Mode::Indirect, 3),
    //LDA_SET
    Instruction::new("LDA", 0xA9, Mode::Immediate, 2),
    Instruction::new("LDA", 0xA5, Mode::ZeroPage, 2),
    Instruction::new("LDA", 0xB5, Mode::ZeroPageX, 2),
    Instruction::new("LDA", 0xAD, Mode::Absolute, 3),
    Instruction::new("LDA", 0xBD, Mode::AbsoluteX, 3),
    Instruction::new("LDA", 0xB9, Mode::AbsoluteY, 3),
    Instruction::new("LDA", 0xA1, Mode::IndirectX, 2),
    Instruction::new("LDA", 0xB1, Mode::IndirectY, 2),
    //LDX_SET
    Instruction::new("LDX", 0xA2, Mode::Immediate, 2),
    Instruction::new("LDX", 0xA6, Mode::ZeroPage, 2),
    Instruction::new("LDX", 0xB6, Mode::ZeroPageY, 2),
    Instruction::new("LDX", 0xAE, Mode::Absolute, 3),
    Instruction::new("LDX", 0xBE, Mode::AbsoluteY, 3),
    //LDY_SET
    Instruction::new("LDY", 0xA0, Mode::Immediate, 2),
    Instruction::new("LDY", 0xA4, Mode::ZeroPage, 2),
    Instruction::new("LDY", 0xB4, Mode::ZeroPageX, 2),
    Instruction::new("LDY", 0xAC, Mode::Absolute, 3),
    Instruction::new("LDY", 0xBC, Mode::AbsoluteX, 3),
    //LSR_SET
    Instruction::new("LSR", 0x4A, Mode::Accumulator, 1),
    Instruction::new("LSR", 0x46, Mode::ZeroPage, 2),
    Instruction::new("LSR", 0x56, Mode::ZeroPageX, 2),
    Instruction::new("LSR", 0x4E, Mode::Absolute, 3),
    Instruction::new("LSR", 0x5E, Mode::AbsoluteX, 3),
    //ORA_SET
    Instruction::new("ORA", 0x09, Mode::Immediate, 2),
    Instruction::new("ORA", 0x05, Mode::ZeroPage, 2),
    Instruction::new("ORA", 0x15, Mode::ZeroPageX, 2),
    Instruction::new("ORA", 0x0D, Mode::Absolute, 3),
    Instruction::new("ORA", 0x1D, Mode::AbsoluteX, 3),
    Instruction::new("ORA", 0x19, Mode::AbsoluteY, 3),
    Instruction::new("ORA", 0x01, Mode::IndirectX, 2),
    Instruction::new("ORA", 0x11, Mode::IndirectY, 2),
    //REGISTER_SET
    Instruction::new("TAX", 0xAA, Mode::Implicit, 1),
    Instruction::new("TXA", 0x8A, Mode::Implicit, 1),
    Instruction::new("DEX", 0xCA, Mode::Implicit, 1),
    Instruction::new("INX", 0xE8, Mode::Implicit, 1),
    Instruction::new("TAY", 0xA8, Mode::Implicit, 1),
    Instruction::new("TYA", 0x98, Mode::Implicit, 1),
    Instruction::new("DEY", 0x88, Mode::Implicit, 1),
    Instruction::new("INY", 0xC8, Mode::Implicit, 1),
    //ROL_SET
    Instruction::new("ROL", 0x2A, Mode::Accumulator, 1),
    Instruction::new("ROL", 0x26, Mode::ZeroPage, 2),
    Instruction::new("ROL", 0x36, Mode::ZeroPageX, 2),
    Instruction::new("ROL", 0x2E, Mode::Absolute, 3),
    Instruction::new("ROL", 0x3E, Mode::AbsoluteX, 3),
    //ROR_SET
    Instruction::new("ROR", 0x6A, Mode::Accumulator, 1),
    Instruction::new("ROR", 0x66, Mode::ZeroPage, 2),
    Instruction::new("ROR", 0x76, Mode::ZeroPageX, 2),
    Instruction::new("ROR", 0x6E, Mode::Absolute, 3),
    Instruction::new("ROR", 0x7E, Mode::AbsoluteX, 3),
    //SBC_SET
    Instruction::new("SBC", 0xE9, Mode::Immediate, 2),
    Instruction::new("SBC", 0xE5, Mode::ZeroPage, 2),
    Instruction::new("SBC", 0xF5, Mode::ZeroPageX, 2),
    Instruction::new("SBC", 0xED, Mode::Absolute, 3),
    Instruction::new("SBC", 0xFD, Mode::AbsoluteX, 3),
    Instruction::new("SBC", 0xF9, Mode::AbsoluteY, 3),
    Instruction::new("SBC", 0xE1, Mode::IndirectX, 2),
    Instruction::new("SBC", 0xF1, Mode::IndirectY, 2),
    //STA_SET
    Instruction::new("STA", 0x85, Mode::ZeroPage, 2),
    Instruction::new("STA", 0x95, Mode::ZeroPageX, 2),
    Instruction::new("STA", 0x8D, Mode::Absolute, 3),
    Instruction::new("STA", 0x9D, Mode::AbsoluteX, 3),
    Instruction::new("STA", 0x99, Mode::AbsoluteY, 3),
    Instruction::new("STA", 0x81, Mode::IndirectX, 2),
    Instruction::new("STA", 0x91, Mode::IndirectY, 2),
    //STACK_SET
    Instruction::new("TXS", 0x9A, Mode::Implicit, 1),
    Instruction::new("TSX", 0xBA, Mode::Implicit, 1),
    Instruction::new("PHA", 0x48, Mode::Implicit, 1),
    Instruction::new("PLA", 0x68, Mode::Implicit, 1),
    Instruction::new("PHP", 0x08, Mode::Implicit, 1),
    Instruction::new("PLP", 0x28, Mode::Implicit, 1),
    //STX_SET
    Instruction::new("STX", 0x86, Mode::ZeroPage, 2),
    Instruction::new("STX", 0x96, Mode::ZeroPageY, 2),
    Instruction::new("STX", 0x8E, Mode::Absolute, 3),
    Instruction::new("STY", 0x84, Mode::ZeroPage, 2),
    Instruction::new("STY", 0x94, Mode::ZeroPageX, 2),
    Instruction::new("STY", 0x8C, Mode::Absolute, 3),
    //OTHER_SET
    Instruction::new("BRK", 0x00, Mode::Implicit, 1),
    Instruction::new("JSR", 0x20, Mode::Absolute, 3),
    Instruction::new("NOP", 0xEA, Mode::Implicit, 1),
    Instruction::new("RTI", 0x40, Mode::Implicit, 1),
    Instruction::new("RTS", 0x60, Mode::Implicit, 1)
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
    use super::{Instruction, Mode, INSTRUCTION_MAP};

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
        assert_eq!("BRK", INSTRUCTION_MAP.get(&0x00).unwrap().name)
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
