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

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Register {
    pub register_id: Registers,
    access_flags: RegisterAccess,
    value: i32,
}

impl Register {
    pub fn new(access: RegisterAccess, reg: Registers) -> Self {
        Self {
            register_id: reg,
            access_flags: access,
            value: 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
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
