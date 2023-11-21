use crate::{cpu::Cpu, exception::RiscVException};

use self::{
    x13::run_x13_instruction, x17::run_x17_instruction, x33::run_x33_instruction,
    x37::run_x37_instruction,
};

use super::{instruction_type::parse_opcode, Instruction};

mod x13;
mod x17;
mod x33;
mod x37;

impl Cpu {
    pub fn run_instruction(&mut self, instruction: Instruction) -> Result<(), RiscVException> {
        let opcode = parse_opcode(instruction);
        match opcode {
            0x13 => run_x13_instruction(self, instruction),
            0x17 => run_x17_instruction(self, instruction),
            0x33 => run_x33_instruction(self, instruction),
            0x37 => run_x37_instruction(self, instruction),
            _ => Err(RiscVException::IllegalInstruction),
        }
    }
}
