use crate::registers::Registers;
use std::fmt;

pub enum ArgumentTypes {
    /// A 16-bit integer value.
    ImmediateInt16,
    /// A 32-bit integer value.
    ImmediateInt32,
    /// A 64-bit integer value (long).
    ImmediateInt64,
    /// A floating point value.
    ImmediateFloat,
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
    /// The argument is a literal (immediate) integer.
    /// </summary>
    ImmediateInteger,
    /// <summary>
    /// The argument is a literal (immediate) float.
    /// </summary>
    ImmediateFloat,
    /// <summary>
    /// The argument is a register pointer.
    /// </summary>
    RegisterPointer,
    /// <summary>
    /// The argument is a literal (immediate) pointer.
    /// </summary>
    ImmediatePointer,
    /// <summary>
    /// The argument is a string.
    /// </summary>
    String,
    /// <summary>
    /// An argument indicating the size of the instruction.
    /// </summary>
    InstructionSizeHint,
}

#[repr(i16)]
pub enum InstructionSizeHint {
    /// <summary>
    /// Instruction data should be of size: 1 byte (4 bits)
    /// </summary>
    Byte,
    /// <summary>
    /// Instruction data should be of size: 4 bytes (32 bits)
    /// </summary>
    Word,
    /// <summary>
    /// Instruction data should be of size: 8 bytes (64 bits)
    /// </summary>
    DWord,
}

#[derive(Debug)]
pub enum Instruction {
    //Subroutine(i32, String),
    NOP(),
    AddLitReg(i32, Registers),
    HLT(),
}

#[repr(i16)]
pub enum OpCode {
    /// <summary>
    /// Subroutine - a pseudo-opcode used to identify a
    /// subroutine position.
    /// </summary>
    Subroutine = -2,
    /// <summary>
    /// Label - a pseudo-opcode used to identify a labels position.
    /// </summary>
    Label = -1,

    /// <summary>
    /// No Operation - a non-operation instruction.
    /// No action to be performed.
    /// </summary>
    NOP,

    /// <summary>
    /// Move Literal to Register - copy a literal into a register.
    /// </summary>
    MovLitReg,
    /// <summary>
    /// Move Register to Register - copy the value of register A
    /// into register B.
    /// </summary>
    MovRegReg,
    /// <summary>
    /// Move Register to Memory - copy the value of the register
    /// into memory.
    /// </summary>
    MovRegMem,
    /// <summary>
    /// Move Memory to Register - copy a value from memory
    /// into the register.
    /// </summary>
    MovMemReg,
    /// <summary>
    /// Move Literal to Memory - copy a literal into memory.
    /// </summary>
    MovLitMem,
    /// <summary>
    /// Move Register* to Register -
    /// copy an integer value from memory into Register B
    /// starting from the index given by the value of register A.
    /// </summary>
    MovRegPtrReg,
    /// <summary>
    /// Move Register* to Register (Type Hint) -
    /// copy a value from memory into Register B starting
    /// from the index given by the value of register A.
    /// </summary>
    MovHRegPtrReg,
    /// <summary>
    /// Move from Memory via Literal Offset to Register -
    /// copy a value from memory starting from the index
    /// given by a literal plus the value of register A
    /// into register B.
    /// </summary>
    MovLitOffReg,
    /// <summary>
    /// Swap - swap the value of registers A and B.
    /// </summary>
    Swap,

    /// <summary>
    /// Add Register to Register - add the contents of
    /// register A into register B.
    /// Result is moved into the accumulator.
    /// </summary>
    AddRegReg,
    /// <summary>
    /// Add Literal to Register - add a literal to the
    /// value of the register.
    /// Result is moved into the accumulator.
    /// </summary>
    AddLitReg,
    /// <summary>
    /// Subtract Literal from Register - subtract a literal
    /// from the value of the register.
    /// Result is moved into the accumulator.
    /// </summary>
    SubLitReg,
    /// <summary>
    /// Subtract Register from Literal - subtract the value
    /// of the register from a literal.
    /// Result is moved into the accumulator.
    /// </summary>
    SubRegLit,
    /// <summary>
    /// Subtract Register from Register - subtract the value
    /// of register B from the value of register A.
    /// Result is moved into the accumulator.
    /// </summary>
    SubRegReg,
    /// <summary>
    /// Increment Register - increment the value of the register.
    /// </summary>
    IncReg,
    /// <summary>
    /// Decrement Register - decrement the value of the register.
    /// </summary>
    DegReg,
    /// <summary>
    /// Multiply Literal by Register - multiply a literal by the
    /// value of the register.
    /// Result is moved into the accumulator.
    /// </summary>
    MulLitReg,
    /// <summary>
    /// Multiply Register by Register - multiply the value of
    /// register A by the value of register B.
    /// Result is moved into the accumulator.
    /// </summary>
    MulRegReg,
    /// <summary>
    /// Modulo Register by Literal - modulo the value of a
    /// register by a literal.
    /// Result is moved into the accumulator.
    /// </summary>
    ModLitReg,
    /// <summary>
    /// Modulo Literal by Register - modulo a literal by
    /// the value of a register.
    /// Result is moved into the accumulator.
    /// </summary>
    MocRegLit,
    /// <summary>
    /// Modulo Register by Register - modulo the value of
    /// register B by the value of register A.
    /// Result is moved into the accumulator.
    /// </summary>
    MocRegReg,

    /// <summary>
    /// Bit Test - test is a given bit within a register is set.
    /// The Zero (Z) flag will be set to the value of the bit.
    /// </summary>
    Bit,
    /// <summary>
    /// Left Shift a register value by a literal.
    /// Calculation is performed directly on the register.
    /// </summary>
    LsfRegLit,
    /// <summary>
    /// Left Shift the value of register A by the value of register B.
    /// Calculation is performed directly on register A.
    /// </summary>
    LsfRegReg,
    /// <summary>
    /// Right Shift a register value by a literal.
    /// Calculation is performed directly on the register.
    /// </summary>
    RsfRegLit,
    /// <summary>
    /// Right Shift the value of register A by the value
    /// of register B.
    /// Calculation is performed directly on register A.
    /// </summary>
    RsfRegReg,
    /// <summary>
    /// Add Register to Literal - add the value of
    /// register A to a literal.
    /// Result is moved into the accumulator.
    /// </summary>
    AndRegLit,
    /// <summary>
    /// Add Register to Register - add the value of register A
    /// to the value of register B.
    /// Result is moved into the accumulator.
    /// </summary>
    AndRegReg,
    /// <summary>
    /// Or Register and Literal - bitwise OR the value of
    /// register A with a literal.
    /// Result is moved into the accumulator.
    /// </summary>
    OrRegLit,
    /// <summary>
    /// Or Register and Register - bitwise OR the value of
    /// register A with the value of register B.
    /// Result is moved into the accumulator.
    /// </summary>
    OrRegReg,
    /// <summary>
    /// XOR Register and Literal - bitwise XOR the value of
    /// register A with a literal.
    /// Result is moved into the accumulator.
    /// </summary>
    XorRegLit,
    /// <summary>
    /// XOR Register and Register - bitwise XOR the value of
    /// register A with the value of register B.
    /// Result is moved into the accumulator.
    /// </summary>
    XorRegReg,
    /// <summary>
    /// NOT - bitwise NOT the value of register A.
    /// Result is moved into the accumulator.
    /// </summary>
    Not,

    /// <summary>
    /// Jump If Not Equal To Literal - if the accumulator is not equal
    /// to the literal A then jump to an address specified by the
    /// literal B.
    /// </summary>
    /// <remarks>
    /// This address is treated as being relative to the base address
    /// of the executable memory region in which the program resides.
    /// </remarks>
    JmpNotEq,
    /// <summary>
    /// Jump If Not Equal Register - if the accumulator is not equal
    /// to the value of the register then jump to an address given by
    /// the literal.
    /// </summary>
    JneReg,
    /// <summary>
    /// Jump If Equal Register - if the accumulator is equal to the
    /// value of the register then jump to an address given by
    /// the literal.
    /// </summary>
    JeqReg,
    /// <summary>
    /// Jump If Equal To Literal - if the accumulator is equal to the
    /// literal A then jump to an address given by the literal B.
    /// </summary>
    JeqLit,
    /// <summary>
    /// Jump If Less Than Register - if the value of the register
    /// is less than the accumulator then jump to an address specified
    /// by the literal.
    /// </summary>
    JltReg,
    /// <summary>
    /// Jump If Less Than Literal - if literal A is less than the
    /// the accumulator then jump to an address specified by
    /// the literal B.
    /// </summary>
    JltLit,
    /// <summary>
    /// Jump If Greater Than Register - if the value of the register
    /// is greater than the accumulator then jump to an address given
    /// by the literal.
    /// </summary>
    JgtReg,
    /// <summary>
    /// Jump If Greater Than Literal - if literal A is greater than
    /// the accumulator then jump to an address specified by
    /// the literal B.
    /// </summary>
    JgtLit,
    /// <summary>
    /// Jump If Less Than Or Equal To Register - if the value of the
    /// register is less than or equal to the accumulator then jump
    /// to an address specified by the literal.
    /// </summary>
    JleReg,
    /// <summary>
    /// Jump If Less Than Or Equal To Literal - if literal A is less
    /// than or equal to the accumulator then jump to an address
    /// specified by the literal B.
    /// </summary>
    JleLit,
    /// <summary>
    /// Jump If Greater Than Or Equal To Register - if the value of
    /// the register is greater than or equal to the accumulator
    /// then jump to an address given by the literal.
    /// </summary>
    JgeReg,
    /// <summary>
    /// Jump If Greater Than Or Equal To Literal - if literal A is
    /// greater or equal to than the accumulator then jump to an
    /// address specified by the literal B.
    /// </summary>
    JgeLit,

    /// <summary>
    /// Push (integer) Literal to stack - push a literal value onto
    /// the stack.
    /// </summary>
    PshLit,
    /// <summary>
    /// Push (integer) Register value to stack - push the value of a
    /// register onto the stack.
    /// </summary>
    PshReg,
    /// <summary>
    /// POP (integer) to Register - pop a value from the stack
    /// into a register.
    /// </summary>
    Pop,
    /// <summary>
    /// Call Literal Address - call a subroutine by a literal address
    /// pointer.
    /// </summary>
    CalLit,
    /// <summary>
    /// Call Register Address - call a subroutine by a register pointer.
    /// </summary>
    CalReg,
    /// <summary>
    /// Return from subroutine.
    /// </summary>
    Ret,

    Pushl,
    Out,

    /// <summary>
    /// Halt - halt the execution of the virtual machine.
    /// </summary>
    Hlt = 32767,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Instruction::NOP() => String::from("nop"),
            Instruction::AddLitReg(literal, reg) => format!("add {:02X}, {}", literal, reg),
            Instruction::HLT() => String::from("hlt"),
        };
        write!(f, "{}", printable)
    }
}
