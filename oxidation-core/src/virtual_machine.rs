use crate::cpu::*;
use log::trace;

pub struct VirtualMachine {
    pub cpu: CPU,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let v = Self { cpu: CPU::new() };

        v
    }

    pub fn initialize(&mut self) {
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
}
