use crate::error::RuntimeError;

pub struct Memory {
    data: Vec<u16>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn read(&self, address: u16) -> Result<u16, RuntimeError> {
        if address as usize >= self.data.len() {
            return Err(RuntimeError::MemoryOutOfBounds(address as usize));
        }

        Ok(self.data[address as usize])
    }

    pub fn write(&mut self, address: u16, value: u16) -> Result<(), RuntimeError> {
        if address as usize >= self.data.len() {
            return Err(RuntimeError::MemoryOutOfBounds(address as usize));
        }

        self.data[address as usize] = value;
        Ok(())
    }

    pub fn load_program(&mut self, bytecode: &[u16], offset: usize) -> Result<(), RuntimeError> {
        if offset >= self.data.len() {
            return Err(RuntimeError::MemoryOutOfBounds(offset));
        }

        let end = offset + bytecode.len();
        if end > self.data.len() {
            return Err(RuntimeError::MemoryOutOfBounds(end));
        }

        self.data[offset..end].copy_from_slice(bytecode);

        Ok(())
    }
}
