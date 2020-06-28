use crate::cpu::*;
use crate::registers::*;
use crate::security_context::SecurityContext;

type Result<T, E = CpuError> = std::result::Result<T, E>;

pub fn add_lit_reg(cpu: &mut CPU, imm: i32, reg: Registers) -> Result<bool> {
    let reg = cpu.registers.get_register_value(reg, SecurityContext::User);
    match reg {
        Err(_) => {
            return Err(CpuError::RegisterAccessViolation);
        }
        Ok(val) => {
            match val {
                RegisterValue::I32(int) => {
                    let res = cpu.registers.set_register_value(
                        Registers::AC,
                        RegisterValue::I32(imm + int),
                        SecurityContext::User,
                    );

                    if let Err(e) = res {
                        return Err(e);
                    } else {
                        return Ok(false);
                    }
                }
                _ => return Err(CpuError::InvalidRegisterValueType),
            };
        }
    }
}
