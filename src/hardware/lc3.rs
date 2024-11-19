use super::{
    instructions::{self, opcode::OpCode},
    memory::Memory,
    register::{Register, Registers},
};

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
            let op: OpCode = OpCode::from(instr >> 12);
            match op {
                OpCode::OpAdd => {
                    //TODO fix this long line
                    instructions::add::add(instr, &mut self.reg);
                }
                OpCode::OpLdi => {
                    instructions::ldi::ldi(instr, &mut self.reg, &self.mem);
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
                    instructions::ldr::ldr(instr, &mut self.reg, &self.mem);
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
