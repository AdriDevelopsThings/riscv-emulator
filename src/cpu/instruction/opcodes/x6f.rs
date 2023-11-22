use crate::{
    cpu::{
        instruction::{
            instruction_type::{JTypeInstruction, TypeInstruction},
            Instruction,
        },
        registers::PC_REGISTER_INDEX,
    },
    exception::RiscVException,
    utils::i20_to_u64,
    Cpu,
};

/// Run an instruction with opcode=1101111
/// JAL jump and link
pub fn run_x6f_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    // instruction is a JType but ins basicly an UType
    let instruction = JTypeInstruction::parse_instruction(instruction);
    let imm = i20_to_u64(instruction.get_full_immediate()).wrapping_mul(2);
    let new_pc = cpu.read_register(PC_REGISTER_INDEX)?.wrapping_add(imm);
    cpu.pc_increment = false;
    cpu.write_register(PC_REGISTER_INDEX, new_pc)?;
    cpu.write_register(instruction.rd, new_pc + 4)
}
