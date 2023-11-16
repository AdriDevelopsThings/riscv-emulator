use std::fmt::Display;

use crate::bus::Bus;

mod instruction;
mod registers;

pub struct Cpu {
    // The cpu has only 63 register because the first register contains 0x0
    registers: [u64; 63],
    bus: Bus
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "register states:")?;
        writeln!(f, "x0={:16X}", 0)?;
        for (i, register) in self.registers.iter().enumerate() {
            if i < 62 {
                write!(f, "x{}", i+ 1)?;
            } else {
                write!(f, "pc")?;
            }
            write!(f, "={:#16X}", *register)?;
            if i < 63 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}