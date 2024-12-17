mod errors;
mod memory;
mod opdcodes;
mod registers;

use errors::{TrapError, VMError};
use memory::Memory;
use opdcodes::*;
use registers::Registers;

struct VM {
    memory: Memory,
    registers: Registers,
}

impl VM {
    /// Creates a new VM instance with initialized memory and registers
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    /// Reads a 16-bit value from the specified memory address
    ///
    /// # Errors
    /// Returns `VMError::InvalidMemoryAccess` if address is invalid
    pub fn read_memory(&self, address: u16) -> Result<u16, VMError> {
        self.memory.read(address)
    }

    /// Writes a 16-bit value to the specified memory address
    ///
    /// # Errors
    /// Returns `VMError::InvalidMemoryAccess` if address is invalid
    pub fn write_memory(&mut self, address: u16, value: u16) -> Result<(), VMError> {
        self.memory.write(address, value)
    }

    /// Reads the value of the specified register
    ///
    /// # Errors
    /// Returns `VMError::InvalidRegister` if register number is invalid
    pub fn read_register(&self, r: usize) -> Result<u16, VMError> {
        self.registers.get(r)
    }

    /// Writes a 16-bit value to the specified register
    pub fn write_register(&mut self, r: usize, value: u16) {
        self.registers.set(r, value);
    }

    /// Updates the condition flags based on the value in the specified register
    pub fn update_flags(&mut self, r: usize) {
        self.registers.update_flags(r);
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        loop {
            // 1. Load one instruction from memory at the address of the PC
            let instruction = self.read_memory(self.registers.pc)?;

            // 2. Increment the PC
            self.registers.pc = self.registers.pc.wrapping_add(1);

            let instruction = (instruction >> 12) & 0xF;
            let opcode: Opcode = Opcode::from(instruction);

            match self.execute(opcode, instruction) {
                // Continue running if Ok or if error is not Halt
                Ok(_) => continue,
                Err(VMError::TrapError(TrapError::Halt)) => {
                    println!("Program terminated normally.");
                    return Ok(());
                }
                Err(e) => return Err(e), // Propagate other errors
            }
        }
    }

    fn execute(&mut self, opcode: Opcode, instruction: u16) -> Result<(), VMError> {
        match opcode {
            Opcode::Br => conditional_branch(self, instruction),
            Opcode::Add => add(self, instruction),
            Opcode::Ld => load(self, instruction),
            Opcode::St => store(self, instruction),
            Opcode::Jsr => jump_subroutine(self, instruction),
            Opcode::And => and(self, instruction),
            Opcode::Ldr => load_register(self, instruction),
            Opcode::Str => store_register(self, instruction),
            Opcode::Rti => Ok(()), // RTI is not implemented
            Opcode::Not => not(self, instruction),
            Opcode::Ldi => ldi(self, instruction),
            Opcode::Sti => store_indirect(self, instruction),
            Opcode::Jmp => jmp(self, instruction),
            Opcode::Res => Ok(()), // Res is not implemented
            Opcode::Lea => load_effective_address(self, instruction),
            Opcode::Trap => trap(self, instruction),
        }
    }
}

fn main() {
    // Read the program file given as the first command line argument
    // This will be used ./lc3-vm path/to/program.obj
    // ensure that the argument is passed and the file is readable
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {:?} [image-file1] ...", args.first());
    }

    // TODO: Load the program into memory

    // Main loop
    let mut vm = VM::new();

    vm.run();
}
