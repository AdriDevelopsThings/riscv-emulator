use crate::{
    cpu::{
        instruction::{
            instruction_type::{ITypeInstruction, TypeInstruction},
            Instruction,
        },
        registers::PC_REGISTER_INDEX,
    },
    exception::RiscVException,
    utils::i12_to_u64,
    Cpu,
};

/// Run an instruction with opcode=1100111
/// JALR jump and link register
pub fn run_x67_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    let instruction = ITypeInstruction::parse_instruction(instruction);
    let imm = i12_to_u64(instruction.imm);
    let new_pc = cpu.read_register(instruction.rs1)?.wrapping_add(imm) & 0xFFFFFFFFFFFFFFFE;
    cpu.pc_increment = false;
    cpu.write_register(instruction.rd, cpu.read_register(PC_REGISTER_INDEX)? + 4)?;
    cpu.write_register(PC_REGISTER_INDEX, new_pc)
}
