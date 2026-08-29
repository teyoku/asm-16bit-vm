use crate::register::Register;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Halt,
    Set(Register, u16),
    Load(Register, u16),
    Store(Register, u16),
    Add(Register, Register),
    Sub(Register, Register),
    Jmp(u16),
    Jeq(u16),
    Jne(u16),
    Push(Register),
    Pop(Register),
    Call(u16),
    Ret,
}

impl Instruction {
    pub fn encode(&self) -> Vec<u16> {
        let mut bytecode = Vec::new();
        match self {
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
        
        bytecode
    }
}