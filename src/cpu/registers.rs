use crate::utils::{into_two_complement_64, parse_two_complement_64};

use super::Cpu;

impl Cpu {
    /// Read the `register`th register
    /// The `register` 0 always has the value 0
    pub fn read_register(&self, register: usize) -> Result<u64, ()> {
        if register == 0 {
            // the first register contains 0x0
            return Ok(0);
        }
        if register > self.registers.len() {
            return Err(());
        }
        Ok(self.registers[register - 1])
    }

    // TODO I'm not sure if this function works
    pub fn read_register_signed(&self, register: usize) -> Result<i64, ()> {
        Ok(parse_two_complement_64(self.read_register(register)?))
    }

    /// Write the value `value` in the `register`th register
    /// Writing a value in the `register` 0 is not possible
    pub fn write_register(&mut self, register: usize, value: u64) -> Result<(), ()> {
        if register > self.registers.len() {
            return Err(());
        }
        if register != 0 {
            // cannot be written to the first regsiter
            self.registers[register - 1] = value;
        }
        Ok(())
    }

    // TODO I'm not sure if this function works
    pub fn write_register_signed(&mut self, register: usize, content: i64) -> Result<(), ()> {
        self.write_register(register, into_two_complement_64(content))
    }
}
