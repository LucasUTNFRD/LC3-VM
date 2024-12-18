# LC3-VM
LC-3 (Little Computer 3) VM implemented in Rust.

## Project Overview

- **Goal**: Build a functional LC-3 virtual machine from scratch.
- **Features**:
  - Simulate the LC-3 architecture.
  - Execute LC-3 assembly programs.

## Reference

The implementation is based on the guide: [Building a Virtual Machine for the LC-3](https://www.jmeiners.com/lc3-vm/).

## TODO
- [ ] Implement the LC-3 architecture.
  - [x] Implement the LC-3 memory.
  - [x] Implement the LC-3 registers.
  - [x] Implement the LC-3 condition flags.
- [ ] Implement the LC-3 instructions.
  - [x] ADD (Add)
  - [x] AND (Bitwise AND)
  - [x] BR (Branch)
  - [x] JMP (Jump)
  - [x] JSR (Jump to Subroutine)
  - [X] LD (Load)
  - [x] LDI (Load Indirect)
  - [x] LDR (Load Register)
  - [x] LEA (Load Effective Address)
  - [x] NOT (Bitwise NOT)
  - [x] ST (Store)
  - [x] STI (Store Indirect)
  - [x] STR (Store Register)
  - [x] TRAP (System Call)
- [x] Document ISA.
- [x] Read assembly programs from files.

