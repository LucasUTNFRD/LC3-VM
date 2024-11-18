pub const MEMORY_SIZE: usize = 1 << 16;

pub struct Memory {
    data: [u16; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; MEMORY_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u16 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.data[address as usize] = value;
    }
}
