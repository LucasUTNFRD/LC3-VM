use crate::hardware::memory::Memory;
use crate::hardware::registers::{Register, Registers};

// define the opcodes
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    OpBr,   // branch
    OpAdd,  // add
    OpLd,   // load
    OpSt,   // store
    OpJsr,  // jump register
    OpAnd,  // bitwise and
    OpLdr,  // load register
    OpStr,  // store register
    OpRti,  // unused
    OpNot,  // bitwise not
    OpLdi,  // load indirect
    OpSti,  // store indirect
    OpJmp,  // jump
    OpRes,  // reserved (unused)
    OpLea,  // load effective address
    OpTrap, // execute trap
}

impl From<u16> for OpCode {
    fn from(value: u16) -> Self {
        match value {
            0 => OpCode::OpBr,
            1 => OpCode::OpAdd,
            2 => OpCode::OpLd,
            3 => OpCode::OpSt,
            4 => OpCode::OpJsr,
            5 => OpCode::OpAnd,
            6 => OpCode::OpLdr,
            7 => OpCode::OpStr,
            8 => OpCode::OpRti,
            9 => OpCode::OpNot,
            10 => OpCode::OpLdi,
            11 => OpCode::OpSti,
            12 => OpCode::OpJmp,
            13 => OpCode::OpRes,
            14 => OpCode::OpLea,
            15 => OpCode::OpTrap,
            _ => panic!("Invalid opcode: {}", value),
        }
    }
}

pub struct VM {
    mem: Memory,
    reg: Registers,
}

fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1 {
        x |= 0xFFFF << bit_count;
    }
    x
}

impl VM {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            reg: Registers::new(),
        }
    }

    // method that loops through the program and executes it
    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let pc = self.reg.get(Register::PC);
            self.reg.set(Register::PC, pc.wrapping_add(1));
            let instr = self.mem.read(pc);
            let op: OpCode = (instr >> 12).into();
            match op {
                OpCode::OpAdd => {
                    // get the destination register (DR)
                    let _r0 = (instr >> 9) & 0x7;
                    // get the first operand (SR1)
                    let _r1 = (instr >> 6) & 0x7;
                    // check if the instruction is in immediate mode
                    let imm_flag = (instr >> 5) & 0x1;
                    if imm_flag == 1 {
                        let imm5 = sign_extend(instr & 0x1F, 5);
                        self.reg
                            .set(Register::R0, self.reg.get(Register::R0) + imm5);
                    } else {
                        let _r2 = instr & 0x7;
                        self.reg.set(
                            Register::R0,
                            self.reg.get(Register::R1) + self.reg.get(Register::R2),
                        );
                    }
                    self.reg.update_flags(Register::R0);
                }
                OpCode::OpLdi => {
                    // destination register (DR)
                    let _r0 = (instr >> 9) & 0x7;
                    // PCoffset 9
                    let pc_offset = sign_extend(instr & 0x1FF, 9);
                    // add pc_offset to the current PC, look at that memory location to get the final address
                    let final_address = self
                        .mem
                        .read(self.reg.get(Register::PC).wrapping_add(pc_offset));
                    self.reg.set(Register::R0, self.mem.read(final_address));
                    self.reg.update_flags(Register::R0);
                }
                OpCode::OpAnd => {
                    todo!();
                }
                OpCode::OpNot => {
                    todo!();
                }
                OpCode::OpBr => {
                    todo!();
                }
                OpCode::OpJmp => {
                    todo!();
                }
                OpCode::OpJsr => {
                    todo!();
                }
                OpCode::OpLd => {
                    todo!();
                }
                OpCode::OpLdr => {
                    todo!();
                }
                OpCode::OpLea => {
                    todo!();
                }
                OpCode::OpSt => {
                    todo!();
                }
                OpCode::OpSti => {
                    todo!();
                }
                OpCode::OpStr => {
                    todo!();
                }
                OpCode::OpTrap => {
                    todo!();
                }
                _ => {
                    return Err(format!("Invalid opcode: {}", instr));
                }
            }
        }
    }
}
