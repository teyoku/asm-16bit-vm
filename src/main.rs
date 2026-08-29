pub mod parser;
use std::{env, error::Error, fs};

use crate::{
    assembler::assemble,
    parser::{parse, parse_u16},
    vm::VirtualMachine,
};

pub mod assembler;
pub mod error;
pub mod instruction;
pub mod register;
pub mod vm;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(format!("Usage: {} <memory-size> <input-file.asm>", args[0]).into());
    }

    let memory_size = parse_u16(&args[1])?;
    let code = fs::read_to_string(&args[2])?;

    let program = parse(&code)?;
    let bytecode = assemble(&program);

    let mut vm = VirtualMachine::new(memory_size as usize);
    vm.load_program(&bytecode, 0)?;
    vm.run()?;
    vm.print_state();

    Ok(())
}
