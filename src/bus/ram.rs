use super::BusComponent;

pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize, code: Vec<u8>) -> Self {
        let mut ram: Vec<u8> = vec![0; size];
        ram.splice(..code.len(), code);
        Self { ram }
    }
}

impl BusComponent for Ram {
    fn read(&self, addr: u64, size: usize) -> Result<u64, ()> {
        if (addr as usize) + (size / 8) > self.ram.len() {
            return Err(());
        }
        let mut v: u64 = 0;
        for n in 0..(size / 8) {
            v |= ((self.ram[(addr as usize) + n]) as u64) << (n * 8);
        }
        Ok(v)
    }

    fn write(&mut self, addr: u64, size: usize, value: u64) -> Result<(), ()> {
        if (addr as usize) + (size / 8) > self.ram.len() {
            return Err(());
        }
        for n in 0..(size / 8) {
            self.ram[(addr as usize) + n] = (value >> (n * 8) & 0xFF) as u8;
        }

        Ok(())
    }
}
