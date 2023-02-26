use crate::instruction::Opcode;
pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    heap: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
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
            heap: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn get_test_vm() -> Self {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;

        test_vm
    }

    pub fn run(&mut self) {
        let mut is_done = false;

        while !is_done {
            is_done = self.excute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.excute_instruction();
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);

        println!("{}", b);
    }

    pub fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);

        self.pc += 1;

        opcode
    }
    fn excute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 == register2 {
                    self.equal_flag = true;
                } else if register1 != register2 {
                    self.equal_flag = false;
                }

                // 저장한 값이 8비트를 먹어서?
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 != register2 {
                    self.equal_flag = true;
                } else if register1 == register2 {
                    self.equal_flag = false;
                }

                // 저장한 값이 8비트를 먹어서?
                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 > register2 {
                    self.equal_flag = true;
                } else if register1 <= register2 {
                    self.equal_flag = false;
                }

                // 저장한 값이 8비트를 먹어서?
                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 < register2 {
                    self.equal_flag = true;
                } else if register1 >= register2 {
                    self.equal_flag = false;
                }

                // 저장한 값이 8비트를 먹어서?
                self.next_8_bits();
            }
            Opcode::GTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 >= register2 {
                    self.equal_flag = true;
                } else if register1 < register2 {
                    self.equal_flag = false;
                }

                // 저장한 값이 8비트를 먹어서?
                self.next_8_bits();
            }
            Opcode::LTQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 <= register2 {
                    self.equal_flag = true;
                } else if register1 > register2 {
                    self.equal_flag = false;
                }

                // 저장한 값이 8비트를 먹어서?
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::ALOC => {
                // 레지스터에 메모리 할당

                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;

                self.heap.resize(new_end as usize, 0);
            }
            Opcode::IGL => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
        }
        false
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;

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
        test_vm.program = vec![5, 0, 0, 0];
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
        let mut test_vm = VM::new();

        test_vm.program = vec![0, 0, 1, 244];

        test_vm.run();

        for i in 0..test_vm.registers.len() {
            println!("{}", test_vm.registers[i]);
        }

        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_add_opcode() {
        // 5 + 10
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        // 10 - 5
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![2, 1, 0, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        // 5 * 10
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_div_opcode() {
        // 5 * 10
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![4, 1, 0, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 20;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;

        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 10;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;

        test_vm.program = vec![14, 1, 0, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;

        test_vm.program = vec![13, 0, 1, 0, 13, 1, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gtq_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;

        test_vm.program = vec![11, 1, 0, 0, 11, 1, 0, 0, 11, 0, 1, 0];

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 255;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_ltq_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 11;

        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 1, 0, 0];

        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[0] = 11;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 255;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 512;

        test_vm.program = vec![17, 0, 0, 0];

        test_vm.run_once();

        assert_eq!(test_vm.heap.len(), 512);
    }
}
