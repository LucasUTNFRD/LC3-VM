const PC_START: u16 = 0x3000;

// #[derive(Debug, Clone, Copy)]
// #[repr(usize)]
// pub enum Register {
//     R0 = 0,
//     R1 = 1,
//     R2 = 2,
//     R3 = 3,
//     R4 = 4,
//     R5 = 5,
//     R6 = 6,
//     R7 = 7,
//     PC = 8,
//     Cond = 9,
//     Count = 10,
// }

// impl From<u16> for Register {
//     /// In order to avoid problems casting u16 instruction to the gven register
//     fn from(r: u16) -> Self {
//         match r {
//             0 => Register::R0,
//             1 => Register::R1,
//             2 => Register::R2,
//             3 => Register::R3,
//             4 => Register::R4,
//             5 => Register::R5,
//             6 => Register::R6,
//             7 => Register::R7,
//             8 => Register::PC,
//             9 => Register::Cond,
//             _ => Register::Count, // Default to reserved opcode instead of panicking
//         }
//     }
// }

enum RegisterFlags {
    Pos = 1 << 0,
    Zro = 1 << 1,
    Neg = 1 << 2,
}

const NUM_REGISTERS: usize = 8; // R0-R7

pub struct Registers {
    regs: [u16; NUM_REGISTERS],
    pc: u16,
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
    pub fn get(&self, register: usize) -> Option<u16> {
        self.regs.get(register).copied()
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
