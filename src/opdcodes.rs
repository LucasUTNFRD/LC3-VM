use crate::registers::Register;
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

fn sign_extend(number: u16, bit_count: i32) -> u16 {
    let mut result = number;
    if let Some(shift_amount) = bit_count.checked_sub(1) {
        if (number >> shift_amount & 1) == 1 {
            result = number | (u16::MAX << bit_count)
        }
    }
    result
}

pub fn add(vm: &mut VM, instruction: u16) {
    // Get destination register (DR)
    let r0 = Register::from((instruction >> 9) & 0x7);

    // Get first source register (SR1)
    let r1 = Register::from((instruction >> 6) & 0x7);

    let imm_flag = (instruction >> 5) & 0x1;

    let mut value = 0;
    if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        value = vm.read_register(r1).wrapping_add(imm5);
        // vm.write_register(r0, vm.read_register(r1) + imm5);
    } else {
        let r2 = Register::from(instruction & 0x7);
        value = vm.read_register(r1).wrapping_add(vm.read_register(r2))
        // vm.write_register(r0, vm.read_register(r1) + vm.read_register(r2));
    }

    vm.write_register(r0, value);

    vm.update_flags(r0);
}
