pub mod instructions;
pub mod memory;
pub mod register;

use self::instructions::sign_extend;
use self::memory::Memory;
use self::register::Registers;

pub mod lc3;
