use crate::{
    bus::BusComponent,
    cpu::instruction::{
        instruction_type::{ITypeInstruction, TypeInstruction},
        Instruction,
    },
    exception::RiscVException,
    utils::i12_to_u64,
    Cpu,
};

/// Run an instruction with opcode=0000011
/// LOAD instructions
pub fn run_x03_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    let instruction = ITypeInstruction::parse_instruction(instruction);
    let addr = cpu.read_register(instruction.rs1)?.wrapping_add(i12_to_u64(instruction.imm));
    let width: usize = match instruction.funct3 & 0x3 {
        0x0 => 8,  // Byte
        0x1 => 16, // Halfword
        0x2 => 32, // Word
        0x3 => 64, // Doubleword
        _ => 0,    // not reachable case
    };
    let unsigned = instruction.funct3 & 0x4 == 0x4;
    let mut loaded = cpu.bus.read(addr, width)?;
    if !unsigned && (loaded & (u64::pow(2, (width as u32) - 1))) != 0 {
        loaded |= !u64::pow(2, (width as u32) - 1);
    } else if unsigned {
        loaded &= u64::pow(2, (width as u32) - 1);
    }
    cpu.write_register(instruction.rd, loaded)
}
