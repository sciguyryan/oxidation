use oxidation_core::instructions::enums::Instruction;
use oxidation_core::registers::Registers;
use oxidation_core::virtual_machine::*;
use oxidation_assembler::*;
use std::io::{self};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new();

    let ins = vec![/*Instruction::NOP(), */Instruction::AddLitReg(123, Registers::R1)/*, Instruction::HLT()*/];
    assembler::assemble(ins.as_slice());

    let mut input_string = String::new();

    let mut vm = VirtualMachine::new(64_000, 100, false);
    //vm.initialize();
    //vm.run_test();

    println!("{}", vm.memory.len());

    match io::stdin().read_line(&mut input_string) {
        _ => println!(""),
    };
}
