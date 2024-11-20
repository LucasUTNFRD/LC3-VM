use super::sign_extend;
use crate::hardware::{
    memory::Memory,
    register::{Register, Registers},
};

pub fn sti(inst: u16, reg: &Registers, mem: &mut Memory) {
    // Get source register (SR) from bits [11:9]
    let sr: Register = Register::from((inst >> 9) & 0x7);

    // Get PCoffset9 from bits [8:0] and sign extend it to 16 bits
    let pc_offset = sign_extend(inst & 0x1FF, 9);

    // Calculate target memory address: PC + offset
    let pc = reg.get(Register::PC);
    let initial_address = pc.wrapping_add(pc_offset);

    // mem[mem[PC + SEXT(PCoffset9)]] = SR;
    let address = mem.read(initial_address);
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
    fn test_sti_basic() {
        let (mut reg, mut mem) = setup();

        // Set value in source register (R2)
        reg.set(Register::R2, 0x1234);

        // Setup indirect addressing:
        // At PC + 1, store pointer to actual target (0x4000)
        mem.write(reg.get(Register::PC) + 1, 0x4000);

        // Create STI instruction:
        // Format: 1011 (STI) 010 (SR=R2) 000000001 (offset=1)
        let instruction = 0b1011_010_000000001;

        sti(instruction, &reg, &mut mem);

        // Verify memory at target address contains the value from R2
        assert_eq!(mem.read(0x4000), 0x1234);
    }
}
