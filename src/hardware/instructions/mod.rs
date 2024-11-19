pub mod add;
// pub mod and;
// pub mod br;
// pub mod jmp;
// pub mod jsr;
pub mod ld;
pub mod ldi;
pub mod ldr;
pub mod lea;
// pub mod not;
// pub mod opcode;
// pub mod st;
// pub mod sti;
// pub mod str;
// pub mod trap;
pub mod opcode;

pub fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }
    x
}
