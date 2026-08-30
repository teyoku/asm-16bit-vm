use std::collections::HashMap;

use crate::{error::AssemblerError, instruction::Instruction, register::Register};

pub fn parse_u16(s: &str) -> Result<u16, AssemblerError> {
    if s.starts_with("0x") {
        u16::from_str_radix(&s[2..], 16).map_err(|_| AssemblerError::ParseIntError(s.to_string()))
    } else {
        s.parse::<u16>()
            .map_err(|_| AssemblerError::ParseIntError(s.to_string()))
    }
}

fn parse_reg_and_u16(
    data: &[&str],
    labels: &HashMap<String, u16>,
) -> Result<(Register, u16), AssemblerError> {
    let &reg_str = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    let reg = reg_str.try_into()?;

    let value_str = data.get(2).ok_or(AssemblerError::MissingArgument(
        "Expected value".to_string(),
    ))?;
    let value = parse_address_or_label(value_str, labels)?;

    Ok((reg, value))
}

fn parse_two_regs(data: &[&str]) -> Result<(Register, Register), AssemblerError> {
    let &reg1_str = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    let reg1 = reg1_str.try_into()?;

    let &reg2_str = data.get(2).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    let reg2 = reg2_str.try_into()?;

    Ok((reg1, reg2))
}

fn parse_single_u16(data: &[&str], labels: &HashMap<String, u16>) -> Result<u16, AssemblerError> {
    let address = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected address".to_string(),
    ))?;
    parse_address_or_label(address, labels)
}

fn parse_single_reg(data: &[&str]) -> Result<Register, AssemblerError> {
    let &reg_str = data.get(1).ok_or(AssemblerError::MissingArgument(
        "Expected register".to_string(),
    ))?;
    reg_str.try_into()
}

fn parse_address_or_label(s: &str, labels: &HashMap<String, u16>) -> Result<u16, AssemblerError> {
    if let Some(&addr) = labels.get(s) {
        return Ok(addr);
    }

    parse_u16(s).map_err(|_| AssemblerError::UnknownLabel(s.to_string()))
}

pub fn parse(code: &str) -> Result<Vec<Instruction>, AssemblerError> {
    let mut labels = HashMap::new();
    let mut current_address = 0u16;

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

        if data[0].ends_with(':') {
            if let Some(label) = data[0].strip_suffix(':') {
                labels.insert(label.to_string(), current_address);
            }
        } else {
            match data[0] {
                "HALT" | "ADD" | "SUB" | "PUSH" | "POP" | "RET" | "AND" | "OR" | "XOR" | "NOT" => {
                    current_address += 1
                }
                "SET" | "LOAD" | "STORE" | "JMP" | "JEQ" | "JNE" | "CALL" => current_address += 2,
                _ => (),
            }
        }
    }

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

        if data[0].ends_with(':') {
            continue;
        }

        match data[0] {
            "HALT" => program.push(Instruction::Halt),
            "SET" => {
                let (reg, value) = parse_reg_and_u16(&data, &labels)?;
                program.push(Instruction::Set(reg, value));
            }
            "LOAD" => {
                let (reg, address) = parse_reg_and_u16(&data, &labels)?;
                program.push(Instruction::Load(reg, address));
            }
            "STORE" => {
                let (reg, address) = parse_reg_and_u16(&data, &labels)?;
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
                let address = parse_single_u16(&data, &labels)?;
                program.push(Instruction::Jmp(address));
            }
            "JEQ" => {
                let address = parse_single_u16(&data, &labels)?;
                program.push(Instruction::Jeq(address));
            }
            "JNE" => {
                let address = parse_single_u16(&data, &labels)?;
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
                let address = parse_single_u16(&data, &labels)?;
                program.push(Instruction::Call(address));
            }
            "RET" => program.push(Instruction::Ret),
            "AND" => {
                let (reg1, reg2) = parse_two_regs(&data)?;
                program.push(Instruction::And(reg1, reg2));
            }
            "OR" => {
                let (reg1, reg2) = parse_two_regs(&data)?;
                program.push(Instruction::Or(reg1, reg2));
            }
            "XOR" => {
                let (reg1, reg2) = parse_two_regs(&data)?;
                program.push(Instruction::Xor(reg1, reg2));
            }
            "NOT" => {
                let reg = parse_single_reg(&data)?;
                program.push(Instruction::Not(reg));
            }
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

    #[test]
    fn test_labels() {
        let code = "
        SET R0 10
        loop_start:
            SUB R0 R1
            JNE loop_start
        HALT
        ";
        let program = parse(code).unwrap();
        assert_eq!(program[2], Instruction::Jne(2));
    }
}
