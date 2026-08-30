use crate::vm::instruction::Instruction;

pub fn assemble(program: &[Instruction]) -> Vec<u16> {
    program
        .iter()
        .flat_map(|instruction| instruction.encode())
        .collect()
}
