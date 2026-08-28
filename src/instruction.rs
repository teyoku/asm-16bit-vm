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