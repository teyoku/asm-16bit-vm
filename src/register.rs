use crate::error::AssemblerError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
}

impl Register {
    pub fn as_index(&self) -> usize {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
        }
    }

    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0 => Some(Register::R0),
            1 => Some(Register::R1),
            2 => Some(Register::R2),
            3 => Some(Register::R3),
            _ => None,
        }
    }
}

impl TryFrom<&str> for Register {
    type Error = AssemblerError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "R0" => Ok(Register::R0),
            "R1" => Ok(Register::R1),
            "R2" => Ok(Register::R2),
            "R3" => Ok(Register::R3),
            _ => Err(AssemblerError::InvalidRegister(value.to_string())),
        }
    }
}
