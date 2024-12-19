use crate::Opcode;

#[derive(Debug)]
pub enum VMError {
    InvalidMemoryAccess(u16), // This includes the address that was attempted to be accessed
    InvalidRegister,
    UnimplemedOpcode(Opcode),
    InvalidCharacter,
    TrapError(TrapError),
    LoadFailed,
    OpenFileFailed(String),
}

#[derive(Debug)]
pub enum TrapError {
    IOError(String),
    InvalidTrapVector(u16),
}
