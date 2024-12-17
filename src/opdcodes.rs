// use crate::registers::Register;
use crate::errors::VMError;
use crate::registers::RegisterFlags;
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

pub fn and(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;

    let sr1 = (instruction >> 6) & 0x7;

    let imm_flag = (instruction >> 5) & 0x1;

    let value: u16 = if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.registers.get(sr1.into())? & imm5
    } else {
        let sr2 = instruction & 0x7;
        vm.registers.get(sr1.into())? & vm.registers.get(sr2.into())?
    };

    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

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

pub fn jmp(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let base_r = (instruction >> 6) & 0x7;
    vm.registers.pc = vm.registers.get(base_r.into())?;
    Ok(())
}

pub fn jump_subroutine(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let long_flag = (instruction >> 11) & 0x1;

    // Save the current PC in R7
    vm.registers.set(7, vm.registers.pc);

    if long_flag == 0 {
        // JSRR
        let base_r = (instruction >> 6) & 0x7;
        vm.registers.pc = vm.registers.get(base_r.into())?;
    } else {
        // JSR
        let pc_offset = sign_extend(instruction & 0x7FF, 11);
        vm.registers.pc = vm.registers.pc.wrapping_add(pc_offset);
    }

    Ok(())
}

pub fn load(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let address = vm.registers.pc.wrapping_add(pc_offset);

    let value = vm.read_memory(address)?;

    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

pub fn load_register(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;
    let base_r = (instruction >> 6) & 0x7;
    let offset = sign_extend(instruction & 0x3F, 6);

    let address = vm.registers.get(base_r.into())?.wrapping_add(offset);

    let value = vm.read_memory(address)?;

    vm.registers.set(dr.into(), value);

    vm.update_flags(dr.into());

    Ok(())
}

pub fn load_effective_address(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;
    let pc_offset = sign_extend(instruction & 0x1FF, 9);

    let address = vm.registers.pc.wrapping_add(pc_offset);

    vm.registers.set(dr.into(), address);

    vm.update_flags(dr.into());

    Ok(())
}

pub fn not(vm: &mut VM, instruction: u16) -> Result<(), VMError> {
    let dr = (instruction >> 9) & 0x7;
    let sr = (instruction >> 6) & 0x7;

    let value = !vm.registers.get(sr.into())?;

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
        vm.write_memory(target_address, expected_value);

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
        vm.write_memory(target_address, expected_value);

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
        vm.write_memory(base_address, 1);
        load_register(&mut vm, 0b0110_000_001_000000)?;
        assert_eq!(vm.registers.condition, RegisterFlags::Pos);

        // Test zero value
        vm.write_memory(base_address.wrapping_add(1), 0);
        load_register(&mut vm, 0b0110_000_001_000001)?;
        assert_eq!(vm.registers.condition, RegisterFlags::Zro);

        // Test negative value
        vm.write_memory(base_address.wrapping_add(2), 0x8000);
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
}
