use crate::security_context::SecurityContext;
use snafu::{ensure, Snafu};
use std::fmt;

type Result<T, E = RegisterError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum RegisterError {
    #[snafu(display("invalid access permissions"))]
    RegisterInvalidAccess,
}

bitflags! {
    #[derive(Default)]
    pub struct RegisterAccess: i8 {
        const N = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const PR = 1 << 3;
        const PW = 1 << 3;
    }
}

enum AccessType {
    Read,
    Write,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegisterValue {
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Register {
    pub register_id: Registers,
    access_flags: RegisterAccess,
    value: RegisterValue,
}

impl Register {
    pub fn new(access: RegisterAccess, reg: Registers, reg_type: RegisterValue) -> Self {
        Self {
            register_id: reg,
            access_flags: access,
            value: reg_type,
        }
    }

    /// Returns the value field of the Register struct.
    pub fn get_value(self, security_context: SecurityContext) -> Result<RegisterValue> {
        ensure!(
            self.validate_access(security_context, AccessType::Read),
            RegisterInvalidAccess
        );

        Ok(self.value)
    }

    /// Returns a reference to the value field of the Register struct.
    pub fn get_value_ref(&self, security_context: SecurityContext) -> Result<&RegisterValue> {
        ensure!(
            self.validate_access(security_context, AccessType::Read),
            RegisterInvalidAccess
        );

        Ok(&self.value)
    }

    pub fn set_value(
        &mut self,
        value: RegisterValue,
        security_context: SecurityContext,
    ) -> Result<()> {
        ensure!(
            self.validate_access(security_context, AccessType::Read),
            RegisterInvalidAccess
        );

        self.value = value;

        Ok(())
    }

    fn validate_access(&self, security_context: SecurityContext, access_type: AccessType) -> bool {
        // TODO - make this actually do something
        true
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Registers {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    AC,
    FL,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Registers::R1 => "R1",
            Registers::R2 => "R2",
            Registers::R3 => "R3",
            Registers::R4 => "R4",
            Registers::R5 => "R5",
            Registers::R6 => "R6",
            Registers::R7 => "R7",
            Registers::R8 => "R8",
            Registers::AC => "AC",
            Registers::FL => "FL",
        };
        write!(f, "{}", printable)
    }
}
