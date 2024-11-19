use super::sign_extend;
use crate::hardware::{
    memory::Memory,
    register::{Register, Registers},
};

/// # Store (ST) Instruction
///
/// Stores the contents of a source register into memory at PC + offset.
///
/// ## Operation
/// 1. Get source register (SR) from bits [11:9]
/// 2. Sign-extend PCoffset9 from bits [8:0] to 16 bits
/// 3. Add PCoffset9 to the current PC to get the target memory address
/// 4. Store the contents of SR into memory at the calculated address
///
/// ## Instruction Format
/// ```text
/// |15 14 13 12|11 10 9|8   7   6   5   4   3   2   1   0|
/// |-------------|-------|-----------------------------------|
/// | 0  0  1  1 |  SR   |           PCoffset9              |
/// ```
///
/// ## Examples
/// ```text
/// ST R2, #15    ; Store contents of R2 at memory[PC + 15]
/// ST R4, #-5    ; Store contents of R4 at memory[PC - 5]
/// ```
pub fn st(inst: u16, reg: &Registers, mem: &mut Memory) {
    // Get source register (SR) from bits [11:9]
    let sr: Register = Register::from((inst >> 9) & 0x7);

    // Get PCoffset9 from bits [8:0] and sign extend it to 16 bits
    let pc_offset = sign_extend(inst & 0x1FF, 9);

    // Calculate target memory address: PC + offset
    let pc = reg.get(Register::PC);
    let address = pc.wrapping_add(pc_offset);

    // Store value from source register to memory
    mem.write(address, reg.get(sr));
}

#[cfg(test)]
mod test {

    use super::*;

    /// Helper function to setup a common test environment
    fn setup() -> (Registers, Memory) {
        (Registers::new(), Memory::new())
    }

    #[test]
    fn test_st_positive_offset() {
        let (mut reg, mut mem) = setup();

        // Set value in source register (R2)
        reg.set(Register::R2, 0x1234);

        // Create ST instruction:
        // Format: 0011 (ST) 010 (SR=R2) 000000001 (offset=1)
        let instruction = 0b0011_010_000000001;

        st(instruction, &reg, &mut mem);

        // Verify memory at PC + 1 contains the value from R2
        assert_eq!(mem.read(reg.get(Register::PC) + 1), 0x1234);
    }

    #[test]
    fn test_st_negative_offset() {
        let (mut reg, mut mem) = setup();

        // Set PC to a higher value to test negative offset
        reg.set(Register::PC, 0x3005);
        reg.set(Register::R3, 0x5678);

        // Create ST instruction with negative offset (-1):
        // Format: 0011 (ST) 011 (SR=R3) 111111111 (offset=-1)
        let instruction = 0b0011_011_111111111;

        st(instruction, &reg, &mut mem);

        assert_eq!(mem.read(0x3004), 0x5678);
    }
}
