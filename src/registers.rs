#[repr(usize)]
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

enum RegisterFlags {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}

pub struct Registers {
    regs: [u16; 10],
}
