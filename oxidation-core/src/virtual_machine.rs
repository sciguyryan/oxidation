use crate::cpu::*;
use log::trace;

pub struct VirtualMachine {
    pub cpu: CPU,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut v = Self { cpu: CPU::new() };
        v.initialize();
        v
    }

    pub fn initialize(&mut self) {
        self.cpu.initialize();
        //trace!("hello from initialize!");
        //trace!("{:?}", std::any::type_name::<crate::registers::Registers>());
    }

    pub fn run(&mut self) {
        trace!("Currently in VirtualMachine::run");

        //println!("{:#?}", self.cpu.registers);

        // TODO - handle errors a bit better here.
        if let Err(e) = self.cpu.run() {
            println!("{}", e);
        } else {
            println!("successfully ran the CPU to completion.");
        }
    }

    pub fn run_test(&mut self) {
        trace!("Currently in VirtualMachine::run_test");
        if let Err(e) = self.cpu.run_test() {
            println!("{}", e);
        } else {
            println!("successfully ran the CPU to completion.");
        }
    }
}
