use crate::{
    cpu::instruction::{
        instruction_type::{parse_funct3, RTypeInstruction, TypeInstruction},
        Instruction,
    },
    Cpu,
};

pub fn run_x33_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), ()> {
    let funct3 = parse_funct3(instruction);
    match funct3 {
        0x0 => run_x33_x0_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        _ => Err(()),
    }
}

// Run an instruction with opcode=0110011 and funct3=000
// ADD / SUB rs1 (+/-) rs2
fn run_x33_x0_instruction(cpu: &mut Cpu, instruction: RTypeInstruction) -> Result<(), ()> {
    let rs1 = cpu.read_register(instruction.rs1)?;
    let rs2 = cpu.read_register(instruction.rs2)?;
    cpu.write_register(
        instruction.rd,
        match instruction.funct7 {
            0x0 => {
                // ADD
                Ok(rs1 + rs2)
            }
            0x20 => {
                // SUB
                Ok(rs1 - rs2)
            }
            _ => Err(()),
        }?,
    )
}