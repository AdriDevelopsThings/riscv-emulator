use crate::exception::RiscVException;

use self::ram::Ram;

mod ram;

/// The ram starts at the address `RAM_BASE`
pub const RAM_BASE: u64 = 0x8000_0000;

/// You can read or write bytes from/to a `BusComponent`
pub trait BusComponent {
    /// read `size` bits (must be a multiple of 8) at the position `addr`
    fn read(&self, addr: u64, size: usize) -> Result<u64, RiscVException>;
    /// write `value` with a size of `size` bits to the position `addr`
    fn write(&mut self, addr: u64, size: usize, value: u64) -> Result<(), RiscVException>;
}

pub struct Bus {
    ram: Ram,
}

impl Bus {
    pub fn new(ram_size: usize, code: Vec<u8>) -> Self {
        Bus {
            ram: Ram::new(ram_size, code),
        }
    }

    pub fn get_ram_size(&self) -> usize {
        self.ram.size()
    }
}

impl BusComponent for Bus {
    fn read(&self, addr: u64, size: usize) -> Result<u64, RiscVException> {
        if addr >= RAM_BASE {
            return self.ram.read(addr - RAM_BASE, size);
        }
        Err(RiscVException::LoadAccessFault)
    }

    fn write(&mut self, addr: u64, size: usize, value: u64) -> Result<(), RiscVException> {
        if addr >= RAM_BASE {
            return self.ram.write(addr - RAM_BASE, size, value);
        }
        Err(RiscVException::LoadAccessFault)
    }
}
