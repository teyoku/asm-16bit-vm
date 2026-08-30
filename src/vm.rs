use crate::{
    error::RuntimeError,
    instruction::{Instruction, Opcode},
    memory::Memory,
    register::Register,
};

pub struct VirtualMachine {
    registers: [u16; 4],
    pub memory: Memory,
    pc: u16, // program counter
    sp: u16, // stack pointer
    zero_flag: bool,
    is_running: bool,
}

impl VirtualMachine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0, 0, 0, 0],
            memory: Memory::new(memory_size),
            pc: 0,
            sp: memory_size as u16,
            zero_flag: false,
            is_running: true,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), RuntimeError> {
        match instruction {
            Instruction::Halt => self.is_running = false,
            Instruction::Set(register, value) => {
                self.registers[register.as_index()] = value;
            }
            Instruction::Load(register, address) => {
                let value = self.memory.read(address)?;
                self.registers[register.as_index()] = value;
            }
            Instruction::Store(register, address) => {
                let value = self.registers[register.as_index()];
                self.memory.write(address, value)?;
            }
            Instruction::Add(register1, register2) => {
                let value = self.registers[register1.as_index()]
                    .wrapping_add(self.registers[register2.as_index()]);
                self.registers[register1.as_index()] = value;
                self.zero_flag = value == 0;
            }
            Instruction::Sub(register1, register2) => {
                let value = self.registers[register1.as_index()]
                    .wrapping_sub(self.registers[register2.as_index()]);
                self.registers[register1.as_index()] = value;
                self.zero_flag = value == 0;
            }
            Instruction::Jmp(address) => self.pc = address,
            Instruction::Jeq(address) => {
                if self.zero_flag {
                    self.pc = address
                }
            }
            Instruction::Jne(address) => {
                if !self.zero_flag {
                    self.pc = address
                }
            }
            Instruction::Push(register) => {
                if self.sp == 0 {
                    return Err(RuntimeError::StackOverflow);
                }

                self.sp -= 1;
                let value = self.registers[register.as_index()];
                self.memory.write(self.sp, value)?;
            }
            Instruction::Pop(register) => {
                self.registers[register.as_index()] = self.memory.read(self.sp)?;
                self.sp += 1;
            }
            Instruction::Call(address) => {
                if self.sp == 0 {
                    return Err(RuntimeError::StackOverflow);
                }

                self.sp -= 1;
                self.memory.write(self.sp, self.pc)?;
                self.pc = address;
            }
            Instruction::Ret => {
                let ret_addr = self.memory.read(self.sp)?;
                self.pc = ret_addr;
                self.sp += 1;
            }
            Instruction::And(register1, register2) => {
                let value =
                    self.registers[register1.as_index()] & self.registers[register2.as_index()];
                self.registers[register1.as_index()] = value;
                self.zero_flag = value == 0
            }
            Instruction::Or(register1, register2) => {
                let value =
                    self.registers[register1.as_index()] | self.registers[register2.as_index()];
                self.registers[register1.as_index()] = value;
                self.zero_flag = value == 0
            }
            Instruction::Xor(register1, register2) => {
                let value =
                    self.registers[register1.as_index()] ^ self.registers[register2.as_index()];
                self.registers[register1.as_index()] = value;
                self.zero_flag = value == 0
            }
            Instruction::Not(register) => {
                let value = !self.registers[register.as_index()];
                self.registers[register.as_index()] = value;
                self.zero_flag = value == 0
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while self.is_running {
            let instruction = self.fetch_and_decode()?;
            self.execute_instruction(instruction)?;
        }
        Ok(())
    }

    fn next_word(&mut self) -> Result<u16, RuntimeError> {
        let word = self.memory.read(self.pc)?;
        self.pc += 1;
        Ok(word)
    }

    fn fetch_and_decode(&mut self) -> Result<Instruction, RuntimeError> {
        let instruction_word = self.next_word()?;
        let opcode_value = (instruction_word >> 12) & 0x000F;

        let opcode: Opcode = opcode_value.try_into()?;

        let extract_reg = |shift| {
            Register::from_u16((instruction_word >> shift) & 0x0003u16)
                .ok_or(RuntimeError::InvalidOpcode(instruction_word))
        };

        match opcode {
            Opcode::Halt => Ok(Instruction::Halt),
            Opcode::Set => Ok(Instruction::Set(extract_reg(10)?, self.next_word()?)),
            Opcode::Load => Ok(Instruction::Load(extract_reg(10)?, self.next_word()?)),
            Opcode::Store => Ok(Instruction::Store(extract_reg(10)?, self.next_word()?)),
            Opcode::Add => Ok(Instruction::Add(extract_reg(10)?, extract_reg(8)?)),
            Opcode::Sub => Ok(Instruction::Sub(extract_reg(10)?, extract_reg(8)?)),
            Opcode::Jmp => Ok(Instruction::Jmp(self.next_word()?)),
            Opcode::Jeq => Ok(Instruction::Jeq(self.next_word()?)),
            Opcode::Jne => Ok(Instruction::Jne(self.next_word()?)),
            Opcode::Push => Ok(Instruction::Push(extract_reg(10)?)),
            Opcode::Pop => Ok(Instruction::Pop(extract_reg(10)?)),
            Opcode::Call => Ok(Instruction::Call(self.next_word()?)),
            Opcode::Ret => Ok(Instruction::Ret),
            Opcode::And => Ok(Instruction::And(extract_reg(10)?, extract_reg(8)?)),
            Opcode::Or => Ok(Instruction::Or(extract_reg(10)?, extract_reg(8)?)),
            Opcode::Xor => Ok(Instruction::Xor(extract_reg(10)?, extract_reg(8)?)),
            Opcode::Not => Ok(Instruction::Not(extract_reg(10)?)),
        }
    }

    pub fn print_state(&self) {
        println!("Registers: {:?}", self.registers);
        println!("PC: {}, SP: {}", self.pc, self.sp);
        println!("Zero Flag: {}", self.zero_flag);
    }
}

#[cfg(test)]
mod tests {
    use crate::{assembler::assemble, error::AssemblerError, parser::parse};

    use super::*;

    #[test]
    fn test_halt_instruction() {
        let mut vm = VirtualMachine::new(1024);
        vm.execute_instruction(Instruction::Halt).unwrap();
        assert!(!vm.is_running);
    }

    #[test]
    fn test_set_instruction() {
        let mut vm = VirtualMachine::new(1024);

        // inserting 12 into 'R0' register
        vm.execute_instruction(Instruction::Set(Register::R0, 12))
            .unwrap();

        assert_eq!(vm.registers[0], 12);
    }

    #[test]
    fn test_load_instruction() {
        let mut vm = VirtualMachine::new(1024);
        // inserting 1234 into memory index '200'
        vm.memory.write(200, 1234).unwrap();
        // inserting 1234 from memory index '200' to 'R2' register
        vm.execute_instruction(Instruction::Load(Register::R2, 200))
            .unwrap();

        assert_eq!(vm.registers[2], 1234);
    }

    #[test]
    fn test_store_instruction() {
        let mut vm = VirtualMachine::new(1024);
        // inserting 2574 into 'R1' register
        vm.execute_instruction(Instruction::Set(Register::R1, 2574))
            .unwrap();
        // inserting value from 'R1' register (2574) to memory index '12'
        vm.execute_instruction(Instruction::Store(Register::R1, 12))
            .unwrap();

        assert_eq!(vm.memory.read(12).unwrap(), 2574);
    }

    #[test]
    fn test_add_instruction() {
        let mut vm = VirtualMachine::new(1024);
        // inserting 10 into 'R0' register
        vm.execute_instruction(Instruction::Set(Register::R0, 10))
            .unwrap();
        // inserting 15 into 'R1' register
        vm.execute_instruction(Instruction::Set(Register::R1, 15))
            .unwrap();

        // add 10 (R0) and 15 (R1) and insert result to 'R0' register
        vm.execute_instruction(Instruction::Add(Register::R0, Register::R1))
            .unwrap();

        assert_eq!(vm.registers[0], 25);
        assert_eq!(vm.registers[1], 15); // old value is still here
    }

    #[test]
    fn test_sub_instruction() {
        let mut vm = VirtualMachine::new(1024);
        // inserting 25 into 'R1' register
        vm.execute_instruction(Instruction::Set(Register::R1, 25))
            .unwrap();
        // inserting 11 into 'R3' register
        vm.execute_instruction(Instruction::Set(Register::R3, 11))
            .unwrap();

        // sub 25 (R1) and 11 (R3) and insert result to 'R1' register
        vm.execute_instruction(Instruction::Sub(Register::R1, Register::R3))
            .unwrap();

        assert_eq!(vm.registers[1], 14);
        assert_eq!(vm.registers[3], 11); // old value is still here
    }

    #[test]
    fn test_zero_flag() {
        let mut vm = VirtualMachine::new(1024);
        // inserting 5 into 'R0' register
        vm.execute_instruction(Instruction::Set(Register::R1, 5))
            .unwrap();
        // inserting 5 into 'R1' register
        vm.execute_instruction(Instruction::Set(Register::R2, 5))
            .unwrap();

        // sub 5 (R1) and 5 (R2) and insert result to 'R1' register
        vm.execute_instruction(Instruction::Sub(Register::R1, Register::R2))
            .unwrap();

        assert_eq!(vm.registers[1], 0);
        assert_eq!(vm.registers[2], 5); // old value is still here
        assert!(vm.zero_flag);
    }

    #[test]
    fn test_jmp_instruction() {
        let mut vm = VirtualMachine::new(1024);
        vm.execute_instruction(Instruction::Jmp(594)).unwrap();

        assert_eq!(vm.pc, 594);
    }

    #[test]
    fn test_jeq_instruction() {
        let mut vm = VirtualMachine::new(1024);

        vm.zero_flag = true;

        // zero flag is true, and we try Jeq to addr '12'
        vm.execute_instruction(Instruction::Jeq(12)).unwrap();
        assert_eq!(vm.pc, 12);

        // zero flag is true, but we try Jne to addr '104'
        vm.execute_instruction(Instruction::Jne(104)).unwrap();
        assert_ne!(vm.pc, 104);
    }

    #[test]
    fn test_jne_instruction() {
        let mut vm = VirtualMachine::new(1024);

        // zero flag is false, and we try Jne to addr '294'
        vm.execute_instruction(Instruction::Jne(294)).unwrap();
        assert_eq!(vm.pc, 294);

        // zero flag is false, and we try Jeq to addr '999'
        vm.execute_instruction(Instruction::Jeq(999)).unwrap();
        assert_ne!(vm.pc, 999);
    }

    #[test]
    fn test_stack() {
        let mut vm = VirtualMachine::new(1024);

        vm.execute_instruction(Instruction::Set(Register::R0, 99))
            .unwrap();
        vm.execute_instruction(Instruction::Push(Register::R0))
            .unwrap();
        vm.execute_instruction(Instruction::Set(Register::R0, 0))
            .unwrap();
        vm.execute_instruction(Instruction::Pop(Register::R0))
            .unwrap();

        assert_eq!(vm.registers[0], 99);
    }

    #[test]
    fn test_call_ret() {
        let mut vm = VirtualMachine::new(1024);

        let bytecode: Vec<u16> = vec![0xB000, 0x0004, 0x0000, 0x0000, 0x1000, 0x002A, 0xC000];

        vm.memory.load_program(&bytecode, 0).unwrap();
        vm.run().unwrap();

        assert!(!vm.is_running);
        assert_eq!(vm.registers[0], 42);
    }

    #[test]
    fn test_parse_valid_program() -> Result<(), AssemblerError> {
        let code = "
            SET R0 5
            HALT
        ";
        let program = parse(code)?;
        let bytecode = assemble(&program);

        let mut vm = VirtualMachine::new(1024);
        vm.memory.load_program(&bytecode, 0).unwrap();
        vm.run().unwrap();

        assert_eq!(vm.registers[0], 5);

        Ok(())
    }

    #[test]
    fn test_parse_missing_argument() {
        let code = "SET R0";

        assert!(matches!(
            parse(code),
            Err(AssemblerError::MissingArgument(message)) if message == "Expected value"
        ));
    }

    #[test]
    fn test_parse_invalid_register() {
        let code = "ADD R9 R1";

        assert!(matches!(
            parse(code),
            Err(AssemblerError::InvalidRegister(_))
        ));
    }

    #[test]
    fn test_bitwise_operations() {
        let mut vm = VirtualMachine::new(1024);

        // R1 = 0b1100 (12), R2 = 0b1010 (10)
        vm.execute_instruction(Instruction::Set(Register::R1, 12))
            .unwrap();
        vm.execute_instruction(Instruction::Set(Register::R2, 10))
            .unwrap();

        // R1 = 12 & 10 = 0b1000 (8)
        vm.execute_instruction(Instruction::And(Register::R1, Register::R2))
            .unwrap();
        assert_eq!(vm.registers[1], 8);

        // NOT 8 (0000_0000_0000_1000 -> 1111_1111_1111_0111 (65527))
        vm.execute_instruction(Instruction::Not(Register::R1))
            .unwrap();
        assert_eq!(vm.registers[1], 65527);
    }
}
