use crate::errors::VMError;

const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    mem: [u16; MEMORY_MAX],
}

impl Memory {
    /// Creates a new Memory instance with all memory locations initialized to 0
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_MAX],
        }
    }

    /// Reads a 16-bit value from the given memory address
    ///
    /// Returns:
    /// - Ok(value) if address is valid
    /// - Err(InvalidMemoryAccess) if address is out of bounds
    pub fn read(&self, address: u16) -> Result<u16, VMError> {
        let addr: usize = address.into();
        self.mem
            .get(addr)
            .copied()
            .ok_or(VMError::InvalidMemoryAccess(address))
    }

    /// Writes a 16-bit value to the given memory address
    ///
    /// Returns:
    /// - Ok(()) if address is valid
    /// - Err(InvalidMemoryAccess) if address is out of bounds
    pub fn write(&mut self, address: u16, value: u16) -> Result<(), VMError> {
        let addr: usize = address.into();
        self.mem
            .get_mut(addr)
            .map(|cell| {
                *cell = value;
            })
            .ok_or(VMError::InvalidMemoryAccess(address))
    }
}
