# 16-Bit Virtual Machine & Assembler

A custom 16-bit virtual machine (VM) and assembler written entirely in Rust. This project implements a complete execution engine that can parse human-readable assembly, assemble it into bytecode, and execute it in an isolated memory environment.

The architecture is built with a focus on core systems programming principles, leveraging Rust's ownership model, safe memory management, and static dispatch, running entirely in a predictable, single-threaded environment.

## Features

- **Custom 16-bit Architecture:** Opcodes and registers are tightly packed into 16-bit words.
- **General Purpose Registers:** 4 explicitly accessible registers (`R0`, `R1`, `R2`, `R3`).
- **Memory Management:** Configurable memory size with a built-in stack pointer (`SP`) and program counter (`PC`).
- **Instruction Set:** Supports arithmetic operations, memory I/O, stack manipulation, and conditional branching.
- **Zero Flag:** Used for conditional jumps based on arithmetic results (`JEQ`, `JNE`).
- **Robust Error Handling:** Comprehensive assembler and runtime error definitions implementing standard `Error` traits.

## Usage

The VM requires two arguments to run: the size of the memory to allocate and the path to the assembly source file.

```bash
cargo run <memory-size> <input-file.asm>
```

**Example:**

```bash
cargo run 1024 program.asm
```

## Instruction Set

| Mnemonic | Arguments       | Description                                                                                              |
| :------- | :-------------- | :------------------------------------------------------------------------------------------------------- |
| `HALT`   | None            | Stops the execution of the virtual machine.                                                              |
| `SET`    | `<REG> <VAL>`   | Sets the register to a literal 16-bit value.                                                             |
| `LOAD`   | `<REG> <ADDR>`  | Loads a value from the memory address into the register.                                                 |
| `STORE`  | `<REG> <ADDR>`  | Stores the value of a register into a memory address.                                                    |
| `ADD`    | `<REG1> <REG2>` | Adds `<REG2>` to `<REG1>`, storing the result in `<REG1>`. Sets the Zero flag if the result is 0.        |
| `SUB`    | `<REG1> <REG2>` | Subtracts `<REG2>` from `<REG1>`, storing the result in `<REG1>`. Sets the Zero flag if the result is 0. |
| `JMP`    | `<ADDR>`        | Unconditionally jumps to the specified memory address.                                                   |
| `JEQ`    | `<ADDR>`        | Jumps to the specified address if the Zero flag is true.                                                 |
| `JNE`    | `<ADDR>`        | Jumps to the specified address if the Zero flag is false.                                                |
| `PUSH`   | `<REG>`         | Pushes the value of the register onto the stack.                                                         |
| `POP`    | `<REG>`         | Pops a value from the stack into the register.                                                           |
| `CALL`   | `<ADDR>`        | Pushes the current PC onto the stack and jumps to the given address.                                     |
| `RET`    | None            | Pops the return address from the stack and jumps back to it.                                             |

## Example Program

```assembly
SET R0 10
SET R1 15
ADD R0 R1
STORE R0 100
HALT
```

_(This program sets `R0` to 10, `R1` to 15, adds them, stores the result (25) at memory address 100, and halts the machine.)_

## Architecture Notes

The engine interprets and executes assembled machine code synchronously. To maintain predictable state transitions and memory safety, the VM strictly utilizes static dispatch and operates single-threaded, ensuring straightforward debugging and state execution tracking.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.md) file for details.