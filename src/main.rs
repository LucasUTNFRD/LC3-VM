// Load one instruction from memory at the address of the PC register.
// Increment the PC register.
// Look at the opcode to determine which type of instruction it should perform.
// Perform the instruction using the parameters in the instruction.
// Go back to step 1.
mod memory;
mod opdcodes;
mod registers;

use memory::Memory;
use opdcodes::Opcode;
use registers::{Register, Registers};

struct VM {
    memory: Memory,
    registers: Registers,
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
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
    let vm = VM::new();

    loop {
        let instruction = todo!();
        let opcode: Opcode = instruction >> 12;

        match opcode {
            Opcode::Br => todo!(),
            Opcode::Add => todo!(),
            Opcode::Ld => todo!(),
            Opcode::St => todo!(),
            Opcode::Jsr => todo!(),
            Opcode::And => todo!(),
            Opcode::Ldr => todo!(),
            Opcode::Str => todo!(),
            Opcode::Rti => todo!(),
            Opcode::Not => todo!(),
            Opcode::Ldi => todo!(),
            Opcode::Sti => todo!(),
            Opcode::Jmp => todo!(),
            Opcode::Res => todo!(),
            Opcode::Lea => todo!(),
            Opcode::Trap => todo!(),
        };
    }
}
