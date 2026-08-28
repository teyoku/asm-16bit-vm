#[derive(Debug, PartialEq)]
pub enum AssemblerError {
    UnknownInstruction(String),
    InvalidRegister(String),
    ParseIntError(String),
    MissingArgument(String),
    UnknownLabel(String),
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblerError::UnknownInstruction(instruction) => {
                write!(f, "Unknown instruction: {instruction}")
            }
            AssemblerError::InvalidRegister(register) => {
                write!(f, "Invalid register: {register}")
            }
            AssemblerError::ParseIntError(value) => {
                write!(f, "Failed to parse integer: {value}")
            }
            AssemblerError::MissingArgument(argument) => {
                write!(f, "Missing argument: {argument}")
            }
            AssemblerError::UnknownLabel(label) => {
                write!(f, "Unknown label: {label}")
            }
        }
    }
}

impl std::error::Error for AssemblerError {}

#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    StackOverflow,
    StackUnderflow,
    MemoryOutOfBounds(usize),
    InvalidOpcode(u16),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::StackOverflow => write!(f, "Stack overflow"),
            RuntimeError::StackUnderflow => write!(f, "Stack underflow"),
            RuntimeError::MemoryOutOfBounds(address) => {
                write!(f, "Memory out of bounds: {address}")
            }
            RuntimeError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {opcode}"),
        }
    }
}
impl std::error::Error for RuntimeError {}
