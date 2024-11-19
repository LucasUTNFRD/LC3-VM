pub const PC_START: u16 = 0x3000;

/// The LC-3 has 10 total registers,
/// each of which is 16 bits. Most of them are general purpose,
/// but a few have designated roles.
/// - 8 general purpose registers (R0-R7)
/// - 1 program counter (PC) register
/// - 1 condition flags (COND) register
#[derive(Debug, Copy, Clone)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    Cond,
    Count,
}

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::PC,
            9 => Register::Cond,
            _ => panic!("Invalid register: {}", value),
        }
    }
}

///condition flags which provide information about the most recently executed calculation.
pub enum ConditionFlag {
    Positive = 1 << 0,
    Zero = 1 << 1,
    Negative = 1 << 2,
}

pub struct Registers {
    pub registers: [u16; Register::Count as usize],
}

impl Registers {
    pub fn new() -> Self {
        let mut registers = [0; Register::Count as usize];
        /* set the PC to starting position */
        registers[Register::PC as usize] = PC_START;
        /* set the condition flag to positive */
        registers[Register::Cond as usize] = ConditionFlag::Positive as u16;
        Self { registers }
    }

    // get and set methods for the Registers
    pub fn get(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    pub fn set(&mut self, r: Register, val: u16) {
        self.registers[r as usize] = val;
    }

    //update the condition flags
    pub fn update_flags(&mut self, r: Register) {
        let updated_flag = self.get(r);
        let flag;
        if updated_flag == 0 {
            flag = ConditionFlag::Zero;
        } else if (updated_flag >> 15) == 1 {
            flag = ConditionFlag::Negative;
        } else {
            flag = ConditionFlag::Positive;
        }
        self.set(Register::Cond, flag as u16);
    }

    pub fn get_flag(&self) -> u16 {
        self.registers[Register::Cond as usize]
    }
}
