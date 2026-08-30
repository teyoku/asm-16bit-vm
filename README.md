# 16-Bit Virtual Machine & Assembler

A custom 16-bit virtual machine (VM) and assembler written entirely in Rust. This project implements a complete execution engine that can parse human-readable assembly, assemble it into bytecode, and execute it in an isolated memory environment.

The architecture is built with a focus on core systems programming principles, leveraging Rust's ownership model, safe memory management, and static dispatch, running entirely in a predictable, single-threaded environment. It follows the **Von Neumann architecture**, treating both the assembled program instructions and the runtime data as residing within the same unified, isolated memory space.

## Features

- **Custom 16-bit Architecture:** Opcodes and registers are tightly packed into 16-bit words. Follows a unified Von Neumann memory model where instructions and data share the same memory structure.
- **General Purpose Registers:** 4 explicitly accessible registers (`R0`, `R1`, `R2`, `R3`).
- **Memory Management:** Configurable, unified memory size containing both bytecode and data, managed via an independent struct, with a built-in stack pointer (`SP`) and program counter (`PC`).
- **Instruction Set:** Supports arithmetic operations, bitwise operations, memory I/O, stack manipulation, and conditional branching.
- **Assembler Directives:** Support for inline comments (`;`), label definitions (`label:`), and memory address mapping for jumps.
- **Zero Flag:** Used for conditional jumps based on arithmetic and bitwise results (`JEQ`, `JNE`).
- **Robust Parsing & Error Handling:** Support for reading base-10 integer and hexadecimal (`0xABCD`) literals, alongside comprehensive assembler and runtime error definitions implementing standard `Error` traits.

## Usage

The VM requires two arguments to run: the size of the memory to allocate and the path to the assembly source file. The program is automatically loaded into memory starting at offset `0`.

```bash
cargo run <memory-size> <input-file.asm>
```

**Example:**

```bash
cargo run 1024 program.asm
```

## Instruction Set

| Mnemonic | Arguments       | Description                                                                                               |
| :------- | :-------------- | :-------------------------------------------------------------------------------------------------------- |
| `HALT`   | None            | Stops the execution of the virtual machine.                                                               |
| `SET`    | `<REG> <VAL>`   | Sets the register to a literal 16-bit value.                                                              |
| `LOAD`   | `<REG> <ADDR>`  | Loads a value from the memory address into the register.                                                  |
| `STORE`  | `<REG> <ADDR>`  | Stores the value of a register into a memory address.                                                     |
| `ADD`    | `<REG1> <REG2>` | Adds `<REG2>` to `<REG1>`, storing the result in `<REG1>`. Sets the Zero flag if the result is 0.         |
| `SUB`    | `<REG1> <REG2>` | Subtracts `<REG2>` from `<REG1>`, storing the result in `<REG1>`. Sets the Zero flag if the result is 0.  |
| `AND`    | `<REG1> <REG2>` | Bitwise AND `<REG2>` to `<REG1>`, storing the result in `<REG1>`. Sets the Zero flag if the result is 0.  |
| `OR`     | `<REG1> <REG2>` | Bitwise OR `<REG2>` to `<REG1>`, storing the result in `<REG1>`. Sets the Zero flag if the result is 0.   |
| `XOR`    | `<REG1> <REG2>` | Bitwise XOR `<REG2>` to `<REG1>`, storing the result in `<REG1>`. Sets the Zero flag if the result is 0.  |
| `NOT`    | `<REG>`         | Bitwise NOT the register, storing the result back in the register. Sets the Zero flag if the result is 0. |
| `JMP`    | `<ADDR\|LABEL>` | Unconditionally jumps to the specified memory address or label.                                           |
| `JEQ`    | `<ADDR\|LABEL>` | Jumps to the specified address or label if the Zero flag is true.                                         |
| `JNE`    | `<ADDR\|LABEL>` | Jumps to the specified address or label if the Zero flag is false.                                        |
| `PUSH`   | `<REG>`         | Pushes the value of the register onto the stack.                                                          |
| `POP`    | `<REG>`         | Pops a value from the stack into the register.                                                            |
| `CALL`   | `<ADDR\|LABEL>` | Pushes the current PC onto the stack and jumps to the given address or label.                             |
| `RET`    | None            | Pops the return address from the stack and jumps back to it.                                              |

## Example Program

```assembly
; Initialize registers
SET R0 0x000A ; Set R0 to 10
SET R1 15     ; Set R1 to 15

loop_start:
    ADD R0 R1
    STORE R0 100

    ; Logic to exit loop
    SET R2 25
    SUB R2 R0
    JNE loop_start ; Jump if R0 is not 25

HALT
```

_(This program demonstrates basic arithmetic, setting values via hex and decimal literals, inline comments, saving to memory, and using a label for a conditional jump before halting.)_

## Architecture Notes

The engine interprets and executes assembled machine code synchronously directly from the VM's memory structure. To maintain predictable state transitions and memory safety, the VM strictly utilizes static dispatch and operates single-threaded, ensuring straightforward debugging and state execution tracking.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.md) file for details.