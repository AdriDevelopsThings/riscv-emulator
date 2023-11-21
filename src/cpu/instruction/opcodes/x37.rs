use crate::{
    cpu::instruction::{
        instruction_type::{TypeInstruction, UTypeInstruction},
        Instruction,
    },
    exception::RiscVException,
    Cpu,
};

/// Run an instruction with opcode=0110111
/// LUI load upper immediate
pub fn run_x37_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    let instruction = UTypeInstruction::parse_instruction(instruction);
    cpu.write_register(instruction.rd, (instruction.imm as u64) << 12)
}
