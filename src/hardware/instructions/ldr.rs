use crate::hardware::{
    memory::Memory,
    register::{Register, Registers},
};

use super::sign_extend;

///An address is computed by sign-extending bits [5:0] to 16 bits and adding this
///value to the contents of the register specified by bits [8:6]. The contents of memory
///at this address are loaded into DR. The condition codes are set, based on whether
///the value loaded is negative, zero, or positive.
///LDR DR, BaseR, offset6
pub fn ldr(instr: u16, reg: &mut Registers, mem: &Memory) {
    // Get destination register (DR) from bits [11:9]
    let dr = Register::from((instr >> 9) & 0x7);
    // Get base register (BaseR) from bits [8:6]
    let base_r = Register::from((instr >> 6) & 0x7);
    // Get and sign-extend 6-bit offset from bits [5:0]
    let offset = sign_extend(instr & 0x3F, 6);
    // Calculate memory address by adding offset to base register value
    let address = reg.get(base_r).wrapping_add(offset);
    // Load value from memory at calculated address into destination register
    reg.set(dr, mem.read(address));
    // Update condition flags based on loaded value
    reg.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use crate::hardware::register::{self, PC_START};

    use super::*;

    ///Helper function to setup a common environment for the tests
    fn setup() -> (Registers, Memory) {
        (Registers::new(), Memory::new())
    }

    #[test]
    fn test_ldr() {
        let (mut reg, mut mem) = setup();

        // Set base register (R2) value
        reg.set(Register::R2, PC_START);

        // Store test value at base + offset
        let test_value = 0x1234;
        mem.write(PC_START + 2, test_value); // 0x3000 + 2

        // Create LDR instruction:
        // Format: 0110 (LDR) 011 (DR=R3) 010 (BaseR=R2) 000010 (offset=2)
        let instruction = 0b0110_011_010_000010;

        ldr(instruction, &mut reg, &mem);

        // Verify R3 contains the loaded value
        assert_eq!(reg.get(Register::R3), test_value);
        // Verify positive flag is set
        assert_eq!(reg.get(Register::Cond), 0b001);
    }

    #[test]
    fn test_ldr_negative_offset() {
        let (mut reg, mut mem) = setup();

        // Set base register (R1) to a higher address
        reg.set(Register::R1, 0x3010);

        // Store test value at base - 2
        let test_value = 0x5678;
        mem.write(0x300E, test_value); // 0x3010 - 2

        // Format: 0110 (LDR) 100 (DR=R4) 001 (BaseR=R1) 111110 (offset=-2)
        let instruction = 0b0110_100_001_111110;

        ldr(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R4), test_value);
    }

    #[test]
    fn test_ldr_zero_value() {
        let (mut reg, mut mem) = setup();

        reg.set(Register::R3, 0x3000);
        mem.write(0x3001, 0x0000);

        // Format: 0110 (LDR) 101 (DR=R5) 011 (BaseR=R3) 000001 (offset=1)
        let instruction = 0b0110_101_011_000001;

        ldr(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R5), 0x0000);
        // Verify zero flag is set
        assert_eq!(reg.get(Register::Cond), 0b010);
    }

    #[test]
    fn test_ldr_negative_value() {
        let (mut reg, mut mem) = setup();

        reg.set(Register::R4, 0x3000);
        // Store negative value
        let negative_value = (-42i16) as u16;
        mem.write(0x3001, negative_value);

        // Format: 0110 (LDR) 110 (DR=R6) 100 (BaseR=R4) 000001 (offset=1)
        let instruction = 0b0110_110_100_000001;

        ldr(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R6), negative_value);
        // Verify negative flag is set
        assert_eq!(reg.get(Register::Cond), 0b100);
    }

    #[test]
    fn test_ldr_max_positive_offset() {
        let (mut reg, mut mem) = setup();

        reg.set(Register::R5, 0x3000);
        // Maximum positive offset is 0x1F (6 bits)
        let test_value = 0xABCD;
        mem.write(0x301F, test_value); // 0x3000 + 0x1F

        // Format: 0110 (LDR) 111 (DR=R7) 101 (BaseR=R5) 011111 (offset=0x1F)
        let instruction = 0b0110_111_101_011111;

        ldr(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R7), test_value);
    }

    #[test]
    fn test_ldr_min_negative_offset() {
        let (mut reg, mut mem) = setup();

        reg.set(Register::R6, 0x3040);
        // Minimum negative offset is -32 (6 bits)
        let test_value = 0xDEAD;
        mem.write(0x3020, test_value); // 0x3040 - 32

        // Format: 0110 (LDR) 001 (DR=R1) 110 (BaseR=R6) 100000 (offset=-32)
        let instruction = 0b0110_001_110_100000;

        ldr(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R1), test_value);
    }

    #[test]
    fn test_ldr_wrapping() {
        let (mut reg, mut mem) = setup();

        // Set base register near maximum memory address
        reg.set(Register::R7, 0xFFFF);

        // Store test value at wrapped address
        let test_value = 0xBEEF;
        mem.write(0x0001, test_value); // 0xFFFF + 2 wraps to 0x0001

        // Format: 0110 (LDR) 010 (DR=R2) 111 (BaseR=R7) 000010 (offset=2)
        let instruction = 0b0110_010_111_000010;

        ldr(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R2), test_value);
    }

    #[test]
    fn test_ldr_same_register() {
        let (mut reg, mut mem) = setup();

        // Test using same register as base and destination
        reg.set(Register::R3, 0x3000);
        let test_value = 0x1234;
        mem.write(0x3002, test_value);

        // Format: 0110 (LDR) 011 (DR=R3) 011 (BaseR=R3) 000010 (offset=2)
        let instruction = 0b0110_011_011_000010;

        ldr(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R3), test_value);
    }
}
