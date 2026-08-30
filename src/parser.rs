use std::collections::HashMap;

use crate::{
    error::AssemblerError,
    vm::{instruction::Instruction, register::Register},
};

pub struct Parser {
    labels: HashMap<String, u16>,
    current_address: u16,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            current_address: 0,
        }
    }

    fn parse_u16(s: &str) -> Result<u16, AssemblerError> {
        if s.starts_with("0x") {
            u16::from_str_radix(&s[2..], 16)
                .map_err(|_| AssemblerError::ParseIntError(s.to_string()))
        } else {
            s.parse::<u16>()
                .map_err(|_| AssemblerError::ParseIntError(s.to_string()))
        }
    }

    fn parse_address_or_label(&self, s: &str) -> Result<u16, AssemblerError> {
        if let Some(&addr) = self.labels.get(s) {
            return Ok(addr);
        }

        Self::parse_u16(s).map_err(|_| AssemblerError::UnknownLabel(s.to_string()))
    }

    fn get_arg<'a>(
        data: &[&'a str],
        index: usize,
        eror_msg: &str,
    ) -> Result<&'a str, AssemblerError> {
        data.get(index)
            .copied()
            .ok_or_else(|| AssemblerError::MissingArgument(eror_msg.to_string()))
    }

    fn parse_reg_and_u16(&self, data: &[&str]) -> Result<(Register, u16), AssemblerError> {
        let reg = Self::get_arg(data, 1, "Expected register")?.try_into()?;
        let value_str = Self::get_arg(data, 2, "Expected value")?;
        let value = self.parse_address_or_label(value_str)?;

        Ok((reg, value))
    }

    fn parse_two_regs(&self, data: &[&str]) -> Result<(Register, Register), AssemblerError> {
        let reg1 = Self::get_arg(data, 1, "Expected first register")?.try_into()?;
        let reg2 = Self::get_arg(data, 2, "Expected second register")?.try_into()?;

        Ok((reg1, reg2))
    }

    fn parse_single_u16(&self, data: &[&str]) -> Result<u16, AssemblerError> {
        let address = Self::get_arg(data, 1, "Expected address")?;
        self.parse_address_or_label(address)
    }

    fn parse_single_reg(&self, data: &[&str]) -> Result<Register, AssemblerError> {
        Self::get_arg(data, 1, "Expected register")?.try_into()
    }

    pub fn parse(&mut self, code: &str) -> Result<Vec<Instruction>, AssemblerError> {
        for line in code.lines() {
            // Handling comments
            let fixed_line = line.split_once(';').map_or(line, |(code, _)| code);

            let data: Vec<&str> = fixed_line.split_whitespace().collect();
            if data.is_empty() {
                continue;
            }

            if data[0].ends_with(':') {
                if let Some(label) = data[0].strip_suffix(':') {
                    self.labels.insert(label.to_string(), self.current_address);
                }
            } else {
                match data[0] {
                    "HALT" | "ADD" | "SUB" | "PUSH" | "POP" | "RET" | "AND" | "OR" | "XOR"
                    | "NOT" => self.current_address += 1,
                    "SET" | "LOAD" | "STORE" | "JMP" | "JEQ" | "JNE" | "JGT" | "JLT" | "CALL" => {
                        self.current_address += 2
                    }
                    _ => (),
                }
            }
        }

        let mut program = Vec::new();

        for line in code.lines() {
            // Handling comments
            let fixed_line = line.split_once(';').map_or(line, |(code, _)| code);

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
                    let (reg, value) = self.parse_reg_and_u16(&data)?;
                    program.push(Instruction::Set(reg, value));
                }
                "LOAD" => {
                    let (reg, address) = self.parse_reg_and_u16(&data)?;
                    program.push(Instruction::Load(reg, address));
                }
                "STORE" => {
                    let (reg, address) = self.parse_reg_and_u16(&data)?;
                    program.push(Instruction::Store(reg, address));
                }
                "ADD" => {
                    let (reg1, reg2) = self.parse_two_regs(&data)?;
                    program.push(Instruction::Add(reg1, reg2));
                }
                "SUB" => {
                    let (reg1, reg2) = self.parse_two_regs(&data)?;
                    program.push(Instruction::Sub(reg1, reg2));
                }
                "JMP" => {
                    let address = self.parse_single_u16(&data)?;
                    program.push(Instruction::Jmp(address));
                }
                "JEQ" => {
                    let address = self.parse_single_u16(&data)?;
                    program.push(Instruction::Jeq(address));
                }
                "JNE" => {
                    let address = self.parse_single_u16(&data)?;
                    program.push(Instruction::Jne(address));
                }
                "JGT" => {
                    let address = self.parse_single_u16(&data)?;
                    program.push(Instruction::Jgt(address));
                }
                "JLT" => {
                    let address = self.parse_single_u16(&data)?;
                    program.push(Instruction::Jlt(address));
                }
                "PUSH" => {
                    let reg = self.parse_single_reg(&data)?;
                    program.push(Instruction::Push(reg));
                }
                "POP" => {
                    let reg = self.parse_single_reg(&data)?;
                    program.push(Instruction::Pop(reg));
                }
                "CALL" => {
                    let address = self.parse_single_u16(&data)?;
                    program.push(Instruction::Call(address));
                }
                "RET" => program.push(Instruction::Ret),
                "AND" => {
                    let (reg1, reg2) = self.parse_two_regs(&data)?;
                    program.push(Instruction::And(reg1, reg2));
                }
                "OR" => {
                    let (reg1, reg2) = self.parse_two_regs(&data)?;
                    program.push(Instruction::Or(reg1, reg2));
                }
                "XOR" => {
                    let (reg1, reg2) = self.parse_two_regs(&data)?;
                    program.push(Instruction::Xor(reg1, reg2));
                }
                "NOT" => {
                    let reg = self.parse_single_reg(&data)?;
                    program.push(Instruction::Not(reg));
                }
                instruction => {
                    return Err(AssemblerError::UnknownInstruction(instruction.to_string()));
                }
            }
        }

        Ok(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u16() {
        assert_eq!(Parser::parse_u16("1024").unwrap(), 1024);
        assert_eq!(Parser::parse_u16("0x0A").unwrap(), 10);
        assert!(matches!(
            Parser::parse_u16("hello!"),
            Err(AssemblerError::ParseIntError(_)),
        ));
    }

    #[test]
    fn test_parse_comment() {
        assert_eq!(
            Parser::new()
                .parse("ADD R0 R1 ; don't touch this!")
                .unwrap(),
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
        let program = Parser::new().parse(code).unwrap();
        assert_eq!(program[2], Instruction::Jne(2));
    }

    #[test]
    fn test_parse_missing_argument() {
        let code = "SET R0";

        assert!(matches!(
            Parser::new().parse(code),
            Err(AssemblerError::MissingArgument(message)) if message == "Expected value"
        ));
    }

    #[test]
    fn test_parse_invalid_register() {
        let code = "ADD R9 R1";

        assert!(matches!(
            Parser::new().parse(code),
            Err(AssemblerError::InvalidRegister(_))
        ));
    }
}
