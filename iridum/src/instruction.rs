#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    IGL,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => return Opcode::HLT,
            _ => return Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Self {
        Instruction { opcode: opcode }
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
}
