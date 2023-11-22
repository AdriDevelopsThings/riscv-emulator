use crate::{
    bus::BusComponent,
    cpu::instruction::{
        instruction_type::{STypeInstruction, TypeInstruction},
        Instruction,
    },
    exception::RiscVException,
    utils::i12_to_u64,
    Cpu,
};

pub fn run_x23_instruction(cpu: &mut Cpu, instruction: Instruction) -> Result<(), RiscVException> {
    let instruction = STypeInstruction::parse_instruction(instruction);
    let addr = cpu.read_register(instruction.rs1)? + i12_to_u64(instruction.get_full_immediate());
    let width: usize = match instruction.funct3 & 0x3 {
        0x0 => 8,  // Byte
        0x1 => 16, // Halfword
        0x2 => 32, // Word
        0x3 => 64, // Doubleword
        _ => 0,    // not reachable case
    };
    cpu.bus
        .write(addr, width, cpu.read_register(instruction.rs2)?)
}
