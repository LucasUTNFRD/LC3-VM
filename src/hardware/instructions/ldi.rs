use super::sign_extend;

use crate::hardware::memory::Memory;
use crate::hardware::register::{Register, Registers};

///This instruction is used to load a value from a location in memory into a register.
///1. First Memory Access:
///   - Takes the current PC (Program Counter)
///   - Adds a PCoffset9 value to it
///   - Reads from this memory location to get an address
///
///2. Second Memory Access:
///   - Uses the address obtained from the first read
///   - Reads the actual value from this address
///   - Stores this final value in the destination register
///

pub fn ldi(instr: u16, reg: &mut Registers, mem: &Memory) {
    // destination register (DR)
    let dr: Register = ((instr >> 9) & 0x7).into();

    // Get PCoffset9 from bit [8:0] and sign extend it
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    // First Memory Access
    // Add offset to PC and read from that location to get the final address
    let address_location = reg.get(Register::PC).wrapping_add(pc_offset);
    let final_address = mem.read(address_location);

    //Second Memory Access
    //Read from the final address and store it in the destination register
    let value = mem.read(final_address);

    // Store the value in the destination register
    reg.set(dr, value);

    // Update the flags
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
    fn test_ldi_basic_load() {
        let (mut reg, mut mem) = setup();

        // Setup memory:
        // At PC + 1 (0x3001), store pointer to 0x4000
        // At 0x4000, store the value 0x1234
        mem.write(0x3001, 0x4000);
        mem.write(0x4000, 0x1234);

        // Create LDI instruction:
        // Format: 1010 (LDI opcode) 000 (R0) 000000001 (offset of 1)
        let instruction = 0b1010_000_000000001;

        ldi(instruction, &mut reg, &mem);

        // Verify R0 contains the correct value
        assert_eq!(reg.get(Register::R0), 0x1234);
    }

    #[test]
    fn test_ldi_max_positive_offset() {
        let (mut reg, mut mem) = setup();

        // Maximum positive PCoffset9 is 0x0FF (255)
        mem.write(0x30FF, 0x4000);
        mem.write(0x4000, 0x1234);

        // Format: 1010 (LDI opcode) 000 (R0) 011111111 (max positive offset)
        let instruction = 0b1010_000_011111111;

        ldi(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R0), 0x1234);
    }

    #[test]
    fn test_ldi_different_register() {
        let (mut reg, mut mem) = setup();

        mem.write(0x3001, 0x4000);
        mem.write(0x4000, 0x5678);

        // Use R3 instead of R0 as destination
        let instruction = 0b1010_011_000000001;

        ldi(instruction, &mut reg, &mem);

        assert_eq!(reg.get(Register::R3), 0x5678);
    }
}
