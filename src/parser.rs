use crate::{error::AssemblerError, instruction::Instruction, register::Register};

pub fn parse_u16(s: &str) -> Result<u16, AssemblerError> {
    if s.starts_with("0x") {
        u16::from_str_radix(&s[2..], 16).map_err(|_| AssemblerError::ParseIntError(s.to_string()))
    } else {
        s.parse::<u16>()
            .map_err(|_| AssemblerError::ParseIntError(s.to_string()))
    }
}

fn parse_reg_and_u16(data: &[&str]) -> Result<(Register, u16), AssemblerError> {
    let reg_str = data
        .get(1)
        .ok_or(AssemblerError::MissingArgument(
            "Expected register".to_string(),
        ))?
        .to_owned();
    let reg = reg_str.try_into()?;

    let value_str = data.get(2).ok_or(AssemblerError::MissingArgument(
        "Expected value".to_string(),
    ))?;
    let value = parse_u16(value_str)?;

    Ok((reg, value))
}

fn parse_two_regs(data: &[&str]) -> Result<(Register, Register), AssemblerError> {
    let reg1_str = data
        .get(1)
        .ok_or(AssemblerError::MissingArgument(
            "Expected register".to_string(),
        ))?
        .to_owned();
    let reg1 = reg1_str.try_into()?;

    let reg2_str = data
        .get(2)
        .ok_or(AssemblerError::MissingArgument(
            "Expected register".to_string(),
        ))?
        .to_owned();
    let reg2 = reg2_str.try_into()?;

    Ok((reg1, reg2))
}

fn parse_single_u16(data: &[&str]) -> Result<u16, AssemblerError> {
    let address = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected address".to_string(),
    ))?;
    parse_u16(address)
}

fn parse_single_reg(data: &[&str]) -> Result<Register, AssemblerError> {
    let reg_str = data
        .get(1)
        .ok_or(AssemblerError::MissingArgument(
            "Expected register".to_string(),
        ))?
        .to_owned();
    reg_str.try_into()
}

pub fn parse(code: &str) -> Result<Vec<Instruction>, AssemblerError> {
    let mut program = Vec::new();

    for line in code.lines() {
        // Handling comments
        let fixed_line = if let Some((part, _)) = line.split_once(';') {
            part
        } else {
            line
        };

        let data: Vec<&str> = fixed_line.split_whitespace().collect();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u16() {
        assert_eq!(parse_u16("1024").unwrap(), 1024);
        assert_eq!(parse_u16("0x0A").unwrap(), 10);
        assert!(matches!(
            parse_u16("hello!"),
            Err(AssemblerError::ParseIntError(_)),
        ));
    }

    #[test]
    fn test_parse_comment() {
        assert_eq!(
            parse("ADD R0 R1 ; don't touch this!").unwrap(),
            vec![Instruction::Add(Register::R0, Register::R1)]
        );
    }
}
