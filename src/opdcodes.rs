use std::io::{Read, Write};
// use std::u8;

// use crate::registers::Register;
use crate::errors::{TrapError, VMError};
use crate::registers::RegisterFlags;
use crate::{VMState, VM};

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

// TODO: improve error handling
pub fn trap(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    vm.write_register(7, vm.registers.pc);

    // print the dbg of the instruction in binary and in hex
    dbg!(format!("{:016b}", instruction));
    dbg!(format!("{:04X}", instruction));
    let trap_vector = instruction & 0xFF;

    dbg!("Trap vector: {:#04X}", trap_vector);

    match trap_vector {
        0x20 => {
            // GETC - Read a single character from the keyboard, The character is not echoed onto the console.
            // Its ASCII code is copied into register 0. The high 8 bits of R0 are cleared.
            let mut buffer = [0; 1];
            std::io::stdin()
                .read_exact(&mut buffer)
                .map_err(|err| VMError::TrapError(TrapError::IOError(err.to_string())))?;

            if let Some(c) = buffer.first() {
                vm.registers.set(0, (*c).into());
                vm.update_flags(0);
            }
            Ok(())
        }
        0x21 => {
            // OUT - Write a character in R0[7:0] to the console display

            // The high 8 bits of R0 are ignored with the mask 0xFF.
            let char_code =
                u8::try_from(vm.read_register(0)? & 0xFF).map_err(|_| VMError::InvalidCharacter)?;

            print!("{}", char::from(char_code));

            std::io::stdout()
                .flush()
                .map_err(|err| VMError::TrapError(TrapError::IOError(err.to_string())))?;

            Ok(())
        }
        0x22 => {
            // PUTS - Write a string of ASCII characters to the console display.

            let mut address = vm.read_register(0)?;

            let mut value = vm.read_memory(address)?;

            while value != 0 {
                let char_code =
                    u8::try_from(value & 0xFF).map_err(|_| VMError::InvalidCharacter)?;

                print!("{}", char::from(char_code));

                address = address.wrapping_add(1);
                value = vm.read_memory(address)?;
            }

            std::io::stdout()
                .flush()
                .map_err(|err| VMError::TrapError(TrapError::IOError(err.to_string())))?;

            Ok(())
        }
        0x23 => {
            // IN - Input a character with echo
            print!("Enter a character: ");

            std::io::stdout()
                .flush()
                .map_err(|err| VMError::TrapError(TrapError::IOError(err.to_string())))?;

            let mut buffer = [0; 1];
            std::io::stdin()
                .read_exact(&mut buffer)
                .map_err(|err| VMError::TrapError(TrapError::IOError(err.to_string())))?;

            if let Some(c) = buffer.first() {
                println!("{}", char::from(*c));
                vm.registers.set(0, (*c).into());
                vm.update_flags(0);
            }
            Ok(())
        }
        0x24 => {
            // PUTSP - Write a string of ASCII characters to the console display.
            let mut address = vm.read_register(0)?;

            let mut value = vm.read_memory(address)?;

            while value != 0 {
                let char1 = u8::try_from(value & 0xFF).map_err(|_| VMError::InvalidCharacter)?;
                print!("{}", char::from(char1));

                let char2 = u8::try_from(value >> 8).map_err(|_| VMError::InvalidCharacter)?;
                if char2 != 0 {
                    print!("{}", char::from(char2));
                }

                address = address.wrapping_add(1);
                value = vm.read_memory(address)?;
            }

            std::io::stdout()
                .flush()
                .map_err(|err| VMError::TrapError(TrapError::IOError(err.to_string())))?;

            Ok(())
        }
        0x25 => {
            // HALT - Halt execution
            println!("HALT");
            vm.state = VMState::Halted;
            Ok(())
        }
        _ => std::process::exit(2),
    }
}

/// Sign extends a number to 16 bits based on its most significant bit
///
/// Takes a number and the count of its significant bits, then extends
/// the sign bit (MSB) across the remaining high bits of a 16-bit value
///
/// # Arguments
/// * `number` - The number to sign extend
/// * `bit_count` - The number of significant bits in the original number
///
/// # Returns
/// The sign-extended 16-bit value
fn sign_extend(number: u16, bit_count: i32) -> u16 {
    let mut result = number;
    if let Some(shift_amount) = bit_count.checked_sub(1) {
        if (number >> shift_amount & 1) == 1 {
            result = number | (u16::MAX << bit_count)
        }
    }
    result
}

/// ADD - Add
///
/// Format: `ADD DR, SR1, SR2` or `ADD DR, SR1, imm5`
///
/// Adds two values and stores the result in a register:
/// - If bit [5] is 0, adds the contents of SR1 and SR2
/// - If bit [5] is 1, adds the contents of SR1 and sign-extended imm5
///
/// Updates condition codes based on the result
pub fn add(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;

    let sr1 = (instruction >> 6) & 0x7;

    let imm_flag = (instruction >> 5) & 0x1;

    let value: u16 = if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.read_register(sr1.into())?.wrapping_add(imm5)
    } else {
        let sr2 = instruction & 0x7;
        vm.registers
            .get(sr1.into())?
            .wrapping_add(vm.read_register(sr2.into())?)
    };

    vm.registers.set(dr.into(), value);
    vm.update_flags(dr.into());
    Ok(())
}

/// LDI - Load Indirect
///
/// Format: `LDI DR, PCoffset9`
///
/// Loads a value from memory using indirect addressing:
/// 1. Adds PCoffset9 to the incremented PC to get the address of a pointer
/// 2. Loads the memory contents at this pointer address
/// 3. Loads the value at the address from step 2 into DR
///
/// Updates condition codes based on the value loaded
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

/// AND - Bitwise AND
///
/// Format: `AND DR, SR1, SR2` or `AND DR, SR1, imm5`
///
/// Performs bitwise AND operation:
/// - If bit [5] is 0, ANDs the contents of SR1 and SR2
/// - If bit [5] is 1, ANDs the contents of SR1 and sign-extended imm5
///
/// Updates condition codes based on the result
pub fn and(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;

    let sr1 = (instruction >> 6) & 0x7;

    let imm_flag = (instruction >> 5) & 0x1;

    let value: u16 = if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.read_register(sr1.into())? & imm5
    } else {
        let sr2 = instruction & 0x7;
        vm.read_register(sr1.into())? & vm.read_register(sr2.into())?
    };

    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

/// BR - Conditional Branch
///
/// Format: `BRnzp PCoffset9`
///
/// Branches to PC + PCoffset9 if any condition code bit that is set in
/// the instruction (n, z, or p) matches the current condition flags
pub fn conditional_branch(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let n = ((instruction >> 11) & 0x1) != 0;
    let z = ((instruction >> 10) & 0x1) != 0;
    let p = ((instruction >> 9) & 0x1) != 0;

    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let condition = vm.registers.condition;

    if (n && condition == RegisterFlags::Neg)
        || (z && condition == RegisterFlags::Zro)
        || (p && condition == RegisterFlags::Pos)
    {
        vm.registers.pc = vm.registers.pc.wrapping_add(pc_offset);
    }

    Ok(())
}

/// JMP - Jump
///
/// Format: `JMP BaseR`
///
/// Jumps to the address contained in the base register
/// Also used for RET when BaseR is R7
pub fn jmp(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let base_r = (instruction >> 6) & 0x7;
    vm.registers.pc = vm.read_register(base_r.into())?;
    Ok(())
}

/// JSR/JSRR - Jump to Subroutine
///
/// Format: `JSR PCoffset11` or `JSRR BaseR`
///
/// Saves PC to R7 then:
/// - If bit [11] is 1 (JSR): PC = PC + PCoffset11
/// - If bit [11] is 0 (JSRR): PC = BaseR
pub fn jump_subroutine(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let long_flag = (instruction >> 11) & 0x1;

    // Save the current PC in R7
    vm.registers.set(7, vm.registers.pc);

    if long_flag == 0 {
        // JSRR
        let base_r = (instruction >> 6) & 0x7;
        vm.registers.pc = vm.read_register(base_r.into())?;
    } else {
        // JSR
        let pc_offset = sign_extend(instruction & 0x7FF, 11);
        vm.registers.pc = vm.registers.pc.wrapping_add(pc_offset);
    }

    Ok(())
}

/// LD - Load
///
/// Format: `LD DR, PCoffset9`
///
/// Loads a value from memory at address PC + PCoffset9 into DR
/// Updates condition codes based on the value loaded
pub fn load(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let address = vm.registers.pc.wrapping_add(pc_offset);

    let value = vm.read_memory(address)?;

    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

/// LDR - Load Register
///
/// Format: `LDR DR, BaseR, offset6`
///
/// Loads a value from memory at address BaseR + offset6 into DR
/// Updates condition codes based on the value loaded
pub fn load_register(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;
    let base_r = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);

    let address = vm.read_register(base_r.into())?.wrapping_add(offset);

    let value = vm.read_memory(address)?;

    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

/// LEA - Load Effective Address
///
/// Format: `LEA DR, PCoffset9`
///
/// Loads the address PC + PCoffset9 into DR
/// Updates condition codes based on the value loaded
pub fn load_effective_address(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let address = vm.registers.pc.wrapping_add(pc_offset);

    vm.registers.set(dr.into(), address);

    vm.update_flags(dr.into());

    Ok(())
}

/// NOT - Bitwise NOT
///
/// Format: `NOT DR, SR`
///
/// Performs bitwise NOT operation on the contents of SR and stores in DR
/// Updates condition codes based on the result
pub fn not(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;
    let sr = (instruction >> 6) & 0x7;

    let value = !vm.read_register(sr.into())?;

    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

pub fn store(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let sr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let address = vm.registers.pc.wrapping_add(pc_offset);

    let value = vm.read_register(sr.into())?;

    vm.write_memory(address, value)?;

    Ok(())
}

pub fn store_indirect(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let sr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let address = vm.registers.pc.wrapping_add(pc_offset);

    let target_address = vm.read_memory(address)?;

    let value = vm.read_register(sr.into())?;

    vm.write_memory(target_address, value)?;

    Ok(())
}

pub fn store_register(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let sr = (instruction >> 9) & 0x7;
    let base_r = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);

    let address = vm.read_register(base_r.into())?.wrapping_add(offset);

    let value = vm.read_register(sr.into())?;

    vm.write_memory(address, value)?;

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unusual_byte_groupings)]
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
        vm.write_memory(initial_address, final_address)?;

        // Store the actual value at the final address
        vm.write_memory(final_address, expected_value)?;

        // Create LDI instruction: LDI R0, #2
        // Format: 1010 000 000000010
        // 1010 = LDI opcode
        // 000 = destination register (R0)
        // 000000010 = PC offset of 2
        let instruction = 0b1010_000_000000010;

        // Execute LDI instruction
        ldi(&mut vm, instruction)?;

        // Verify the value was loaded correctly
        assert_eq!(vm.read_register(0)?, expected_value);

        Ok(())
    }

    #[test]
    fn test_and_register_mode() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Setup initial register values
        vm.write_register(1, 0b1100); // R1 = 12 (1100 in binary)
        vm.write_register(2, 0b1010); // R2 = 10 (1010 in binary)

        // Create AND instruction: AND R0, R1, R2
        // Format: 0101 000 001 000 010
        // 0101 = AND opcode
        // 000 = destination register (R0)
        // 001 = first source register (R1)
        // 0 = register mode flag
        // 010 = second source register (R2)
        let instruction = 0b0101_000_001_0_00_010;

        // Execute AND instruction
        and(&mut vm, instruction)?;

        // Verify result (1100 & 1010 = 1000 = 8)
        assert_eq!(vm.read_register(0)?, 0b1000);

        Ok(())
    }

    #[test]
    fn test_and_immediate_mode() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Setup initial register value
        vm.write_register(1, 0b1111); // R1 = 15 (1111 in binary)

        // Create AND instruction: AND R0, R1, #3
        // Format: 0101 000 001 1 00011
        // 0101 = AND opcode
        // 000 = destination register (R0)
        // 001 = first source register (R1)
        // 1 = immediate mode flag
        // 00011 = immediate value (3)
        let instruction = 0b0101_000_001_1_00011;

        // Execute AND instruction
        and(&mut vm, instruction)?;

        // Verify result (1111 & 0011 = 0011 = 3)
        assert_eq!(vm.read_register(0)?, 0b0011);

        Ok(())
    }

    #[test]
    fn test_br_positive_flag() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set positive flag by writing a positive value to R0
        vm.write_register(0, 1);
        vm.update_flags(0);

        // Create BR instruction: BRp #2
        // Format: 0000 001 000000010
        // 0000 = BR opcode
        // 001 = only p flag set (n=0, z=0, p=1)
        // 000000010 = offset of 2
        let instruction = 0b0000_001_000000010;

        let initial_pc = vm.registers.pc;

        conditional_branch(&mut vm, instruction)?;

        // PC should be incremented by 2
        assert_eq!(vm.registers.pc, initial_pc + 2);

        Ok(())
    }

    #[test]
    fn test_br_negative_flag() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set negative flag by writing a negative value to R0
        vm.write_register(0, 0x8000); // Most significant bit set
        vm.update_flags(0);

        // Create BR instruction: BRn #-2
        // Format: 0000 100 111111110
        // 0000 = BR opcode
        // 100 = only n flag set (n=1, z=0, p=0)
        // 111111110 = offset of -2 in 9-bit two's complement
        let instruction = 0b0000_100_111111110;

        let initial_pc = vm.registers.pc;

        conditional_branch(&mut vm, instruction)?;

        // PC should be decremented by 2
        assert_eq!(vm.registers.pc, initial_pc - 2);

        Ok(())
    }

    #[test]
    fn test_br_zero_flag() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set zero flag by writing zero to R0
        vm.write_register(0, 0);
        vm.update_flags(0);

        // Create BR instruction: BRz #1
        // Format: 0000 010 000000001
        // 0000 = BR opcode
        // 010 = only z flag set (n=0, z=1, p=0)
        // 000000001 = offset of 1
        let instruction = 0b0000_010_000000001;

        let initial_pc = vm.registers.pc;

        conditional_branch(&mut vm, instruction)?;

        // PC should be incremented by 1
        assert_eq!(vm.registers.pc, initial_pc + 1);

        Ok(())
    }

    #[test]
    fn test_br_multiple_flags() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set zero flag
        vm.write_register(0, 0);
        vm.update_flags(0);

        // Create BR instruction: BRnzp #2 (should branch because all flags are checked)
        // Format: 0000 111 000000010
        // 0000 = BR opcode
        // 111 = all flags set (n=1, z=1, p=1)
        // 000000010 = offset of 2
        let instruction = 0b0000_111_000000010;

        let initial_pc = vm.registers.pc;

        conditional_branch(&mut vm, instruction)?;

        // PC should be incremented by 2
        assert_eq!(vm.registers.pc, initial_pc + 2);

        Ok(())
    }

    #[test]
    fn test_jmp_basic() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up target address in R1
        let target_address = 0x3100;
        vm.write_register(1, target_address);

        // Create JMP instruction: JMP R1
        // Format: 1100 000 001 000000
        // 1100 = JMP opcode
        // 000 = unused
        // 001 = base register (R1)
        // 000000 = unused
        let instruction = 0b1100_000_001_000000;

        jmp(&mut vm, instruction)?;

        // Verify PC was updated to target address
        assert_eq!(vm.registers.pc, target_address);

        Ok(())
    }

    #[test]
    fn test_jmp_ret() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up return address in R7
        let return_address = 0x3200;
        vm.write_register(7, return_address);

        // Create RET instruction (JMP R7)
        // Format: 1100 000 111 000000
        let instruction = 0b1100_000_111_000000;

        jmp(&mut vm, instruction)?;

        // Verify PC was updated to return address
        assert_eq!(vm.registers.pc, return_address);

        Ok(())
    }

    #[test]
    fn test_jsr_long() -> Result<(), VMError> {
        let mut vm = setup_vm();
        let initial_pc = vm.registers.pc;

        // Create JSR instruction with positive offset
        // Format: 0100 1 00000000101
        // 0100 = JSR opcode
        // 1 = long flag (JSR)
        // 00000000101 = offset of 5
        let instruction = 0b0100_1_00000000101;

        jump_subroutine(&mut vm, instruction)?;

        // Verify R7 contains original PC
        assert_eq!(vm.read_register(7)?, initial_pc);

        // Verify PC was updated correctly
        assert_eq!(vm.registers.pc, initial_pc + 5);

        Ok(())
    }

    #[test]
    fn test_jsrr() -> Result<(), VMError> {
        let mut vm = setup_vm();
        let initial_pc = vm.registers.pc;

        // Set up target address in R1
        let target_address = 0x3100;
        vm.write_register(1, target_address);

        // Create JSRR instruction
        // Format: 0100 0 00 001 000000
        // 0100 = JSR opcode
        // 0 = register mode flag (JSRR)
        // 00 = unused
        // 001 = base register (R1)
        // 000000 = unused
        let instruction = 0b0100_0_00_001_000000;

        jump_subroutine(&mut vm, instruction)?;

        // Verify R7 contains original PC
        assert_eq!(vm.read_register(7)?, initial_pc);

        // Verify PC was updated to target address
        assert_eq!(vm.registers.pc, target_address);

        Ok(())
    }

    #[test]
    fn test_load() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up test value in memory
        let expected_value = 0x4242;
        let pc_offset = 2;
        let target_address = vm.registers.pc.wrapping_add(pc_offset);
        vm.write_memory(target_address, expected_value)?;

        // Create LD instruction: LD R0, #2
        // Format: 0010 000 000000010
        // 0010 = LD opcode
        // 000 = destination register (R0)
        // 000000010 = PC offset of 2
        let instruction = 0b0010_000_000000010;

        load(&mut vm, instruction)?;

        // Verify value was loaded into R0
        assert_eq!(vm.read_register(0)?, expected_value);

        // Verify condition flags were updated
        assert_eq!(vm.registers.condition, RegisterFlags::Pos);

        Ok(())
    }

    #[test]
    fn test_load_register() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up base register (R1) with base address
        let base_address = 0x3000;
        vm.write_register(1, base_address);

        // Set up test value in memory at base_address + offset
        let offset = 2;
        let expected_value = 0x4240;
        let target_address = base_address.wrapping_add(offset);
        vm.write_memory(target_address, expected_value)?;

        // Create LDR instruction: LDR R0, R1, #2
        // Format: 0110 000 001 000010
        // 0110 = LDR opcode
        // 000 = destination register (R0)
        // 001 = base register (R1)
        // 000010 = offset of 2
        let instruction = 0b0110_000_001_000010;

        load_register(&mut vm, instruction)?;

        // Verify value was loaded into R0
        assert_eq!(vm.read_register(0)?, 0x4240);

        Ok(())
    }

    #[test]
    fn test_load_register_updates_flags() -> Result<(), VMError> {
        let mut vm = setup_vm();
        let base_address = 0x3000;
        vm.write_register(1, base_address);

        // Test positive value
        vm.write_memory(base_address, 1)?;
        load_register(&mut vm, 0b0110_000_001_000000)?;
        assert_eq!(vm.registers.condition, RegisterFlags::Pos);

        // Test zero value
        vm.write_memory(base_address.wrapping_add(1), 0)?;
        load_register(&mut vm, 0b0110_000_001_000001)?;
        assert_eq!(vm.registers.condition, RegisterFlags::Zro);

        // Test negative value
        vm.write_memory(base_address.wrapping_add(2), 0x8000)?;
        load_register(&mut vm, 0b0110_000_001_000010)?;
        assert_eq!(vm.registers.condition, RegisterFlags::Neg);

        Ok(())
    }

    #[test]
    fn test_load_effective_address_basic() -> Result<(), VMError> {
        let mut vm = setup_vm();
        let initial_pc = vm.registers.pc;
        let offset = 5;

        // Create LEA instruction: LEA R0, #5
        // Format: 1110 000 000000101
        // 1110 = LEA opcode
        // 000 = destination register (R0)
        // 000000101 = offset of 5
        let instruction = 0b1110_000_000000101;

        load_effective_address(&mut vm, instruction)?;

        // Verify the calculated address was stored in R0
        assert_eq!(vm.read_register(0)?, initial_pc.wrapping_add(offset));

        Ok(())
    }

    #[test]
    fn test_not() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up test value in R1
        let initial_value = 0b1010;
        vm.write_register(1, initial_value);

        // Create NOT instruction: NOT R0, R1
        // Format: 1001 000 001 111111
        // 1001 = NOT opcode
        // 000 = destination register (R0)
        // 001 = source register (R1)
        // 111111 = unused
        let instruction = 0b1001_000_001_111111;

        not(&mut vm, instruction)?;

        // Verify the bitwise NOT was stored in R0
        assert_eq!(vm.read_register(0)?, !initial_value);

        Ok(())
    }

    #[test]
    fn test_store() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up value in source register (R1)
        let value_to_store = 0x4242;
        vm.write_register(1, value_to_store);

        // Calculate target address (PC + offset)
        let pc_offset = 2;
        let target_address = vm.registers.pc.wrapping_add(pc_offset);

        // Create ST instruction: ST R1, #2
        // Format: 0011 001 000000010
        // 0011 = ST opcode
        // 001 = source register (R1)
        // 000000010 = PC offset of 2
        let instruction = 0b0011_001_000000010;

        store(&mut vm, instruction)?;

        // Verify value was stored in memory at target address
        assert_eq!(vm.read_memory(target_address)?, value_to_store);

        Ok(())
    }

    #[test]
    fn test_store_indirect() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up value in source register (R1)
        let value_to_store = 0x4242;
        vm.write_register(1, value_to_store);

        // Set up pointer in memory
        let pointer_offset = 2;
        let pointer_addr = vm.registers.pc.wrapping_add(pointer_offset);
        let final_addr = 0x3100;
        vm.write_memory(pointer_addr, final_addr)?;

        // Create STI instruction: STI R1, #2
        // Format: 1011 001 000000010
        // 1011 = STI opcode
        // 001 = source register (R1)
        // 000000010 = PC offset of 2
        let instruction = 0b1011_001_000000010;

        store_indirect(&mut vm, instruction)?;

        // Verify value was stored in memory at final address
        assert_eq!(vm.read_memory(final_addr)?, value_to_store);

        Ok(())
    }

    #[test]
    fn test_store_register() -> Result<(), VMError> {
        let mut vm = setup_vm();

        // Set up base register (R1) with base address
        let base_address = 0x3000;
        vm.write_register(1, base_address);

        // Set up value in source register (R2)
        let value_to_store = 0x4242;
        vm.write_register(2, value_to_store);

        // Create STR instruction: STR R2, R1, #2
        // Format: 0111 010 001 000010
        // 0111 = STR opcode
        // 010 = source register (R2)
        // 001 = base register (R1)
        // 000010 = offset of 2
        let instruction = 0b0111_010_001_000010;

        store_register(&mut vm, instruction)?;

        // Calculate target address and verify value was stored
        let target_address = base_address.wrapping_add(2);
        assert_eq!(vm.read_memory(target_address)?, value_to_store);

        Ok(())
    }
}
