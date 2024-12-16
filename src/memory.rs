const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    mem: [u16; MEMORY_MAX],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_MAX],
        }
    }

    pub fn read(&self, address: u16) -> u16 {
        let addr: usize = address.into();
        self.mem.get(addr).copied().unwrap_or(0)
    }

    pub fn write(&mut self, address: u16, value: u16) {
        let addr: usize = address.into();
        if let Some(cell) = self.mem.get_mut(addr) {
            *cell = value;
        }
    }
}
