use crate::{
    cpu::instruction::{
        instruction_type::{parse_funct3, RTypeInstruction, TypeInstruction},
        Instruction,
    },
    exception::RiscVException,
    Cpu,
};

/// Run an instruction with opcode=0110011
pub fn run_x33_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    let funct3 = parse_funct3(instruction);
    match funct3 {
        0x0 => run_x33_x0_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        0x1 => run_x33_x1_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        0x2 => run_x33_x2_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        0x3 => run_x33_x3_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        0x4 => run_x33_x4_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        0x5 => run_x33_x5_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        0x6 => run_x33_x6_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        0x7 => run_x33_x7_instruction(cpu, RTypeInstruction::parse_instruction(instruction)),
        _ => Err(RiscVException::IllegalInstruction),
    }
}

/// Run an instruction with opcode=0110011 and funct3=000
/// ADD / SUB rs1 (+/-) rs2
fn run_x33_x0_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    let rs1 = cpu.read_register(instruction.rs1)?;
    let rs2 = cpu.read_register(instruction.rs2)?;
    cpu.write_register(
        instruction.rd,
        match instruction.funct7 {
            0x0 => {
                // ADD
                Ok(rs1.wrapping_add(rs2))
            }
            0x20 => {
                // SUB
                Ok(rs1.wrapping_sub(rs2))
            }
            _ => Err(RiscVException::IllegalInstruction), // not reachable case
        }?,
    )
}
/// Run an instruction with opcode=0110011 and funct3=001
/// SLL shift logical left
fn run_x33_x1_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? << (cpu.read_register(instruction.rs2)? & 0x1F),
    )
}

/// Run an instruction with opcode=0110011 and funct3=010
/// SLT store less than
fn run_x33_x2_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    if (cpu.read_register(instruction.rs1)? as i64) < (cpu.read_register(instruction.rs2)? as i64) {
        cpu.write_register(instruction.rd, 1)
    } else {
        cpu.write_register(instruction.rd, 0)
    }
}

/// Run an instruction with opcode=0110011 and funct3=011
/// SLTU store less than (unsigned)
fn run_x33_x3_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    if cpu.read_register(instruction.rs1)? < cpu.read_register(instruction.rs2)? {
        cpu.write_register(instruction.rd, 1)
    } else {
        cpu.write_register(instruction.rd, 0)
    }
}

/// Run an instruction with opcode=0110011 and funct3=100
/// XOR
fn run_x33_x4_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? ^ cpu.read_register(instruction.rs2)?,
    )
}

/// Run an instruction with opcode=0110011 and funct3=101
/// SRL / SRA  shift logical right / shift arithmetic right
fn run_x33_x5_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    match instruction.funct7 {
        0x0 => {
            // SRL (logical)
            cpu.write_register(
                instruction.rd,
                cpu.read_register(instruction.rs1)? >> (cpu.read_register(instruction.rs2)? & 0x1F),
            )
        }
        0x20 => {
            // SRA (arithmetic)
            cpu.write_register(
                instruction.rd,
                ((cpu.read_register(instruction.rs1)? as i64)
                    >> (cpu.read_register(instruction.rs2)? & 0x1F)) as u64,
            )
        }
        _ => Err(RiscVException::IllegalInstruction), // not reachable case
    }
}

/// Run an instruction with opcode=0110011 and funct3=110
/// OR
fn run_x33_x6_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? | cpu.read_register(instruction.rs2)?,
    )
}

/// Run an instruction with opcode=0110011 and funct3=111
/// AND
fn run_x33_x7_instruction(
    cpu: &mut Cpu,
    instruction: RTypeInstruction,
) -> Result<(), RiscVException> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? & cpu.read_register(instruction.rs2)?,
    )
}
