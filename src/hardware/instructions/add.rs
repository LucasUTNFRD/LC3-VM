use super::sign_extend;
use crate::hardware::register::{Register, Registers};

/// - ADD takes two values and stores them in a register.
/// - In register mode, the second value to add is found in a register.
/// - In immediate mode, the second value is embedded in the right-most 5 bits of the instruction.
/// - Values which are shorter than 16 bits need to be sign extended.
/// - Any time an instruction modifies a register, the condition flags need to be updated.
/// ## Instruction Format
/// Register Mode (imm_flag = 0):
/// ```text
/// |15 14 13 12|11 10 9|8 7 6|5|4 3 2|1 0|
/// |-------------|-------|-----|---|-----|--|
/// | 0  0  0  1 |  DR   | SR1 |0 |0 0 0|SR2|
/// ```
///
/// Immediate Mode (imm_flag = 1):
/// ```text
/// |15 14 13 12|11 10 9|8 7 6|5|4   3   2   1   0|
/// |-------------|-------|-----|---|---------------|
/// | 0  0  0  1 |  DR   | SR1 |1 |    imm5       |
/// ```
pub fn add(instr: u16, reg: &mut Registers) {
    // Get destination register (DR) from bits [11:9]
    let dr = Register::from((instr >> 9) & 0x7);

    // Get first source register (SR1) from bits [8:6]
    let sr1 = Register::from((instr >> 6) & 0x7);

    // check if the instruction is in immediate mode
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        reg.set(dr, reg.get(sr1).wrapping_add(imm5));
    } else {
        let sr2: Register = Register::from(instr & 0x7);
        reg.set(dr, reg.get(sr1).wrapping_add(reg.get(sr2)));
    }
    reg.update_flags(dr);
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
    fn add_positive_value() {
        let mut reg = Registers::new();
        reg.set(Register::R3, 5); //sr1
        reg.set(Register::R5, 3); //sr2

        // Create ADD instruction in register mode:
        // Format: 0001 (ADD opcode) 010 (DR=R2) 011 (SR1=R3) 0 (register mode) 00 101 (SR2=R5)
        //         0001 010 011 0 00 101
        let instr = 0b0001_010_011_0_00_101;

        add(instr, &mut reg);

        // Result should be stored in R2 (DR)
        // R2 = R3 + R5 = 5 + 3 = 8
        assert_eq!(reg.get(Register::R2), 8);
    }

    #[test]
    fn add_negative_value() {
        let mut reg = Registers::new();
        reg.set(Register::R3, 5); //sr1
        reg.set(Register::R5, -3i16 as u16); //sr2

        // Create ADD instruction in register mode:
        // Format: 0001 (ADD opcode) 010 (DR=R2) 011 (SR1=R3) 0 (register mode) 00 101 (SR2=R5)
        //         0001 010 011 0 00 101
        let instr = 0b0001_010_011_0_00_101;

        add(instr, &mut reg);

        // Result should be stored in R2 (DR)
        // R2 = R3 + R5 = 5 + -3 = 2
        assert_eq!(reg.get(Register::R2), 2);
    }

    #[test]
    fn add_two_negative_values() {
        let mut reg = Registers::new();

        // Set initial values in registers
        reg.set(Register::R3, -5i16 as u16); // Convert -5 to two's complement
        reg.set(Register::R5, -3i16 as u16); // Convert -3 to two's complement

        // Format: 0001 (ADD) 010 (DR=R2) 011 (SR1=R3) 0 (register mode) 00 101 (SR2=R5)
        let instr = 0b0001_010_011_0_00_101;

        add(instr, &mut reg);

        // R2 = R3 + R5 = (-5) + (-3) = -8
        assert_eq!(reg.get(Register::R2), (-8i16) as u16);

        // Check that negative flag is set
        // TODO check negative flag without hardcoded value in binary
        assert_eq!(reg.get(Register::Cond), 0b100);
    }
}
