# LC3-VM
LC-3 (Little Computer 3) VM implemented in Rust.

## Reference

The implementation is based on the guide: [Building a Virtual Machine for the LC-3](https://www.jmeiners.com/lc3-vm/).


## Getting Started

### Clone the Repository

```bash
git clone https://github.com/LucasUTNFRD/LC3-VM.git
cd LC3-VM
```

### Running the VM

You can run the VM using the provided Makefile with:

```bash
# Run with the default example (rogue.obj)
make run

# Run with a specific LC-3 object file
make run FILENAME=path/to/your/program.obj

# Run with a given example
make run FILENAME=examples/{example_name}.obj
```
