use crate::{
    cpu::{
        instruction::{
            instruction_type::{TypeInstruction, UTypeInstruction},
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
    let instruction = UTypeInstruction::parse_instruction(instruction);
    let imm = i20_to_u64(instruction.imm).wrapping_mul(2);
    println!("old {:x}", cpu.read_register(PC_REGISTER_INDEX)?);
    println!("{:b} {}", instruction.imm, imm);
    let new_pc = cpu.read_register(PC_REGISTER_INDEX)?.wrapping_add(imm);
    println!("new {:x}", new_pc);
    cpu.write_register(PC_REGISTER_INDEX, new_pc)?;
    cpu.write_register(instruction.rd, new_pc + 4)
}
