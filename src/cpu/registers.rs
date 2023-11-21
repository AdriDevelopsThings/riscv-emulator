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
}
