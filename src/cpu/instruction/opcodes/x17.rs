use crate::{
    cpu::{
        instruction::{
            instruction_type::{TypeInstruction, UTypeInstruction},
            Instruction,
        },
        registers::PC_REGISTER_INDEX,
    },
    exception::RiscVException,
    Cpu,
};

/// Run an instruction with opcode=0010111
/// AUIPC add upper immediate to pc
pub fn run_x17_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    let instruction = UTypeInstruction::parse_instruction(instruction);
    cpu.write_register(
        instruction.rd,
        cpu.read_register(PC_REGISTER_INDEX)? + ((instruction.imm as u64) << 12),
    )
}
