use oxidation_core::instructions::enums::Instruction;
use oxidation_core::instructions::enums::OpCode;
use oxidation_core::registers::Registers;
use binary_rw::{BinaryWriter, OpenType};

pub fn assemble(instructions: &[Instruction]) {
    let mut binary_file = BinaryWriter::new("c.bin", OpenType::OpenAndCreate).expect("Failed to create binary reader.");
    for i in instructions {
        match i {
            Instruction::NOP() => write_opcode(&mut binary_file, OpCode::NOP),
            Instruction::AddLitReg(lit, reg) => write_i32_reg(&mut binary_file, OpCode::AddLitReg, lit,reg),
            Instruction::HLT() => write_opcode(&mut binary_file, OpCode::Hlt),
        };
    }
}

fn write_opcode(writer: &mut BinaryWriter, opcode: OpCode) {
    writer.write_i16(opcode as i16);
}

fn write_i32_reg(writer: &mut BinaryWriter, opcode: OpCode, param1: &i32, reg1: &Registers) {
    writer.write_i16(opcode as i16);
    writer.write_i32(*param1);
    writer.write_u8(*reg1 as u8);
}