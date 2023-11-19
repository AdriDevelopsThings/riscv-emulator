use crate::bus::{Bus, BusComponent, RAM_BASE};

use super::Cpu;

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
        cpu.registers[62] = RAM_BASE;
        cpu
    }

    fn fetch_instruction(&self) -> Result<Instruction, ()> {
        // read 32 bit value at the possition of the program counter from the ram
        Ok(self.bus.read(self.registers[62], 32)? as Instruction)
    }

    pub fn run_next_instruction(&mut self) -> Result<(), ()> {
        // fetch the next instruction
        let instruction = self.fetch_instruction()?;
        self.run_instruction(instruction)?;
        // increment program counter
        self.registers[62] += 4;
        Ok(())
    }
}
