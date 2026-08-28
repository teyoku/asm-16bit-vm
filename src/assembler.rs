use crate::instruction::Instruction;

pub fn assemble(program: &[Instruction]) -> Vec<u16> {
    let mut bytecode = Vec::new();
    for instruction in program {
        match instruction {
            Instruction::Halt => bytecode.push(0),
            Instruction::Set(register, address) => {
                bytecode.push((1 << 12) | ((register.as_index() as u16) << 10));
                bytecode.push(*address);
            }
            Instruction::Load(register, address) => {
                bytecode.push((2 << 12) | ((register.as_index() as u16) << 10));
                bytecode.push(*address);
            }
            Instruction::Store(register, address) => {
                bytecode.push((3 << 12) | ((register.as_index() as u16) << 10));
                bytecode.push(*address);
            }
            Instruction::Add(register1, register2) => {
                let reg1 = (register1.as_index() as u16) << 10;
                let reg2 = (register2.as_index() as u16) << 8;
                bytecode.push((4 << 12) | reg1 | reg2);
            }
            Instruction::Sub(register1, register2) => {
                let reg1 = (register1.as_index() as u16) << 10;
                let reg2 = (register2.as_index() as u16) << 8;
                bytecode.push((5 << 12) | reg1 | reg2);
            }
            Instruction::Jmp(address) => {
                bytecode.push(6 << 12);
                bytecode.push(*address);
            }
            Instruction::Jeq(address) => {
                bytecode.push(7 << 12);
                bytecode.push(*address);
            }
            Instruction::Jne(address) => {
                bytecode.push(8 << 12);
                bytecode.push(*address);
            }
            Instruction::Push(register) => {
                bytecode.push(9 << 12 | ((register.as_index() as u16) << 10));
            }
            Instruction::Pop(register) => {
                bytecode.push(10 << 12 | ((register.as_index() as u16) << 10));
            }
            Instruction::Call(address) => {
                bytecode.push(11 << 12);
                bytecode.push(*address);
            }
            Instruction::Ret => bytecode.push(12 << 12),
        }
    }

    bytecode
}
