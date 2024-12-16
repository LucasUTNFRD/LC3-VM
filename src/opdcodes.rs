// use crate::registers::Register;
use crate::errors::VMError;
use crate::VM;

#[repr(u16)]
pub enum Opcode {
    Br = 0, /* branch */
    Add,    /* add  */
    Ld,     /* load */
    St,     /* store */
    Jsr,    /* jump register */
    And,    /* bitwise and */
    Ldr,    /* load register */
    Str,    /* store register */
    Rti,    /* unused */
    Not,    /* bitwise not */
    Ldi,    /* load indirect */
    Sti,    /* store indirect */
    Jmp,    /* jump */
    Res,    /* reserved (unused) */
    Lea,    /* load effective address */
    Trap,   /* execute trap */
}

impl From<u16> for Opcode {
    fn from(op: u16) -> Self {
        match op {
            0 => Opcode::Br,
            1 => Opcode::Add,
            2 => Opcode::Ld,
            3 => Opcode::St,
            4 => Opcode::Jsr,
            5 => Opcode::And,
            6 => Opcode::Ldr,
            7 => Opcode::Str,
            8 => Opcode::Rti,
            9 => Opcode::Not,
            10 => Opcode::Ldi,
            11 => Opcode::Sti,
            12 => Opcode::Jmp,
            13 => Opcode::Res,
            14 => Opcode::Lea,
            15 => Opcode::Trap,
            _ => Opcode::Res, // Default to reserved opcode instead of panicking
        }
    }
}

pub fn sign_extend(number: u16, bit_count: i32) -> u16 {
    let mut result = number;
    if let Some(shift_amount) = bit_count.checked_sub(1) {
        if (number >> shift_amount & 1) == 1 {
            result = number | (u16::MAX << bit_count)
        }
    }
    result
}

/// ADD takes two values and stores them in a register.
/// - In register mode, the second value to add is found in a register.
/// - In immediate mode, the second value is embedded in the right-most 5 bits of the instruction.
/// - Values which are shorter than 16 bits need to be sign extended.
/// - Any time an instruction modifies a register, the condition flags need to be updated.
pub fn add(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;

    let sr1 = (instruction >> 6) & 0x7;

    let imm_flag = (instruction >> 5) & 0x1;

    let value: u16 = if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.registers.get(sr1.into())?.wrapping_add(imm5)
    } else {
        let sr2 = instruction & 0x7;
        vm.registers
            .get(sr1.into())?
            .wrapping_add(vm.registers.get(sr2.into())?)
    };

    vm.registers.set(dr.into(), value);
    vm.update_flags(dr.into());
    Ok(())
}

pub fn ldi(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;

    // Extract and sign-extend PC offset from instruction bits [8:0]
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    // Calculate address of pointer by adding PC offset to current PC
    let pointer_addr = vm.registers.pc.wrapping_add(pc_offset);

    // Read memory at pointer_addr to get target address
    let target_addr = vm.read_memory(pointer_addr)?;

    // Read memory at target address to get final value
    let value = vm.read_memory(target_addr)?;

    // Store value in destination register
    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::VM;

    fn setup_vm() -> VM {
        VM::new()
    }

    #[test]
    fn test_add_register_mode() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Setup initial register values
        vm.write_register(1, 5); // R1 = 5
        vm.write_register(2, 3); // R2 = 3

        // Create ADD instruction: ADD R0, R1, R2
        // Format: 0001 000 001 000 010
        // 0001 = ADD opcode
        // 000 = destination register (R0)
        // 001 = first source register (R1)
        // 0 = register mode flag
        // 010 = second source register (R2)
        let instruction = 0b0001_000_001_0_00_010;

        // Execute ADD instruction
        add(&mut vm, instruction)?;

        // Verify result
        assert_eq!(vm.read_register(0)?, 8); // 5 + 3 = 8

        Ok(())
    }

    #[test]
    fn test_add_immediate_mode() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Setup initial register value
        vm.write_register(1, 5); // R1 = 5

        // Create ADD instruction: ADD R0, R1, #3
        // Format: 0001 000 001 1 00011
        // 0001 = ADD opcode
        // 000 = destination register (R0)
        // 001 = first source register (R1)
        // 1 = immediate mode flag
        // 00011 = immediate value (3)
        let instruction = 0b0001_000_001_1_00011;

        // Execute ADD instruction
        add(&mut vm, instruction)?;

        // Verify result
        assert_eq!(vm.read_register(0)?, 8); // 5 + 3 = 8
        Ok(())
    }

    #[test]
    fn test_ldi_basic() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Setup memory for indirect loading
        let initial_address = vm.registers.pc.wrapping_add(2); // PC + 2
        let final_address = 0x3100;
        let expected_value = 0x4242;

        // Store the final address at the initial address
        vm.write_memory(initial_address, final_address);

        // Store the actual value at the final address
        vm.write_memory(final_address, expected_value);

        // Create LDI instruction: LDI R0, #2
        // Format: 1010 000 000000010
        // 1010 = LDI opcode
        // 000 = destination register (R0)
        // 000000010 = PC offset of 2
        let pc_offset: u16 = 2;
        let instruction = 0b1010_000_000000010;

        // Execute LDI instruction
        ldi(&mut vm, instruction)?;

        // Verify the value was loaded correctly
        assert_eq!(vm.read_register(0)?, expected_value);

        Ok(())
    }
}
