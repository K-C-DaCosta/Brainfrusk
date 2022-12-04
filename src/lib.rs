use std::io::{Read, Write};
mod instruction;
use instruction::*; 

pub struct BrainFrusk<'inst, 'mem> {
    memory_buffer: &'mem mut [u8],
    instruction_buffer: &'inst mut [Instruction],
    instruction_ptr: usize,
    data_ptr: usize,
}

impl<'inst, 'mem> BrainFrusk<'inst, 'mem> {
    pub fn new() -> Self {
        Self {
            memory_buffer: &mut [],
            instruction_buffer: &mut [],
            instruction_ptr: 0,
            data_ptr: 0,
        }
    }

    pub fn with_memory<Buf>(mut self, buffer: &'mem mut [u8]) -> Self {
        self.memory_buffer = buffer;
        self
    }

    pub fn with_instruction_buffer(mut self, buffer: &'inst mut [Instruction]) -> Self {
        self.instruction_buffer = buffer;
        self
    }

    pub fn data(&self)->u8{
        unsafe{
            *self.memory_buffer.get_unchecked(self.data_ptr)
        }
    }
}

