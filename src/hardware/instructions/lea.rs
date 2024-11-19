use crate::hardware::register::{Register, Registers};

use super::sign_extend;

pub fn lea(instr: u16, reg: &mut Registers) {
    let dr: Register = Register::from((instr >> 9) & 0x7);
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = reg.get(Register::PC);
    let addr = pc.wrapping_add(pc_offset);
    reg.set(dr, addr);
    reg.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Registers {
        Registers::new()
    }

    #[test]
    fn test_lea() {
        let mut reg = setup();

        let instruction = 0b1110_010_000000001;

        lea(instruction, &mut reg);

        assert_eq!(reg.get(Register::R2), 0x3001);
        assert_eq!(reg.get(Register::Cond), 0b001);
    }
}
