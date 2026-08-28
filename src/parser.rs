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
                let reg_str = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                let reg = parse_register(reg_str)?;
                let value_str = data.get(2).ok_or(AssemblerError::MissingArgument(
                    "Expected value".to_string(),
                ))?;
                let value = parse_u16(value_str)?;
                program.push(Instruction::Set(reg, value));
            }
            "LOAD" => {
                let reg_str = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                let reg = parse_register(reg_str)?;
                let address_str = data.get(2).ok_or(AssemblerError::MissingArgument(
                    "Expected address".to_string(),
                ))?;
                let address = parse_u16(address_str)?;
                program.push(Instruction::Load(reg, address));
            }
            "STORE" => {
                let reg_str = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                let reg = parse_register(reg_str)?;
                let address_str = data.get(2).ok_or(AssemblerError::MissingArgument(
                    "Expected address".to_string(),
                ))?;
                let address = parse_u16(address_str)?;
                program.push(Instruction::Store(reg, address));
            }
            "ADD" => {
                let reg1_str = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                let reg1 = parse_register(reg1_str)?;
                let reg2_str = data.get(2).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                let reg2 = parse_register(reg2_str)?;
                program.push(Instruction::Add(reg1, reg2));
            }
            "SUB" => {
                let reg1_str = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                let reg1 = parse_register(reg1_str)?;
                let reg2_str = data.get(2).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                let reg2 = parse_register(reg2_str)?;
                program.push(Instruction::Sub(reg1, reg2));
            }
            "JMP" => {
                let address = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected address".to_string(),
                ))?;
                program.push(Instruction::Jmp(parse_u16(address)?));
            }
            "JEQ" => {
                let address = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected address".to_string(),
                ))?;
                program.push(Instruction::Jeq(parse_u16(address)?));
            }
            "JNE" => {
                let address = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected address".to_string(),
                ))?;
                program.push(Instruction::Jne(parse_u16(address)?));
            }
            "PUSH" => {
                let reg = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                program.push(Instruction::Push(parse_register(reg)?));
            }
            "POP" => {
                let reg = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected register".to_string(),
                ))?;
                program.push(Instruction::Pop(parse_register(reg)?));
            }
            "CALL" => {
                let address = data.get(1).ok_or(AssemblerError::MissingArgument(
                    "Expected address".to_string(),
                ))?;
                program.push(Instruction::Call(parse_u16(address)?));
            }
            "RET" => program.push(Instruction::Ret),
            instruction => return Err(AssemblerError::UnknownInstruction(instruction.to_string())),
        }
    }

    Ok(program)
}
