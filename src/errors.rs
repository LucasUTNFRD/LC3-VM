#[derive(Debug)]
pub enum VMError {
    InvalidMemoryAccess(u16), // This includes the address that was attempted to be accessed
    InvalidRegister,
    GetcFailed,
    InvalidCharacter,
}

// TODO: Implement the Display trait for VMError (not done bc rn im not using it)
