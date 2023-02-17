use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        // loop {
        //     if self.pc >= self.program.len() {
        //         break;
        //     }
        //     match self.decode_opcode() {
        //         Opcode::HLT => {
        //             println!("HLT encountered");
        //             return;
        //         }
        //         _ => {
        //             println!("Unrecognized opcode found! Terminating!");
        //             return;
        //         }
        //     }
        // }

        let mut is_done = false;

        while !!is_done {
            is_done = self.excute_instruction();
        }
    }

    fn excute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                false
            }
        }
        true
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;

        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 16) | self.program[self.pc + 1] as u16;
        self.pc += 1;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        // let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = vec![0, 0, 0, 0];
        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();

        test_vm.program = vec![200, 0, 0, 1];
        test_vm.run();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![0, 0, 1, 244];

        test_vm.run();

        assert_eq!(test_vm.registers[0], 500);
    }
}
