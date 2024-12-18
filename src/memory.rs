use std::f32::consts::TAU;

use crate::errors::VMError;

const MEMORY_MAX: usize = 1 << 16;

pub struct Memory {
    mem: [u16; MEMORY_MAX],
}

const MR_KBSR: usize = 0xFE00; // Keyboard status register
const MR_KBDR: usize = 0xFE02; // Keyboard data register

fn check_key() -> bool {
    todo!()
}
fn getchar() -> u16 {
    todo!()
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
    /// Special handling for memory-mapped registers:
    /// - KBSR (0xFE00): Returns keyboard status (MSB set if key available)
    /// - KBDR (0xFE02): Returns ASCII code of last key pressed
    ///
    /// Returns:
    /// - Ok(value) if address is valid
    /// - Err(InvalidMemoryAccess) if address is out of bounds
    pub fn read(&mut self, address: u16) -> Result<u16, VMError> {
        let addr: usize = address.into();

        if addr == MR_KBSR {
            if check_key() {
                self.mem[MR_KBSR] = 1 << 15;
                self.mem[MR_KBDR] = getchar();
            } else {
                self.mem[MR_KBSR] = 0;
            }
        }

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
