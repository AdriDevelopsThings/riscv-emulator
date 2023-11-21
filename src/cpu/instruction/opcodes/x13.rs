use crate::{
    cpu::{
        instruction::{
            instruction_type::{
                parse_funct3, ITypeInstruction, SpecialITypeInstruction, TypeInstruction,
            },
            Instruction,
        },
        Cpu,
    },
    utils::{i12_to_u64, i12_to_u64_unsigned},
};

/// Run an instruction with opcode=0010011
pub fn run_x13_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), ()> {
    let funct3 = parse_funct3(instruction);
    match funct3 {
        0x0 => run_x13_x0_instruction(cpu, ITypeInstruction::parse_instruction(instruction)),
        0x1 => run_x13_x1_instruction(cpu, SpecialITypeInstruction::parse_instruction(instruction)),
        0x2 => run_x13_x2_instruction(cpu, ITypeInstruction::parse_instruction(instruction)),
        0x3 => run_x13_x3_instruction(cpu, ITypeInstruction::parse_instruction(instruction)),
        0x4 => run_x13_x4_instruction(cpu, ITypeInstruction::parse_instruction(instruction)),
        0x5 => run_x13_x5_instruction(cpu, SpecialITypeInstruction::parse_instruction(instruction)),
        0x6 => run_x13_x6_instruction(cpu, ITypeInstruction::parse_instruction(instruction)),
        0x7 => run_x13_x7_instruction(cpu, ITypeInstruction::parse_instruction(instruction)),
        _ => Err(()),
    }
}

/// --- Logic instructions

/// ADDI add immediate
fn run_x13_x0_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    let imm = i12_to_u64(instruction.imm);
    let sum = cpu.read_register(instruction.rs1)? + imm;
    cpu.write_register(instruction.rd, sum)
}

/// Run an instruction with opcode=0010011 and funct3=010
// SLTI set less than immediate
fn run_x13_x2_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    if (cpu.read_register(instruction.rs1)? as i64) < (i12_to_u64(instruction.imm) as i64) {
        cpu.write_register(instruction.rd, 1)
    } else {
        cpu.write_register(instruction.rd, 0)
    }
}

/// Run an instruction with opcode=0010011 and funct3=011
/// SLTIU set less than immediate unsigned
fn run_x13_x3_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    if cpu.read_register(instruction.rs1)? < i12_to_u64_unsigned(instruction.imm) {
        cpu.write_register(instruction.rd, 1)
    } else {
        cpu.write_register(instruction.rd, 0)
    }
}

/// Run an instruction with opcode=0010011 and funct3=100
/// XORI
fn run_x13_x4_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? ^ i12_to_u64(instruction.imm),
    )
}

/// Run an instruction with opcode=0010011 and funct3=110
/// ORI
fn run_x13_x6_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? | i12_to_u64(instruction.imm),
    )
}

/// Run an instruction with opcode=0010011 and funct3=111
/// ANDI
fn run_x13_x7_instruction(cpu: &mut Cpu, instruction: ITypeInstruction) -> Result<(), ()> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? & i12_to_u64(instruction.imm),
    )
}

// --- Shift instructions

/// Run an instruction with opcode=0010011 and funct3=001
/// SLLI shift left immediate
fn run_x13_x1_instruction(cpu: &mut Cpu, instruction: SpecialITypeInstruction) -> Result<(), ()> {
    cpu.write_register(
        instruction.rd,
        cpu.read_register(instruction.rs1)? << instruction.imm_1,
    )
}

/// Run an instruction with opcode=0010011 and funct3=001
/// SRLI shift right immediate OR RAI shift right arithmetic immediate
fn run_x13_x5_instruction(cpu: &mut Cpu, instruction: SpecialITypeInstruction) -> Result<(), ()> {
    match instruction.imm_2 {
        0x0 => {
            // SRLI (logic)
            cpu.write_register(
                instruction.rd,
                cpu.read_register(instruction.rs1)? >> instruction.imm_1,
            )
        }
        0x20 => {
            // SRAI (arithmetic)
            cpu.write_register(
                instruction.rd,
                ((cpu.read_register(instruction.rs1)? as i64) >> instruction.imm_1) as u64,
            )
        }
        _ => Err(()),
    }
}
