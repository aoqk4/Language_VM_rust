use nom::types::CompleteStr;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    JMP,
    JMPF,
    JMPB,
    EQ,
    NEQ,
    GTQ,
    LTQ,
    LT,
    GT,
    JEQ,
    NOP,
    ALOC,
    INC,
    DEC,
    IGL,
}
impl From<Opcode> for u8 {
    fn from(op: Opcode) -> Self {
        match op {
            Opcode::LOAD => 0,
            Opcode::ADD => 1,
            Opcode::SUB => 2,
            Opcode::MUL => 3,
            Opcode::DIV => 4,
            Opcode::HLT => 5,
            Opcode::JMP => 6,
            Opcode::JMPF => 7,
            Opcode::JMPB => 8,
            Opcode::EQ => 9,
            Opcode::NEQ => 10,
            Opcode::GTQ => 11,
            Opcode::LTQ => 12,
            Opcode::LT => 13,
            Opcode::GT => 14,
            Opcode::JEQ => 15,
            Opcode::NOP => 16,
            Opcode::ALOC => 17,
            Opcode::INC => 18,
            Opcode::DEC => 19,
            Opcode::IGL => 100,
        }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GTQ,
            12 => Opcode::LTQ,
            13 => Opcode::LT,
            14 => Opcode::GT,
            15 => Opcode::JEQ,
            16 => Opcode::NOP,
            17 => Opcode::ALOC,
            18 => Opcode::INC,
            19 => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(value: CompleteStr<'a>) -> Self {
        match value {
            CompleteStr("load") => Opcode::LOAD,
            CompleteStr("add") => Opcode::ADD,
            CompleteStr("sub") => Opcode::SUB,
            CompleteStr("mul") => Opcode::MUL,
            CompleteStr("div") => Opcode::DIV,
            CompleteStr("hlt") => Opcode::HLT,
            CompleteStr("jmp") => Opcode::JMP,
            CompleteStr("jmpf") => Opcode::JMPF,
            CompleteStr("jmpb") => Opcode::JMPB,
            CompleteStr("eq") => Opcode::EQ,
            CompleteStr("neq") => Opcode::NEQ,
            CompleteStr("gtq") => Opcode::GTQ,
            CompleteStr("ltq") => Opcode::LTQ,
            CompleteStr("lt") => Opcode::LT,
            CompleteStr("gt") => Opcode::GT,
            CompleteStr("jeq") => Opcode::JEQ,
            CompleteStr("nop") => Opcode::NOP,
            CompleteStr("aloc") => Opcode::ALOC,
            CompleteStr("inc") => Opcode::INC,
            CompleteStr("dec") => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instuction() {
        let instrcution = Instruction::new(Opcode::HLT);
        assert_eq!(instrcution.opcode, Opcode::HLT);
    }

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);

        let opcode = Opcode::from(CompleteStr("igl"));
        assert_eq!(opcode, Opcode::IGL);
    }
}
