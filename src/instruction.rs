use lazy_static::lazy_static;

#[derive(Debug)]
enum Mode {
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
    Accumulator
}

//Temporary
#[allow(dead_code)]
#[derive(Debug)]
pub struct Instruction {
    name: &'static str,
    opcode: u8,
    mode: Mode,
    len: u8
}
const ADC_SET: [Instruction; 8] = [
    Instruction{name: "ADC", opcode: 0x69, mode: Mode::Immediate,   len: 2},
    Instruction{name: "ADC", opcode: 0x65, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "ADC", opcode: 0x75, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "ADC", opcode: 0x6D, mode: Mode::Absolute,    len: 3},
    Instruction{name: "ADC", opcode: 0x7D, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "ADC", opcode: 0x79, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "ADC", opcode: 0x61, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "ADC", opcode: 0x71, mode: Mode::IndirectY,   len: 2},
];

const AND_SET: [Instruction; 8] = [
    Instruction{name: "AND", opcode: 0x29, mode: Mode::Immediate,   len: 2},
    Instruction{name: "AND", opcode: 0x25, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "AND", opcode: 0x35, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "AND", opcode: 0x2D, mode: Mode::Absolute,    len: 3},
    Instruction{name: "AND", opcode: 0x3D, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "AND", opcode: 0x39, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "AND", opcode: 0x21, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "AND", opcode: 0x31, mode: Mode::IndirectY,   len: 2},
];

const ASL_SET: [Instruction; 5] = [
    Instruction{name: "ASL", opcode: 0x0A, mode: Mode::Accumulator, len: 1},
    Instruction{name: "ASL", opcode: 0x06, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "ASL", opcode: 0x16, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "ASL", opcode: 0x0E, mode: Mode::Absolute,    len: 3},
    Instruction{name: "ASL", opcode: 0x1E, mode: Mode::AbsoluteX,   len: 3},
];

const BIT_SET: [Instruction; 2] = [
    Instruction{name: "BIT", opcode: 0x24, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "BIT", opcode: 0x2C, mode: Mode::Absolute,    len: 3},
];

const BRANCHES_SET: [Instruction; 8] = [
    Instruction{name: "BPL", opcode: 0x10, mode: Mode::Implicit,    len: 1},
    Instruction{name: "BMI", opcode: 0x30, mode: Mode::Implicit,    len: 1},
    Instruction{name: "BVC", opcode: 0x50, mode: Mode::Implicit,    len: 1},
    Instruction{name: "BVS", opcode: 0x70, mode: Mode::Implicit,    len: 1},
    Instruction{name: "BCC", opcode: 0x90, mode: Mode::Implicit,    len: 1},
    Instruction{name: "BCS", opcode: 0xB0, mode: Mode::Implicit,    len: 1},
    Instruction{name: "BNE", opcode: 0xD0, mode: Mode::Implicit,    len: 1},
    Instruction{name: "BEQ", opcode: 0xF0, mode: Mode::Implicit,    len: 1}
];

const CMP_SET: [Instruction; 8] = [
    Instruction{name: "CMP", opcode: 0xC9, mode: Mode::Immediate,   len: 2},
    Instruction{name: "CMP", opcode: 0xC5, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "CMP", opcode: 0xD5, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "CMP", opcode: 0xCD, mode: Mode::Absolute,    len: 3},
    Instruction{name: "CMP", opcode: 0xDD, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "CMP", opcode: 0xD9, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "CMP", opcode: 0xC1, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "CMP", opcode: 0xD1, mode: Mode::IndirectY,   len: 2},
];

const CPX_SET: [Instruction; 3] = [
    Instruction{name: "CPX", opcode: 0xE0, mode: Mode::Immediate,   len: 2},
    Instruction{name: "CPX", opcode: 0xE4, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "CPX", opcode: 0xEC, mode: Mode::Absolute,    len: 3},
];

const CPY_SET: [Instruction; 3] = [
    Instruction{name: "CPY", opcode: 0xC0, mode: Mode::Immediate,   len: 2},
    Instruction{name: "CPY", opcode: 0xC4, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "CPY", opcode: 0xCC, mode: Mode::Absolute,    len: 3},
];

const DEC_SET: [Instruction; 4] = [
    Instruction{name: "DEC", opcode: 0xC6, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "DEC", opcode: 0xD6, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "DEC", opcode: 0xCE, mode: Mode::Absolute,    len: 3},
    Instruction{name: "DEC", opcode: 0xDE, mode: Mode::AbsoluteX,   len: 3},
];

const EOR_SET: [Instruction; 8] = [
    Instruction{name: "EOR", opcode: 0x49, mode: Mode::Immediate,   len: 2},
    Instruction{name: "EOR", opcode: 0x45, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "EOR", opcode: 0x55, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "EOR", opcode: 0x4D, mode: Mode::Absolute,    len: 3},
    Instruction{name: "EOR", opcode: 0x5D, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "EOR", opcode: 0x59, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "EOR", opcode: 0x41, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "EOR", opcode: 0x51, mode: Mode::IndirectY,   len: 2},
];

const PROC_STATUS_SET: [Instruction; 7] = [
    Instruction{name: "CLC", opcode: 0x18, mode: Mode::Implicit,    len: 1},
    Instruction{name: "SEC", opcode: 0x38, mode: Mode::Implicit,    len: 1},
    Instruction{name: "CLI", opcode: 0x58, mode: Mode::Implicit,    len: 1},
    Instruction{name: "SEI", opcode: 0x78, mode: Mode::Implicit,    len: 1},
    Instruction{name: "CLV", opcode: 0xB8, mode: Mode::Implicit,    len: 1},
    Instruction{name: "CLD", opcode: 0xD8, mode: Mode::Implicit,    len: 1},
    Instruction{name: "SED", opcode: 0xF8, mode: Mode::Implicit,    len: 1},
];

const INC_SET: [Instruction; 4] = [
    Instruction{name: "INC", opcode: 0xE6, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "INC", opcode: 0xF6, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "INC", opcode: 0xEE, mode: Mode::Absolute,    len: 3},
    Instruction{name: "INC", opcode: 0xFE, mode: Mode::AbsoluteX,   len: 3},
];

const JMP_SET: [Instruction; 2] = [
    Instruction{name: "JMP", opcode: 0x4C, mode: Mode::Absolute,    len: 3},
    Instruction{name: "JMP", opcode: 0x6C, mode: Mode::Indirect,    len: 3},
];

const LDA_SET: [Instruction; 8] = [
    Instruction{name: "LDA", opcode: 0xA9, mode: Mode::Immediate,   len: 2},
    Instruction{name: "LDA", opcode: 0xA5, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "LDA", opcode: 0xB5, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "LDA", opcode: 0xAD, mode: Mode::Absolute,    len: 3},
    Instruction{name: "LDA", opcode: 0xBD, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "LDA", opcode: 0xB9, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "LDA", opcode: 0xA1, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "LDA", opcode: 0xB1, mode: Mode::IndirectY,   len: 2},
];

const LDX_SET: [Instruction; 5] = [
    Instruction{name: "LDX", opcode: 0xA2, mode: Mode::Immediate,   len: 2},
    Instruction{name: "LDX", opcode: 0xA6, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "LDX", opcode: 0xB6, mode: Mode::ZeroPageY,   len: 2},
    Instruction{name: "LDX", opcode: 0xAE, mode: Mode::Absolute,    len: 3},
    Instruction{name: "LDX", opcode: 0xBE, mode: Mode::AbsoluteY,   len: 3},
];

const LDY_SET: [Instruction; 5] = [
    Instruction{name: "LDY", opcode: 0xA0, mode: Mode::Immediate,   len: 2},
    Instruction{name: "LDY", opcode: 0xA4, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "LDY", opcode: 0xB4, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "LDY", opcode: 0xAC, mode: Mode::Absolute,    len: 3},
    Instruction{name: "LDY", opcode: 0xBC, mode: Mode::AbsoluteX,   len: 3},
];

const LSR_SET: [Instruction; 5] = [
    Instruction{name: "LSR", opcode: 0x4A, mode: Mode::Accumulator, len: 1},
    Instruction{name: "LSR", opcode: 0x46, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "LSR", opcode: 0x56, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "LSR", opcode: 0x4E, mode: Mode::Absolute,    len: 3},
    Instruction{name: "LSR", opcode: 0x5E, mode: Mode::AbsoluteX,   len: 3},
];

const ORA_SET: [Instruction; 8] = [
    Instruction{name: "ORA", opcode: 0x09, mode: Mode::Immediate,   len: 2},
    Instruction{name: "ORA", opcode: 0x05, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "ORA", opcode: 0x15, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "ORA", opcode: 0x0D, mode: Mode::Absolute,    len: 3},
    Instruction{name: "ORA", opcode: 0x1D, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "ORA", opcode: 0x19, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "ORA", opcode: 0x01, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "ORA", opcode: 0x11, mode: Mode::IndirectY,   len: 2}
];

const REGISTER_SET: [Instruction; 8] = [
    Instruction{name: "TAX", opcode: 0xAA, mode: Mode::Implicit,    len: 1},
    Instruction{name: "TXA", opcode: 0x8A, mode: Mode::Implicit,    len: 1},
    Instruction{name: "DEX", opcode: 0xCA, mode: Mode::Implicit,    len: 1},
    Instruction{name: "INX", opcode: 0xE8, mode: Mode::Implicit,    len: 1},
    Instruction{name: "TAY", opcode: 0xA8, mode: Mode::Implicit,    len: 1},
    Instruction{name: "TYA", opcode: 0x98, mode: Mode::Implicit,    len: 1},
    Instruction{name: "DEY", opcode: 0x88, mode: Mode::Implicit,    len: 1},
    Instruction{name: "INY", opcode: 0xC8, mode: Mode::Implicit,    len: 1}
];

const ROL_SET: [Instruction; 5] = [
    Instruction{name: "ROL", opcode: 0x2A, mode: Mode::Accumulator, len: 1},
    Instruction{name: "ROL", opcode: 0x26, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "ROL", opcode: 0x36, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "ROL", opcode: 0x2E, mode: Mode::Absolute,    len: 3},
    Instruction{name: "ROL", opcode: 0x3E, mode: Mode::AbsoluteX,   len: 3},
];

const ROR_SET: [Instruction; 5] = [
    Instruction{name: "ROR", opcode: 0x6A, mode: Mode::Accumulator, len: 1},
    Instruction{name: "ROR", opcode: 0x66, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "ROR", opcode: 0x76, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "ROR", opcode: 0x6E, mode: Mode::Absolute,    len: 3},
    Instruction{name: "ROR", opcode: 0x7E, mode: Mode::AbsoluteX,   len: 3},
];

const SBC_SET: [Instruction; 8] = [
    Instruction{name: "SBC", opcode: 0xE9, mode: Mode::Immediate,   len: 2},
    Instruction{name: "SBC", opcode: 0xE5, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "SBC", opcode: 0xF5, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "SBC", opcode: 0xED, mode: Mode::Absolute,    len: 3},
    Instruction{name: "SBC", opcode: 0xFD, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "SBC", opcode: 0xF9, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "SBC", opcode: 0xE1, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "SBC", opcode: 0xF1, mode: Mode::IndirectY,   len: 2}
];

const STA_SET: [Instruction; 7] = [
    Instruction{name: "STA", opcode: 0x85, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "STA", opcode: 0x95, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "STA", opcode: 0x8D, mode: Mode::Absolute,    len: 3},
    Instruction{name: "STA", opcode: 0x9D, mode: Mode::AbsoluteX,   len: 3},
    Instruction{name: "STA", opcode: 0x99, mode: Mode::AbsoluteY,   len: 3},
    Instruction{name: "STA", opcode: 0x81, mode: Mode::IndirectX,   len: 2},
    Instruction{name: "STA", opcode: 0x91, mode: Mode::IndirectY,   len: 2}
];

const STACK_SET: [Instruction; 6] = [
    Instruction{name: "TXS", opcode: 0x9A, mode: Mode::Implicit,    len: 1},
    Instruction{name: "TSX", opcode: 0xBA, mode: Mode::Implicit,    len: 1},
    Instruction{name: "PHA", opcode: 0x48, mode: Mode::Implicit,    len: 1},
    Instruction{name: "PLA", opcode: 0x68, mode: Mode::Implicit,    len: 1},
    Instruction{name: "PHP", opcode: 0x08, mode: Mode::Implicit,    len: 1},
    Instruction{name: "PLP", opcode: 0x28, mode: Mode::Implicit,    len: 1}
];

const STX_SET: [Instruction; 3] = [
    Instruction{name: "STX", opcode: 0x86, mode: Mode::ZeroPage,    len: 2}, // 0x86
    Instruction{name: "STX", opcode: 0x96, mode: Mode::ZeroPageY,   len: 2},
    Instruction{name: "STX", opcode: 0x8E, mode: Mode::Absolute,    len: 3},
];

const STY_SET: [Instruction; 3] = [
    Instruction{name: "STY", opcode: 0x84, mode: Mode::ZeroPage,    len: 2},
    Instruction{name: "STY", opcode: 0x94, mode: Mode::ZeroPageX,   len: 2},
    Instruction{name: "STY", opcode: 0x8C, mode: Mode::Absolute,    len: 3},
];

const BRK: Instruction = Instruction{name: "BRK", opcode: 0x00, mode: Mode::Implicit, len: 1};
const JSR: Instruction = Instruction{name: "JSR", opcode: 0x20, mode: Mode::Absolute, len: 3};
const NOP: Instruction = Instruction{name: "NOP", opcode: 0xEA, mode: Mode::Implicit, len: 1};
const RTI: Instruction = Instruction{name: "RTI", opcode: 0x40, mode: Mode::Implicit, len: 1};
const RTS: Instruction = Instruction{name: "RTS", opcode: 0x60, mode: Mode::Implicit, len: 1};

lazy_static! {
    static ref INSTRUCTIONS: Vec<&'static Instruction> = {
        let mut v = Vec::new();
        v.push(&BRK);
        v.push(&JSR);
        v.push(&NOP);
        v.push(&RTI);
        v.push(&RTS);
        for instruction in ADC_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in AND_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in ASL_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in BIT_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in BRANCHES_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in CMP_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in CPX_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in CPY_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in DEC_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in EOR_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in PROC_STATUS_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in INC_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in JMP_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in LDA_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in LDX_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in LDY_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in LSR_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in ORA_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in REGISTER_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in ROL_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in ROR_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in SBC_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in STA_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in STACK_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in STX_SET.iter()
        {
            v.push(instruction);
        }
        for instruction in STY_SET.iter()
        {
            v.push(instruction);
        }
        v
    };

    pub static ref INSTRUCTION_MAP : std::collections::HashMap<u8, &'static Instruction> = {
        let mut m = std::collections::HashMap::<u8,&'static Instruction>::new();
        for instruction in INSTRUCTIONS.iter() {
            if let Some(_) = m.insert(instruction.opcode, instruction) {
                panic!("INSTRUCTION_MAP : Error, opcode already defined adding instruction : {:#?}", instruction)
            }
        }
        m
    };
}

#[cfg(test)]
mod tests {
    use super::{INSTRUCTION_MAP, Mode, Instruction};

    fn get_lenght_from_mode(mode: &Mode) -> u8
    {
        match mode {
            Mode::Immediate   => 2,
            Mode::ZeroPage    => 2,
            Mode::ZeroPageX   => 2,
            Mode::ZeroPageY   => 2,
            Mode::Absolute    => 3,
            Mode::AbsoluteX   => 3,
            Mode::AbsoluteY   => 3,
            Mode::Indirect    => 3,
            Mode::IndirectX   => 2,
            Mode::IndirectY   => 2,
            Mode::Implicit    => 1,
            Mode::Accumulator => 1
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
            .map(|(_,i)| i)
            .filter(|i| i.len != get_lenght_from_mode(&i.mode))
            .collect();
        assert_eq!(0, bad_instructions.len())
    }
}