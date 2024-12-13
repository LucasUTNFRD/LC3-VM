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

    pub fn read_memory(&self, address: u16) -> u16 {
        self.memory.read(address)
    }

    pub fn write_memory(&mut self, address: u16, value: u16) {
        self.memory.write(address, value);
    }

    pub fn read_register(&self, r: Register) -> u16 {
        self.registers.get(r)
    }

    pub fn write_register(&mut self, r: Register, value: u16) {
        self.registers.set(r, value);
    }

    pub fn update_flags(&mut self, r: Register) {
        self.registers.update_flags(r);
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
        let instruction = vm.read_memory(vm.read_register(Register::PC) + 1);
        // TODO: shift instruction and get opcode
        let opcode: Opcode = instruction >> 12;

        match opcode {
            Opcode::Add => todo!(),
            Opcode::Br => todo!(),
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
