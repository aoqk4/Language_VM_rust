# Language_VM_rust

## Build your own X 의 So You Want to Build a Language VM의 코드 실습

## Goal

1. Reasonaly performant compared to modern impl (use Python for comparison)

2. Fault tolerant

3. command and control platform for running applications

4. Clustering of VMs

## Day 1. Starting Point.

1. impl simple VM struct

## Day 2. Basic Operation Code.

1. impl Operation code ( Opcode) & add opcode in VM struct

## Day 3. More Opcode

1. impl ADD, DIV, SUB, MUL and LOAD

## Day 4. impl JUMP

1. impl JUMP & JUMP FRONT

2. test LOAD & JUMP and fixed program code

## Day 5. impl compare Opcode

1. impl EQ, NEQ, GT, LT, GTQ, LTQ, JEQ

2. test code.

## Day 6. REPL

1. structed REPL (in vm)

2. excute vm.program with hex in REPL

## Day 7. nom??

1. cargo add nom

2. How can i use macro ?? & completeStr ???

## Next Goal. use nom & macro

## Day 8. Downgrade nom ver (macro fixed too..) & impl parsers

0. Downgrade nom ver..

1. Impl parsers

## Day 9. Fix instruction_Parser Err (Add Copy, Clone for Opcode)

1. fix Parser & test

2. fix REPL hex to Parser lang.

TODO. Case-sensitive assembler fix...

## Day 10. Clippy & improving assembler & add parser form.

1. cargo clippy & make more clean code.

2. add nop in Opcode & impl and use from(CompleteStr) for Opcode.

3. add instruction_parser form. (for LOAD, ADD, HLT)

## Day 11. Alloc memory

1. Allocate memory VM.

## Day 12. Small fix

1. small bug fix..

## Day 13. Add Assembler Label(+ Usage) & fix REPL

1. Add Label (for reference) and Usage tag

2. Fix Command load_file and clear_program

## Day 14. Symbol Tables

1. add Symbol struct in assembler & test

2. fix stack overflow

## Day 15. yml and cargo add clap

1. add read_yml and cargo clap
