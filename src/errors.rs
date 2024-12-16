#[derive(Debug)]
pub enum VMError {
    InvalidMemoryAccess,
    InvalidRegister,
}
