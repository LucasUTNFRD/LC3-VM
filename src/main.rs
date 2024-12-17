mod errors;
mod memory;
mod opdcodes;
mod registers;

use errors::VMError;
use memory::Memory;
use opdcodes::{
    add, and, conditional_branch, jmp, jump_subroutine, ldi, load, load_effective_address,
    load_register, not, Opcode,
};
use registers::Registers;

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

    // Implement read_memory method using memory.read wich returns a Result
    pub fn read_memory(&self, address: u16) -> Result<u16, VMError> {
        self.memory.read(address)
    }

    pub fn write_memory(&mut self, address: u16, value: u16) {
        self.memory.write(address, value);
    }

    pub fn read_register(&self, r: usize) -> Result<u16, VMError> {
        self.registers.get(r)
    }

    pub fn write_register(&mut self, r: usize, value: u16) {
        self.registers.set(r, value);
    }

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

            self.execute(opcode, instruction)?;
        }
    }
    fn execute(&mut self, opcode: Opcode, instruction: u16) -> Result<(), VMError> {
        match opcode {
            Opcode::Br => conditional_branch(self, instruction),
            Opcode::Add => add(self, instruction),
            Opcode::Ld => load(self, instruction),
            Opcode::St => todo!(),
            Opcode::Jsr => jump_subroutine(self, instruction),
            Opcode::And => and(self, instruction),
            Opcode::Ldr => load_register(self, instruction),
            Opcode::Str => todo!(),
            Opcode::Rti => todo!(),
            Opcode::Not => not(self, instruction),
            Opcode::Ldi => ldi(self, instruction),
            Opcode::Sti => todo!(),
            Opcode::Jmp => jmp(self, instruction),
            Opcode::Res => todo!(),
            Opcode::Lea => load_effective_address(self, instruction),
            Opcode::Trap => todo!(),
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
