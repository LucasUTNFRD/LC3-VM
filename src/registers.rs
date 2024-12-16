use crate::errors::VMError;

const PC_START: u16 = 0x3000;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RegisterFlags {
    Pos = 1 << 0,
    Zro = 1 << 1,
    Neg = 1 << 2,
}

const NUM_REGISTERS: usize = 8; // R0-R7

pub struct Registers {
    regs: [u16; NUM_REGISTERS],
    pub pc: u16,
    condition: RegisterFlags,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            regs: [0; NUM_REGISTERS],
            pc: PC_START,
            condition: RegisterFlags::Zro,
        }
    }

    /// Get the value of a register without getting clippy warnings
    pub fn get(&self, register: usize) -> Result<u16, VMError> {
        self.regs
            .get(register)
            .copied()
            .ok_or(VMError::InvalidRegister)
    }

    pub fn set(&mut self, register: usize, value: u16) {
        if let Some(reg) = self.regs.get_mut(register) {
            *reg = value;
        }
    }

    /// Any time a value is written to a register, the condition flags should be updated
    pub fn update_flags(&mut self, register: usize) {
        // Acces the register value in field regs and update the flag
        if let Some(reg) = self.regs.get(register) {
            self.condition = match reg {
                0 => RegisterFlags::Zro,
                _ if reg >> 15 == 1 => RegisterFlags::Neg,
                _ => RegisterFlags::Pos,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers_init() {
        let regs = Registers::new();

        for reg in regs.regs.iter() {
            assert_eq!(*reg, 0);
        }

        // assert that condition flags are set to zero
        assert_eq!(regs.condition, RegisterFlags::Zro);

        // assert program counter is set to 0x3000
        assert_eq!(regs.pc, PC_START);
    }

    #[test]
    fn test_update_flags() {
        let mut regs = Registers::new();
        regs.set(0, 0);
        regs.update_flags(0);
        assert_eq!(regs.condition, RegisterFlags::Zro);

        regs.set(0, 1 << 15);
        regs.update_flags(0);
        assert_eq!(regs.condition, RegisterFlags::Neg);

        regs.set(0, 1);
        regs.update_flags(0);
        assert_eq!(regs.condition, RegisterFlags::Pos);
    }
}
