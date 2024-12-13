const PC_START: u16 = 0x3000;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    PC = 8,
    Cond = 9,
    Count = 10,
}

impl From<u16> for Register {
    /// In order to avoid problems casting u16 instruction to the gven register
    fn from(r: u16) -> Self {
        match r {
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
            _ => panic!("Invalid register"),
        }
    }
}

enum RegisterFlags {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}

pub struct Registers {
    regs: [u16; 10],
}

impl Registers {
    pub fn new() -> Self {
        let mut regs = [0; Register::Count as usize];
        regs[Register::PC as usize] = PC_START;
        regs[Register::Cond as usize] = RegisterFlags::POS as u16;

        Self { regs }
    }

    pub fn get(&self, r: Register) -> u16 {
        self.regs[r as usize]
    }

    pub fn set(&mut self, r: Register, value: u16) {
        self.regs[r as usize] = value;
    }

    pub fn update_flags(&mut self, r: Register) {
        let r = self.get(r);

        if r == 0 {
            self.regs[Register::Cond as usize] = RegisterFlags::ZRO as u16;
        } else if r >> 15 == 1 {
            self.regs[Register::Cond as usize] = RegisterFlags::NEG as u16;
        } else {
            self.regs[Register::Cond as usize] = RegisterFlags::POS as u16;
        }
    }
}
