use super::{
    instructions::{self, sign_extend},
    memory::Memory,
    register::{Register, Registers},
};

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
                    instructions::add::add(instr, &mut self.reg);
                }
                OpCode::OpLdi => {
                    todo!();
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
