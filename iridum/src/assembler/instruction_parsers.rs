// use nom::multispace;
use nom::types::CompleteStr;

use super::directive_parsers::directive;
use super::label_parsers::*;
use super::opcode_parsers::*;
// use super::operand_parsers::integer_operand;
use super::operand_parsers::operand;
// use super::register_parsers::register;
use super::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);
impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results = vec![];

        match self.opcode {
            Some(Token::Op { code }) => {
                results.push(code as u8);
            }
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };
        #[allow(clippy::manual_flatten)]
        for operand in &[&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results)
            }
        }
        while results.len() < 4 {
            results.push(0);
        }

        results
    }
    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => results.push(*reg_num),
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;

                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        }
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    #[allow(clippy::collapsible_match)]
    pub fn label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }
}

// named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
//     do_parse!(
//         o: opcode >>
//         r: register >>
//         i: integer_operand >>
//         (
//             AssemblerInstruction{
//                 opcode: o,
//                 operand1: Some(r),
//                 operand2: Some(i),
//                 operand3: None
//             }

//         )
//     )
// );

// named!(pub instruction_two<CompleteStr, AssemblerInstruction>,
//     do_parse!(
//         o: opcode >>
//         opt!(multispace) >>
//         (
//             AssemblerInstruction {
//                 opcode: o,
//                 operand1 : None,
//                 operand2 : None,
//                 operand3 : None,
//             }
//         )

//     )
// );

// named!(pub instruction_thr<CompleteStr, AssemblerInstruction>,
//     do_parse!(
//         o: opcode >>
//         r1: register >>
//         r2: register >>
//         r3: register >>
//         (
//             AssemblerInstruction{
//                 opcode: o,
//                 operand1: Some(r1),
//                 operand2: Some(r2),
//                 operand3: Some(r3),
//             }
//         )
//     )
// );

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction |
            directive
        ) >>
        (
            ins
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Opcode;

    // #[test]
    // fn test_parse_instruction_form_one() {
    //     let result = instruction(CompleteStr("load $0 #100\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 label: None,
    //                 opcode: Some(Token::Op { code: Opcode::LOAD }),
    //                 operand1: Some(Token::Register { reg_num: 0 }),
    //                 operand2: Some(Token::IntegerOperand { value: 100 }),
    //                 operand3: None,
    //                 directive: None,
    //             }
    //         ))
    //     );
    // }

    // #[test]
    // fn test_parse_instruction_form_two() {
    //     let result = instruction(CompleteStr("hlt\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 label: None,
    //                 opcode: Some(Token::Op { code: Opcode::HLT }),
    //                 operand1: None,
    //                 operand2: None,
    //                 operand3: None,
    //                 directive: None,
    //             }
    //         ))
    //     );
    // }

    // #[test]
    // fn test_parse_instruction_form_thr() {
    //     let result = instruction(CompleteStr("add $0 $1 $2\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 label: None,
    //                 opcode: Some(Token::Op { code: Opcode::ADD }),
    //                 operand1: Some(Token::Register { reg_num: 0 }),
    //                 operand2: Some(Token::Register { reg_num: 1 }),
    //                 operand3: Some(Token::Register { reg_num: 2 }),
    //                 directive: None,
    //             }
    //         ))
    //     );
    // }
}
