#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3
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
            _ => None
        }
    }
}