use crate::{error::AssemblerError, instruction::Instruction, register::Register};

fn parse_register(s: &str) -> Result<Register, AssemblerError> {
    match s {
        "R0" => Ok(Register::R0),
        "R1" => Ok(Register::R1),
        "R2" => Ok(Register::R2),
        "R3" => Ok(Register::R3),
        _ => Err(AssemblerError::InvalidRegister(s.to_string())),
    }
}

pub fn parse_u16(s: &str) -> Result<u16, AssemblerError> {
    s.parse::<u16>()
        .map_err(|_| AssemblerError::ParseIntError(s.to_string()))
}

fn parse_reg_and_u16(data: &[&str]) -> Result<(Register, u16), AssemblerError> {
    let reg_str = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    let reg = parse_register(reg_str)?;

    let value_str = data.get(2).ok_or(AssemblerError::MissingArgument(
        "Expected value".to_string(),
    ))?;
    let value = parse_u16(value_str)?;

    Ok((reg, value))
}

fn parse_two_regs(data: &[&str]) -> Result<(Register, Register), AssemblerError> {
    let reg1_str = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    let reg1 = parse_register(reg1_str)?;

    let reg2_str = data.get(2).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    let reg2 = parse_register(reg2_str)?;

    Ok((reg1, reg2))
}

fn parse_single_u16(data: &[&str]) -> Result<u16, AssemblerError> {
    let address = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected address".to_string(),
    ))?;
    parse_u16(address)
}

fn parse_single_reg(data: &[&str]) -> Result<Register, AssemblerError> {
    let reg_str = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    parse_register(reg_str)
}

pub fn parse(code: &str) -> Result<Vec<Instruction>, AssemblerError> {
    let mut program = Vec::new();

    for line in code.lines() {
        let data: Vec<&str> = line.split_whitespace().collect();
        if data.is_empty() {
            continue;
        }

        match data[0] {
            "HALT" => program.push(Instruction::Halt),
            "SET" => {
                let (reg, value) = parse_reg_and_u16(&data)?;
                program.push(Instruction::Set(reg, value));
            }
            "LOAD" => {
                let (reg, address) = parse_reg_and_u16(&data)?;
                program.push(Instruction::Load(reg, address));
            }
            "STORE" => {
                let (reg, address) = parse_reg_and_u16(&data)?;
                program.push(Instruction::Store(reg, address));
            }
            "ADD" => {
                let (reg1, reg2) = parse_two_regs(&data)?;
                program.push(Instruction::Add(reg1, reg2));
            }
            "SUB" => {
                let (reg1, reg2) = parse_two_regs(&data)?;
                program.push(Instruction::Sub(reg1, reg2));
            }
            "JMP" => {
                let address = parse_single_u16(&data)?;
                program.push(Instruction::Jmp(address));
            }
            "JEQ" => {
                let address = parse_single_u16(&data)?;
                program.push(Instruction::Jeq(address));
            }
            "JNE" => {
                let address = parse_single_u16(&data)?;
                program.push(Instruction::Jne(address));
            }
            "PUSH" => {
                let reg = parse_single_reg(&data)?;
                program.push(Instruction::Push(reg));
            }
            "POP" => {
                let reg = parse_single_reg(&data)?;
                program.push(Instruction::Pop(reg));
            }
            "CALL" => {
                let address = parse_single_u16(&data)?;
                program.push(Instruction::Call(address));
            }
            "RET" => program.push(Instruction::Ret),
            instruction => return Err(AssemblerError::UnknownInstruction(instruction.to_string())),
        }
    }

    Ok(program)
}
