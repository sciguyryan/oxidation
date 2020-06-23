use crate::registers::*;
use log::trace;
use snafu::{ensure, Snafu};
use std::collections::HashMap;

type Result<T, E = CpuError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum CpuError {
    #[snafu(display("no memory sequence ID specified"))]
    MemorySequenceIdNotSet,
}

pub struct CPU {
    exec_mem_seq_id: i16,
    is_halted: bool,
    pub registers: RegisterCollection,
}

#[derive(Debug)]
pub struct RegisterCollection {
    registers: HashMap<Registers, Register>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            exec_mem_seq_id: -1,
            is_halted: false,
            registers: RegisterCollection::new(),
        }
    }

    pub fn initialize() {}

    pub fn run(&mut self) -> Result<bool> {
        trace!("Currently in cpu::run.");
        /*if self.exec_mem_seq_id == -1 {
            return MemorySequenceIdNotSet
                .fail()
                .map_err(core::convert::Into::into);
        }*/
        ensure!(self.exec_mem_seq_id > -1, MemorySequenceIdNotSet);

        while !self.is_halted {
            self.is_halted = true;
        }

        Ok(true)
    }
}

impl RegisterCollection {
    pub fn new() -> Self {
        let mut v = Self {
            registers: HashMap::new(),
        };

        // Initialize and configure the registers.
        v.initialize_registers();

        v
    }

    fn initialize_registers(&mut self) {
        let rw = RegisterAccess::R | RegisterAccess::W;

        self.registers
            .insert(Registers::R1, Register::new(rw, Registers::R1));
        self.registers
            .insert(Registers::R2, Register::new(rw, Registers::R2));
        self.registers
            .insert(Registers::R3, Register::new(rw, Registers::R3));
        self.registers
            .insert(Registers::R4, Register::new(rw, Registers::R4));
        self.registers
            .insert(Registers::R5, Register::new(rw, Registers::R5));
        self.registers
            .insert(Registers::R6, Register::new(rw, Registers::R6));
        self.registers
            .insert(Registers::R7, Register::new(rw, Registers::R7));
        self.registers
            .insert(Registers::R8, Register::new(rw, Registers::R8));
    }
}
