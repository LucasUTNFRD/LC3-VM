#[derive(Debug)]
pub enum VMError {
    InvalidMemoryAccess(u16), // This includes the address that was attempted to be accessed
    InvalidRegister,
    GetcFailed,
    InvalidCharacter,
    TrapError(TrapError),
}

#[derive(Debug)]
pub enum TrapError {
    InvalidVector(u16),
    IOError(String),
    Halt,
    InvalidCharacterCode(u16),
}

// TODO: Implement the Display trait for VMError (not done bc rn im not using it)
