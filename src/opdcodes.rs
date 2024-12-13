use crate::registers::Register;
use crate::VM;

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

///
fn sign_extend(x: u16, bit_count: i32) -> u16 {
    let mut result = x;

    if (x >> (bit_count - 1)) & 1 == 1 {
        result |= u16::MAX << bit_count;
    }
    result.into()
}

pub fn add(vm: &mut VM, instruction: u16) {
    // Get destination register (DR)
    let r0 = Register::from((instruction >> 9) & 0x7);

    // Get first source register (SR1)
    let r1 = Register::from((instruction >> 6) & 0x7);

    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        vm.write_register(r0, vm.read_register(r1) + imm5);
    } else {
        let r2 = Register::from(instruction & 0x7);
        vm.write_register(r0, vm.read_register(r1) + vm.read_register(r2));
    }

    vm.update_flags(r0);
}
