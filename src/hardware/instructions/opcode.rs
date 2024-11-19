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
