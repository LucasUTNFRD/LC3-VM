use super::sign_extend;

use crate::hardware::memory::Memory;
use crate::hardware::register::{Register, Registers};

///An address is computed by sign-extending bits [8:0] to 16 bits and adding this
///value to the incremented PC.
///The contents of memory at this address are loaded into DR.
///The condition codes are set, based on whether the value loaded is
///negative, zero, or positive.
pub fn ld(instr: u16, reg: &mut Registers, mem: &Memory) {
    // Get destination register (DR) from bits [11:9]
    let dr: Register = Register::from((instr >> 9) & 0x7);

    // Get PCoffset9 from bits [8:0] and sign extend it to 16 bits
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    // Get current PC value
    let pc = reg.get(Register::PC);

    // Add offset to PC, read memory at that address, and store in DR
    reg.set(dr, mem.read(pc.wrapping_add(pc_offset)));

    // Update condition flags based on value loaded
    reg.update_flags(dr);
}

#[cfg(test)]
mod tests {
    use super::*;

    ///Helper function to setup a common environment for the tests
    fn setup() -> (Registers, Memory) {
        (Registers::new(), Memory::new())
    }

    #[test]
    fn test_ld_positive_offset() {
        let (mut reg, mut mem) = setup();

        // Store test value in memory at PC + offset
        let test_value = 0x1234;
        mem.write(0x3001, test_value); // PC(0x3000) + 1

        // Create LD instruction:
        // Format: 0010 (LD) 010 (DR=R2) 000000001 (offset=1)
        let instruction = 0b0010_010_000000001;

        ld(instruction, &mut reg, &mem);

        // Verify R2 contains the loaded value
        assert_eq!(reg.get(Register::R2), test_value);
        // Verify positive flag is set
        assert_eq!(reg.get(Register::Cond), 0b001);
    }

    #[test]
    fn test_ld_negative_offset() {
        let (mut reg, mut mem) = setup();

        // Set PC to a higher value to test negative offset
        reg.set(Register::PC, 0x3005);

        // Store test value in memory at PC - 1
        let test_value = 0x5678;
        mem.write(0x3004, test_value);

        // Create LD instruction with negative offset (-1):
        // Format: 0010 (LD) 011 (DR=R3) 111111111 (offset=-1)
        let instruction = 0b0010_011_111111111;

        ld(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R3), test_value);
    }

    #[test]
    fn test_ld_zero_value() {
        let (mut reg, mut mem) = setup();

        // Store zero in memory at PC + offset
        mem.write(0x3001, 0x0000);

        // Format: 0010 (LD) 100 (DR=R4) 000000001 (offset=1)
        let instruction = 0b0010_100_000000001;

        ld(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R4), 0x0000);
        // Verify zero flag is set
        assert_eq!(reg.get(Register::Cond), 0b010);
    }
}
