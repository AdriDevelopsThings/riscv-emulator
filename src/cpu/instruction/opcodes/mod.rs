use crate::cpu::Cpu;

use self::{x13::run_x13_instruction, x33::run_x33_instruction};

use super::{instruction_type::parse_opcode, Instruction};

mod x13;
mod x33;

impl Cpu {
    pub fn run_instruction(&mut self, instruction: Instruction) -> Result<(), ()> {
        let opcode = parse_opcode(instruction);
        match opcode {
            0x13 => run_x13_instruction(self, instruction),
            0x33 => run_x33_instruction(self, instruction),
            _ => Err(()),
        }
    }
}
