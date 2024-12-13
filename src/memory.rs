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
        self.mem[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.mem[address as usize] = value;
    }
}
