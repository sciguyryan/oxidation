use crate::instructions::enums::Instruction;
use crate::instructions::implementations as ins_imps;
use crate::registers::*;
use crate::security_context::SecurityContext;
use log::trace;
use snafu::{ensure, Snafu};
//use std::ops::{Index, IndexMut};

type Result<T, E = CpuError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum CpuError {
    #[snafu(display("no memory sequence ID was specified"))]
    MemorySequenceIdNotSet,
    #[snafu(display("an invalid register ID was specified"))]
    InvalidRegisterId,
    #[snafu(display("an attempt was made to access a register with invalid permissions"))]
    RegisterAccessViolation,
    #[snafu(display(
        "an attempt was made to set a register with a value type that it cannot support"
    ))]
    InvalidRegisterValueType,
}

#[derive(Debug)]
pub struct RegisterCollection {
    pub registers: Vec<Register>,
}

pub struct CPU {
    exec_mem_seq_id: i16,
    is_halted: bool,
    pub registers: RegisterCollection,
}

impl RegisterCollection {
    pub fn new() -> Self {
        let mut rc = Self {
            registers: Vec::new(),
        };

        rc.initialize_registers();

        rc
    }

    pub fn get_register_value_ref(
        &self,
        register_id: Registers,
        security_context: SecurityContext,
    ) -> Result<&RegisterValue> {
        match self.get_register_ref(register_id) {
            Err(e) => Err(e),
            Ok(r) => match r.get_value_ref(security_context) {
                Ok(val) => {
                    return Ok(&val);
                }
                Err(_) => {
                    return Err(CpuError::RegisterAccessViolation);
                }
            },
        }
    }

    pub fn get_register_value(
        &self,
        register_id: Registers,
        security_context: SecurityContext,
    ) -> Result<RegisterValue> {
        match self.get_register_ref(register_id) {
            Err(e) => Err(e),
            Ok(r) => match r.get_value(security_context) {
                Ok(val) => {
                    return Ok(val);
                }
                Err(_) => {
                    return Err(CpuError::RegisterAccessViolation);
                }
            },
        }
    }

    pub fn set_register_value(
        &mut self,
        register_id: Registers,
        value: RegisterValue,
        security_context: SecurityContext,
    ) -> Result<()> {
        match self.get_register_mut_ref(register_id) {
            Err(e) => Err(e),
            Ok(r) => match r.set_value(value, security_context) {
                Ok(_) => {
                    return Ok(());
                }
                Err(_) => {
                    return Err(CpuError::RegisterAccessViolation);
                }
            },
        }
    }

    pub fn get_register_ref(&self, register_id: Registers) -> Result<&Register> {
        let reg = self.registers.iter().find(|r| r.register_id == register_id);
        if let Some(r) = reg {
            Ok(r)
        } else {
            Err(CpuError::InvalidRegisterId)
        }
    }

    pub fn get_register_mut_ref(&mut self, register_id: Registers) -> Result<&mut Register> {
        let reg = self
            .registers
            .iter_mut()
            .find(|r| r.register_id == register_id);
        if let Some(r) = reg {
            Ok(r)
        } else {
            Err(CpuError::InvalidRegisterId)
        }
    }

    fn initialize_registers(&mut self) {
        let rw = RegisterAccess::R | RegisterAccess::W;

        self.registers
            .push(Register::new(rw, Registers::R1, RegisterValue::I32(0)));
        self.registers
            .push(Register::new(rw, Registers::R2, RegisterValue::I32(0)));
        self.registers
            .push(Register::new(rw, Registers::R3, RegisterValue::I32(0)));
        self.registers
            .push(Register::new(rw, Registers::R4, RegisterValue::I32(0)));
        self.registers
            .push(Register::new(rw, Registers::R5, RegisterValue::I32(0)));
        self.registers
            .push(Register::new(rw, Registers::R6, RegisterValue::I32(0)));
        self.registers
            .push(Register::new(rw, Registers::R7, RegisterValue::I32(0)));
        self.registers
            .push(Register::new(rw, Registers::R8, RegisterValue::I32(0)));

        self.registers
            .push(Register::new(rw, Registers::AC, RegisterValue::I32(0)));
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            exec_mem_seq_id: -1,
            is_halted: false,
            registers: RegisterCollection::new(),
        }
    }

    /// Initialize the CPU.
    pub fn initialize(&mut self) {}

    /// Run the CPU until the program execution is complete.
    pub fn run(&mut self) -> Result<bool> {
        trace!("Currently in cpu::run.");
        ensure!(self.exec_mem_seq_id > -1, MemorySequenceIdNotSet);

        while !self.is_halted {
            // TODO - execute instruction here.
            self.is_halted = true;
        }

        Ok(true)
    }

    pub fn run_test(&mut self) -> Result<bool> {
        trace!("Currently in cpu::run.");
        //ensure!(self.exec_mem_seq_id > -1, MemorySequenceIdNotSet);

        let ins = Instruction::AddLitReg(123, Registers::R1);

        println!("{:#?}", self.registers);
        self.execute(ins);
        println!("{:#?}", self.registers);

        Ok(true)
    }

    fn fetch_decode(&mut self) {}

    fn execute(&mut self, ins: Instruction) -> Result<bool> {
        trace!("Currently in cpu::execute.");
        println!("Executing: {:?}", ins);
        let halt: Result<bool, CpuError> = match ins {
            Instruction::NOP() => Ok(false),
            Instruction::AddLitReg(lit, reg) => ins_imps::add_lit_reg(self, lit, reg),
            Instruction::HLT() => Ok(true),
        };

        // In the event of an error we will instruct the CPU
        // to halt.
        self.is_halted = match halt {
            Ok(b) => b,
            Err(_) => true,
        };

        halt
    }
}
