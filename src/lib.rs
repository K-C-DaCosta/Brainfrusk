use std::io::{Read, Write};
mod instruction;
pub use instruction::*;

pub struct Interpreter<'inst, 'mem> {
    memory_buffer: &'mem mut [u8],
    instruction_buffer: &'inst mut [Instruction],
    instruction_ptr: usize,
    data_ptr: usize,
}

impl<'inst, 'mem> Interpreter<'inst, 'mem> {
    pub fn new() -> Self {
        Self {
            memory_buffer: &mut [],
            instruction_buffer: &mut [],
            instruction_ptr: 0,
            data_ptr: 0,
        }
    }

    pub fn with_memory(mut self, buffer: &'mem mut [u8]) -> Self {
        self.memory_buffer = buffer;
        self
    }

    pub fn with_instruction_buffer(mut self, buffer: &'inst mut [Instruction]) -> Self {
        self.instruction_buffer = buffer;
        self
    }

    fn data(&self) -> u8 {
        unsafe { *self.memory_buffer.get_unchecked(self.data_ptr) }
    }

    fn current_instruction(&self) -> Instruction {
        self.instruction_buffer[self.instruction_ptr]
    }

    fn instruction_pointer_in_bounds(&self) -> bool {
        self.instruction_ptr < self.instruction_buffer.len()
    }

    pub fn run(&mut self) {
        while self.instruction_pointer_in_bounds() {
            self.current_instruction().execute(self);
        }
    }
}
