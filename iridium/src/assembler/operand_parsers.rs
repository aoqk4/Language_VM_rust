use super::label_parsers::label_usage;
use nom::digit;
use nom::types::CompleteStr;

use crate::assembler::register_parsers::register;
use crate::assembler::Token;

named!(
    pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        label_usage |
        register
    )
);

named!(
    pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            reg_num: digit >>
            (
                Token::IntegerOperand { value: reg_num.parse::<i32>().unwrap() }
            )
        )
    )
);

// named!(irstring<CompleteStr, Token>,
//     do_parse!(
//         tag!("'") >>
//         content: take_until!("'") >>
//         tag!("'") >>
//         (
//             Token::IrString{ name: content.to_string() }
//         )
//     )
// );

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let result = integer_operand(CompleteStr("#255"));
        assert_eq!(result.is_ok(), true);

        let (rest, value) = result.unwrap();
        assert_eq!(rest, CompleteStr(""));
        assert_eq!(value, Token::IntegerOperand { value: 255 });

        let result = integer_operand(CompleteStr("10"));
        assert_eq!(result.is_ok(), false);
    }

    // #[test]
    // fn test_parse_string_operand() {
    //     let result = irstring(CompleteStr("'This is a test'"));
    //     assert_eq!(result.is_ok(), true);
    // }
}
