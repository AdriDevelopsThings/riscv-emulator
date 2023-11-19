use crate::cpu::{
    instruction::{
        instruction_type::{parse_funct3, ITypeInstruction, TypeInstruction},
        Instruction,
    },
    Cpu,
};

/// Run an instruction with opcode=0010011
pub fn run_x13_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), ()> {
    let funct3 = parse_funct3(instruction);
    match funct3 {
        0x0 => run_x13_x0_instruction(cpu, ITypeInstruction::parse_instruction(instruction)),
        _ => Err(()),
    }
}

/// Run an instruction with opcode=0010011 and funct3=00000
/// ADDI add immediate
fn run_x13_x0_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    // TODO: values sign-extended
    let imm = instruction.imm as u64;
    let sum = cpu.read_register(instruction.rs1)? + imm;
    cpu.write_register(instruction.rd, sum)
}

/// Run an instruction with opcode=0010011 and funct3=00010
// SLTI set less than immediate
fn run_x13_x2_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    // TODO: values sign-extended
    if cpu.read_register(instruction.rs1)? < (instruction.imm as u64) {
        cpu.write_register(instruction.rd, 1)
    } else {
        cpu.write_register(instruction.rd, 0)
    }
}
