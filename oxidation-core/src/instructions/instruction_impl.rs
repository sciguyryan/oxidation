use crate::cpu::CPU;
use crate::instructions::instruction_data::*;
use crate::opcodes::OpCode;
use crate::security_context::SecurityContext;

pub enum ArgumentTypes {
    /// A 16-bit integer value.
    Int16,
    /// A 32-bit integer value.
    Int32,
    /// A 64-bit integer value (long).
    Int64,
    /// A floating point value.
    Float,
    /// A register identifier.
    Register,
    /// A string.
    String,
    /// An instruction size hint identifier.
    InstructionSizeHint,
}

pub enum ArgumentRefTypes {
    /// <summary>
    /// The argument is a register identifier.
    /// </summary>
    Register,
    /// <summary>
    /// The argument is a literal integer.
    /// </summary>
    LiteralInteger,
    /// <summary>
    /// The argument is a literal float.
    /// </summary>
    LiteralFloat,
    /// <summary>
    /// The argument is a register pointer.
    /// </summary>
    RegisterPointer,
    /// <summary>
    /// The argument is a literal pointer.
    /// </summary>
    LiteralPointer,
    /// <summary>
    /// The argument is an expression.
    /// </summary>
    Expression,
    /// <summary>
    /// The argument is a string.
    /// </summary>
    String,
    /// <summary>
    /// An argument indicating the size of the instruction.
    /// </summary>
    InstructionSizeHint,
}

pub trait Instruction {
    fn get_security_context() -> SecurityContext;
    fn execute(data: InstructionData, cpu: CPU) -> bool;
    fn to_string(&self, data: InstructionData) -> String;
}

struct AddRegReg {
    pub output_literals_as_hex: bool,
    pub argument_types: Vec<ArgumentTypes>,
    pub argument_ref_types: Vec<ArgumentRefTypes>,
    pub op_code: OpCode,
    pub asm_name: String,
}

impl AddRegReg {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            output_literals_as_hex: true,
            argument_types: vec![ArgumentTypes::Register, ArgumentTypes::Register],
            argument_ref_types: vec![ArgumentRefTypes::Register, ArgumentRefTypes::Register],
            op_code: OpCode::AddRegReg,
            asm_name: "add".to_string(),
        }
    }
}

impl Instruction for AddRegReg {
    #[allow(dead_code)]
    fn get_security_context() -> SecurityContext {
        SecurityContext::User
    }

    #[allow(dead_code, unused_variables)]
    fn execute(data: InstructionData, cpu: CPU) -> bool {
        false
    }

    #[allow(dead_code, unused_variables)]
    fn to_string(&self, data: InstructionData) -> String {
        format!("{} foo bar", self.asm_name)
    }
}
