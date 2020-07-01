use crate::cpu::*;
use crate::memory::*;
use log::trace;

pub struct VirtualMachine {
    pub cpu: CPU,
    pub memory: Memory,
}

impl VirtualMachine {
    pub fn new(memory_size: u32, stack_capacity: u32, cpu_can_swap_regions: bool) -> Self {
        let mut v = Self {
            cpu: CPU::new(),
            memory: Memory::new(memory_size, stack_capacity),
        };
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
