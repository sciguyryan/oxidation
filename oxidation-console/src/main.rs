use oxidation_core::virtual_machine::*;
use std::io::{self};

fn main() {
    simple_logger::init().unwrap();

    let mut input_string = String::new();

    let mut vm = VirtualMachine::new();
    //vm.initialize();
    vm.run_test();

    match io::stdin().read_line(&mut input_string) {
        _ => println!(""),
    };
}
