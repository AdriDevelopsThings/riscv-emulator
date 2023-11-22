use crate::{
    cpu::{
        instruction::{
            instruction_type::{parse_funct3, BTypeInstruction, TypeInstruction},
            Instruction,
        },
        registers::PC_REGISTER_INDEX,
    },
    exception::RiscVException,
    utils::i12_to_u64,
    Cpu,
};

/// Run an instruction with opcode=1100011
/// Conditional Branches
pub fn run_x63_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    let funct3 = parse_funct3(instruction);
    let instruction = BTypeInstruction::parse_instruction(instruction);
    let rs1 = cpu.read_register(instruction.rs1)?;
    let rs2 = cpu.read_register(instruction.rs2)?;
    let branch = match funct3 {
        0x0 => Ok(rs1 == rs2),                   // BEQ branch equal
        0x1 => Ok(rs1 != rs2),                   // BNE branch not equal
        0x4 => Ok((rs1 as i64) < (rs2 as i64)),  // BLT branch lower than
        0x5 => Ok((rs1 as i64) >= (rs2 as i64)), // BGE branch greater or eqal
        0x6 => Ok(rs1 < rs2),                    // BLTU branch lower than (unsigned)
        0x7 => Ok(rs1 >= rs2),                   // BGEU branch greater or equal (unsigned),
        _ => Err(RiscVException::IllegalInstruction),
    }?;
    if branch {
        let imm = i12_to_u64(instruction.get_full_immediate()).wrapping_mul(2);
        let new_pc = cpu.read_register(PC_REGISTER_INDEX)?.wrapping_add(imm);
        cpu.pc_increment = false;
        cpu.write_register(PC_REGISTER_INDEX, new_pc)?;
    }
    Ok(())
}
