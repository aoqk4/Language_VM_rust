use crate::instruction::Opcode;

pub mod opcode_parsers;
pub mod operand_parsers;
pub mod register_parsers;

pub mod instruction_parsers;
pub mod program_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}
