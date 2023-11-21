use crate::{
    bus::{Bus, BusComponent, RAM_BASE},
    exception::RiscVException,
};

use super::{registers::PC_REGISTER_INDEX, Cpu};

mod instruction_type;
mod opcodes;

type Instruction = u32; // a riscv instruction has the size of 32 bits

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        let mut cpu = Cpu {
            registers: [0; 63],
            bus,
        };
        // set program counter to address where the ram starts
        cpu.registers[PC_REGISTER_INDEX - 1] = RAM_BASE; // direct accessing the registers needs PC_REGISTER_INDEX to be decreased by 1
        cpu
    }

    fn fetch_instruction(&self) -> Result<Instruction, RiscVException> {
        // read 32 bit value at the possition of the program counter from the ram
        Ok(self.bus.read(self.registers[PC_REGISTER_INDEX - 1], 32)? as Instruction)
    }

    pub fn run_next_instruction(&mut self) -> Result<(), RiscVException> {
        // fetch the next instruction
        let instruction = self.fetch_instruction()?;
        self.run_instruction(instruction)?;
        // increment program counter
        self.registers[PC_REGISTER_INDEX - 1] += 4; // direct accessing the registers needs PC_REGISTER_INDEX to be decreased by 1
        Ok(())
    }
}
