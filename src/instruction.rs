use crate::{error::RuntimeError, register::Register};

#[derive(Debug, Clone, Copy, PartialEq)]
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
    And(Register, Register),
    Or(Register, Register),
    Xor(Register, Register),
    Not(Register),
}

impl Instruction {
    pub fn encode(&self) -> Vec<u16> {
        let mut bytecode = Vec::new();
        match self {
            Instruction::Halt => bytecode.push(Opcode::Halt as u16),
            Instruction::Set(register, address) => {
                bytecode.push(((Opcode::Set as u16) << 12) | ((register.as_index() as u16) << 10));
                bytecode.push(*address);
            }
            Instruction::Load(register, address) => {
                bytecode.push(((Opcode::Load as u16) << 12) | ((register.as_index() as u16) << 10));
                bytecode.push(*address);
            }
            Instruction::Store(register, address) => {
                bytecode
                    .push(((Opcode::Store as u16) << 12) | ((register.as_index() as u16) << 10));
                bytecode.push(*address);
            }
            Instruction::Add(register1, register2) => {
                let reg1 = (register1.as_index() as u16) << 10;
                let reg2 = (register2.as_index() as u16) << 8;
                bytecode.push(((Opcode::Add as u16) << 12) | reg1 | reg2);
            }
            Instruction::Sub(register1, register2) => {
                let reg1 = (register1.as_index() as u16) << 10;
                let reg2 = (register2.as_index() as u16) << 8;
                bytecode.push(((Opcode::Sub as u16) << 12) | reg1 | reg2);
            }
            Instruction::Jmp(address) => {
                bytecode.push((Opcode::Jmp as u16) << 12);
                bytecode.push(*address);
            }
            Instruction::Jeq(address) => {
                bytecode.push((Opcode::Jeq as u16) << 12);
                bytecode.push(*address);
            }
            Instruction::Jne(address) => {
                bytecode.push((Opcode::Jne as u16) << 12);
                bytecode.push(*address);
            }
            Instruction::Push(register) => {
                bytecode.push((Opcode::Push as u16) << 12 | ((register.as_index() as u16) << 10));
            }
            Instruction::Pop(register) => {
                bytecode.push((Opcode::Pop as u16) << 12 | ((register.as_index() as u16) << 10));
            }
            Instruction::Call(address) => {
                bytecode.push((Opcode::Call as u16) << 12);
                bytecode.push(*address);
            }
            Instruction::Ret => bytecode.push((Opcode::Ret as u16) << 12),
            Instruction::And(register1, register2) => {
                bytecode.push(
                    (Opcode::And as u16) << 12
                        | ((register1.as_index() as u16) << 10)
                        | ((register2.as_index() as u16) << 8),
                );
            }
            Instruction::Or(register1, register2) => {
                bytecode.push(
                    (Opcode::Or as u16) << 12
                        | ((register1.as_index() as u16) << 10)
                        | ((register2.as_index() as u16) << 8),
                );
            }
            Instruction::Xor(register1, register2) => {
                bytecode.push(
                    (Opcode::Xor as u16) << 12
                        | ((register1.as_index() as u16) << 10)
                        | ((register2.as_index() as u16) << 8),
                );
            }
            Instruction::Not(register) => {
                bytecode.push((Opcode::Not as u16) << 12 | ((register.as_index() as u16) << 10));
            }
        }

        bytecode
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum Opcode {
    Halt = 0,
    Set = 1,
    Load = 2,
    Store = 3,
    Add = 4,
    Sub = 5,
    Jmp = 6,
    Jeq = 7,
    Jne = 8,
    Push = 9,
    Pop = 10,
    Call = 11,
    Ret = 12,
    And = 13,
    Or = 14,
    Xor = 15,
    Not = 16,
}

impl TryFrom<u16> for Opcode {
    type Error = RuntimeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Halt),
            1 => Ok(Opcode::Set),
            2 => Ok(Opcode::Load),
            3 => Ok(Opcode::Store),
            4 => Ok(Opcode::Add),
            5 => Ok(Opcode::Sub),
            6 => Ok(Opcode::Jmp),
            7 => Ok(Opcode::Jeq),
            8 => Ok(Opcode::Jne),
            9 => Ok(Opcode::Push),
            10 => Ok(Opcode::Pop),
            11 => Ok(Opcode::Call),
            12 => Ok(Opcode::Ret),
            13 => Ok(Opcode::And),
            14 => Ok(Opcode::Or),
            15 => Ok(Opcode::Xor),
            16 => Ok(Opcode::Not),
            _ => Err(RuntimeError::InvalidOpcode(value)),
        }
    }
}
