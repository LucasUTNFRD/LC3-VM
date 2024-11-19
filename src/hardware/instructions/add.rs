use super::sign_extend;
use crate::hardware::register::{Register, Registers};

/// - ADD takes two values and stores them in a register.
/// - In register mode, the second value to add is found in a register.
/// - In immediate mode, the second value is embedded in the right-most 5 bits of the instruction.
/// - Values which are shorter than 16 bits need to be sign extended.
/// - Any time an instruction modifies a register, the condition flags need to be updated.
pub fn add(instr: u16, reg: &mut Registers) {
    // get the destination register (DR)
    let _r0 = (instr >> 9) & 0x7;
    // get the first operand (SR1)
    let _r1 = (instr >> 6) & 0x7;
    // check if the instruction is in immediate mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        reg.set(Register::R0, reg.get(Register::R1).wrapping_add(imm5));
    } else {
        let _r2 = instr & 0x7;
        reg.set(
            Register::R0,
            reg.get(Register::R1).wrapping_add(reg.get(Register::R2)),
        );
    }
    reg.update_flags(Register::R0);
}

//add test for add in add register mode and for add in immediate mode
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_inmediate_positive_value() {
        let mut reg = Registers::new();
        reg.set(Register::R1, 5);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source = R1, Immediate Flag = 1, Immediate Value = 3
        let instr = 0b0001_000_001_1_00011;
        add(instr, &mut reg);
        // R0 = R1 + 3 = 5 + 3 = 8
        assert_eq!(reg.get(Register::R0), 8);
    }

    #[test]
    fn add_inmediate_negative_value() {
        let mut reg = Registers::new();
        reg.set(Register::R1, 5);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source = R1, Immediate Flag = 1, Immediate Value = -3
        let instr = 0b0001_000_001_1_11101;
        add(instr, &mut reg);
        // R0 = R1 + -3 = 5 + -3 = 2
        assert_eq!(reg.get(Register::R0), 2);
    }

    #[test]
    fn add_register_positive_value() {
        let mut reg = Registers::new();
        reg.set(Register::R1, 5);
        reg.set(Register::R2, 3);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source = R1, Immediate Flag = 0, Source = R2
        let instr = 0b0001_000_001_0_000010;
        add(instr, &mut reg);
        // R0 = R1 + R2 = 5 + 3 = 8
        assert_eq!(reg.get(Register::R0), 8);
    }

    #[test]
    fn add_register_negative_value() {
        let mut reg = Registers::new();
        reg.set(Register::R1, 5);
        reg.set(Register::R2, -3i16 as u16);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source = R1, Immediate Flag = 0, Source = R2
        let instr = 0b0001_000_001_0_111101;
        add(instr, &mut reg);
        // R0 = R1 + R2 = 5 + -3 = 2
        assert_eq!(reg.get(Register::R0), 2);
    }
}
