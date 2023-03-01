use nom::types::CompleteStr;

use crate::instruction::Opcode;

use self::program_parser::{program, Program};

pub mod directive_parsers;
pub mod instruction_parsers;
pub mod label_parsers;
pub mod opcode_parsers;
pub mod operand_parsers;
pub mod program_parser;
pub mod register_parsers;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_remainder, program)) => {
                self.process_first_phase(&program);
                Some(self.process_second_phase(&program))
            }
            Err(e) => {
                println!("There was an error assembling the code: {:?}", e);
                None
            }
        }
    }
    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];

        for i in &p.instructions {
            let mut bytes = i.to_bytes();
            program.append(&mut bytes);
        }

        program
    }

    fn extract_labels(&mut self, p: &Program) {
        let mut c = 0;

        for i in &p.instructions {
            if i.is_label() {
                match i.label_name() {
                    Some(name) => {
                        let symbol = Symbol::new(name, SymbolType::Label, c);
                        self.symbols.add_symbol(symbol);
                    }
                    None => {}
                }
            }

            c += 4;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}
impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Self {
        Self {
            name,
            offset,
            symbol_type,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { symbols: vec![] }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&mut self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[cfg(test)]
mod tests {

    use crate::vm::VM;

    use super::*;

    // 기호 관련 테스트
    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);

        let v = sym.symbol_value("test");
        assert_eq!(v.is_some(), true);

        let v = v.unwrap();
        assert_eq!(v, 12);

        let v = sym.symbol_value("does_not_exist");
        assert_eq!(v.is_some(), false);
    }

    // 어셈블러 테스트
    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let test_string =
            "load $0 #100\nload $1 #1\nload $2 #0\ntest: inc $0\nneq $0 $2\njeq @test\nhlt";
        let program = asm.assemble(test_string).unwrap();

        println!("{:?}", program);

        let mut vm = VM::new();
        assert_eq!(program.len(), 24);
        vm.add_bytes(program);
        assert_eq!(vm.program.len(), 24);
    }
}
