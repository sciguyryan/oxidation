use oxidation_core::virtual_machine::*;
use std::io::{self};

fn main() {
    simple_logger::init().unwrap();

    let mut input_string = String::new();

    let mut vm = VirtualMachine::new(64_000, 100, false);
    //vm.initialize();
    vm.run_test();

    println!("{}", vm.memory.len());

    match io::stdin().read_line(&mut input_string) {
        _ => println!(""),
    };
}
