mod errors;
mod memory;
mod opdcodes;
mod registers;

use std::{fs::File, io::Read};

use errors::{TrapError, VMError};
use memory::Memory;
use opdcodes::*;
use registers::Registers;
use termios::*;

struct VM {
    memory: Memory,
    registers: Registers,
    state: VMState,
}

#[derive(Debug, PartialEq)]
enum VMState {
    Running,
    Halted,
}

impl VM {
    /// Creates a new VM instance with initialized memory and registers
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
            state: VMState::Running,
        }
    }

    /// Reads a 16-bit value from the specified memory address
    ///
    /// # Errors
    /// Returns `VMError::InvalidMemoryAccess` if address is invalid
    pub fn read_memory(&mut self, address: u16) -> Result<u16, VMError> {
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

    /// Loads an LC-3 program file into memory
    ///
    /// # Arguments
    /// * `file` - Path to the .obj file to load
    ///
    /// # Process
    /// 1. Opens and reads the file into a buffer
    /// 2. Extracts the origin address from the first two bytes
    /// 3. Loads each subsequent 16-bit instruction into memory starting at origin
    ///
    /// # Errors
    /// * `VMError::OpenFileFailed` - If file cannot be opened
    /// * `VMError::LoadFailed` - If file format is invalid
    /// * `VMError::InvalidMemoryAccess` - If program would load to invalid address
    pub fn load_program(&mut self, file: &str) -> Result<(), VMError> {
        let mut file = File::open(file).map_err(|_| VMError::OpenFileFailed(file.to_string()))?;

        let mut buffer: Vec<u8> = Vec::new();

        file.read_to_end(&mut buffer)
            .map_err(|_| VMError::LoadFailed)?;

        let origin = match (buffer.first(), buffer.get(1)) {
            (Some(&first_byte), Some(&second_byte)) => {
                u16::from_be_bytes([first_byte, second_byte])
            }
            _ => return Err(VMError::LoadFailed),
        };

        let mut current_address = origin;

        for chunk in buffer.chunks_exact(2).skip(1) {
            // check that the chunk is the correct size
            if chunk.len() != 2 {
                return Err(VMError::LoadFailed);
            }

            let instruction = match (chunk.first(), chunk.get(1)) {
                (Some(&first_byte), Some(&second_byte)) => {
                    u16::from_be_bytes([first_byte, second_byte])
                }
                _ => return Err(VMError::LoadFailed),
            };

            self.write_memory(current_address, instruction)?;
            current_address = current_address.wrapping_add(1);
        }

        Ok(())
    }

    /// Runs the VM's main execution loop
    ///
    /// # Process
    /// 1. Fetches instruction from memory at PC
    /// 2. Increments PC
    /// 3. Decodes instruction opcode
    /// 4. Executes instruction
    /// 5. Repeats until halted
    ///
    /// # Errors
    /// Returns VMError if instruction execution fails
    pub fn run(&mut self) -> Result<(), VMError> {
        while self.state == VMState::Running {
            // 1. Load one instruction from memory at the address of the PC
            let instruction = self.read_memory(self.registers.pc)?;

            // 2. Increment the PC
            self.registers.pc = self.registers.pc.wrapping_add(1);

            let instruction_read = (instruction >> 12) & 0xF;
            let opcode: Opcode = Opcode::from(instruction_read);

            self.execute(opcode, instruction)?;
        }
        Ok(())
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
            Opcode::Rti => Err(VMError::UnimplemedOpcode(Opcode::Rti)),
            Opcode::Not => not(self, instruction),
            Opcode::Ldi => ldi(self, instruction),
            Opcode::Sti => store_indirect(self, instruction),
            Opcode::Jmp => jmp(self, instruction),
            Opcode::Res => Err(VMError::UnimplemedOpcode(Opcode::Res)),
            Opcode::Lea => load_effective_address(self, instruction),
            Opcode::Trap => trap(self, instruction),
        }
    }
}

fn main() {
    // Configure termios
    let mut termios = if let Ok(termios) = Termios::from_fd(0) {
        termios
    } else {
        eprintln!("Failed to get termios settings");
        std::process::exit(1);
    };

    //turn on canonical mode and echo mode
    termios.c_lflag &= !(ICANON | ECHO);

    if let Err(e) = tcsetattr(0, TCSAFLUSH, &termios) {
        eprintln!("Failed to set termios settings: {:?}", e);
        std::process::exit(1);
    }

    // Read the program file given as the first command line argument
    // This will be used ./lc3-vm path/to/program.obj
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ./lc3-vm path/to/program.obj");
        std::process::exit(1);
    }

    let filename = if let Some(name) = args.get(1) {
        name
    } else {
        eprintln!("No program file provided.");
        std::process::exit(1);
    };

    // Main loop
    let mut vm = VM::new();

    // TODO: Load the program into memory
    if vm.load_program(filename).is_err() {
        eprintln!("Error loading program: {:?}", filename);
        std::process::exit(1);
    }

    match vm.run() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            match e {
                VMError::InvalidMemoryAccess(addr) => {
                    eprintln!("Invalid memory access at address: 0x{:04X}", addr);
                    std::process::exit(1);
                }
                VMError::UnimplemedOpcode(opcode) => {
                    eprintln!("Unimplemented opcode: {:?}", opcode);
                    std::process::exit(1);
                }
                VMError::TrapError(trap_error) => match trap_error {
                    TrapError::IOError(msg) => {
                        eprintln!("IO error: {:?}", msg);
                        std::process::exit(1);
                    }
                    TrapError::InvalidTrapVector(vector) => {
                        eprintln!("Invalid trap vector: 0x{:04X}", vector);
                        std::process::exit(1);
                    }
                },
                VMError::OpenFileFailed(path) => {
                    eprintln!("Failed to open file: {:?}", path);
                    std::process::exit(1);
                }

                _ => {
                    eprintln!("VM error: {:?}", e);
                    std::process::exit(1);
                }
            }
            // eprintln!("VM error: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use registers::RegisterFlags;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_load_program() {
        let mut vm = VM::new();

        const PATH: &str = "examples/hello-world.obj";
        //print the current path to check if the file is being read
        match vm.load_program(PATH) {
            Ok(_) => (),
            Err(e) => println!("Error: {:?}", e),
        }

        for i in 0..16 {
            let value = vm.read_memory(0x3000 + i).unwrap();
            println!("Memory[0x{:04X}] = 0x{:04X}", 0x3000 + i, value);
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::as_conversions)]
    fn test_load_and_run_simple_add() -> Result<(), VMError> {
        // Create VM and load program
        let expected_values = [
            0x5020, // AND R0, R0, #0
            0x1025, // ADD R0, R0, #5
            0x5260, // AND R1, R1, #0
            0x1263, // ADD R1, R1, #3
            0x1401, // ADD R2, R0, R1
            0xF025, // TRAP x25 -> HALT
        ];
        const PATH: &str = "examples/simple_add.obj";
        let mut vm = VM::new();
        vm.load_program(PATH)?;

        // Check that the loaded program is correct
        for (i, &expected) in expected_values.iter().enumerate() {
            let value = vm.read_memory(0x3000 + i as u16)?;
            assert_eq!(
                value,
                expected,
                "Memory[0x{:04X}] should be 0x{:04X}",
                0x3000 + i as u16,
                expected
            );
        }

        // Run the program
        vm.run()?;

        // Verify final register values
        assert_eq!(vm.read_register(0)?, 5, "R0 should contain 5");

        assert_eq!(vm.read_register(1)?, 3, "R1 should contain 3");

        assert_eq!(
            vm.read_register(2)?,
            8,
            "R2 should contain 8 (sum of R0 and R1)"
        );

        // Verify condition flags
        // Result was positive (8), so positive flag should be set
        assert_eq!(
            vm.registers.condition,
            RegisterFlags::Pos,
            "Condition flags should be set to positive after addition"
        );

        Ok(())
    }
}
