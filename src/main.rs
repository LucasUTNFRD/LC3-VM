mod memory;
mod opdcodes;
mod registers;

use memory::Memory;
use opdcodes::Opcode;
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

    pub fn read_memory(&self, address: u16) -> u16 {
        self.memory.read(address)
    }

    pub fn write_memory(&mut self, address: u16, value: u16) {
        self.memory.write(address, value);
    }

    pub fn read_register(&self, r: usize) -> u16 {
        self.registers.get(r).unwrap_or(0)
    }

    pub fn write_register(&mut self, r: usize, value: u16) {
        self.registers.set(r, value);
    }

    pub fn update_flags(&mut self, r: usize) {
        self.registers.update_flags(r);
    }

    pub fn run(&mut self) {
        loop {
            // 1. Load one instruction from memory at the address of the PC
            let instruction = self.read_memory(self.registers.pc);

            // 2. Increment the PC
            self.registers.pc = self.registers.pc.wrapping_add(1);

            let opcode: Opcode = Opcode::from((instruction >> 12) & 0xF);

            self.execute(opcode, instruction);
        }
    }
    pub fn execute(&mut self, opcode: Opcode, instruction: u16) {
        todo!()
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
