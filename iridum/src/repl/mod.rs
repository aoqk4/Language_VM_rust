use nom::types::CompleteStr;

use crate::assembler::program_parser::program;
use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    // 위키 눌렀을 때? 명령 그래도 유지하기 위해서
    command_buffer: Vec<String>,
    // REPL 구조체가 VM 구조체를 소유한다.
    vm: VM,
}

impl REPL {
    pub fn new() -> Self {
        Self {
            command_buffer: vec![],
            vm: VM::new(),
        }
    }

    // "0x ~~" 를 받지 않고 16진수 받아서 vec of u8로 리턴
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();

        let mut results: Vec<u8> = vec![];

        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(res) => {
                    results.push(res);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    pub fn run(&mut self) {
        println!("Welcome to Iridum! Let's be productive!");
        loop {
            // 나중에는 밖에서 생성하고 재사용 할 수 있도록 한다.
            let mut buffer = String::new();

            let _stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            _stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Farewall! Have a great day!");
                    std::process::exit(0);
                }
                ".history" => {
                    for com in &self.command_buffer {
                        println!("{}", com);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");

                    for ins in &self.vm.program {
                        println!("{}", ins);
                    }

                    println!("End of Program Listing");
                }
                ".registers" => {
                    println!("Listing registers and all contents : ");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing");
                }
                _ => {
                    let parsed_program = program(CompleteStr(buffer));

                    if parsed_program.is_err() {
                        println!("Unable to parse input");
                        continue;
                    }

                    let (_, result) = parsed_program.unwrap();

                    let bytecode = result.to_bytes();

                    // TODO: Make a function to let us add bytes to the VM

                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }

                    self.vm.run_once();
                }
            }
        }
    }
}
