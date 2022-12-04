#[derive(Copy, Clone)]
pub enum Instruction {
    IncrementDataPtr,

}

pub struct BrainFrusk<'inst, 'mem> {
    memory_buffer: &'mem [u8],
    instruction_buffer: &'inst [Instruction],
    instruction_ptr: usize,
    data_ptr:usize,
}

impl<'inst, 'mem> BrainFrusk<'inst, 'mem> {
    pub fn new() -> Self {
        Self {
            memory_buffer: &[],
            instruction_buffer: &[],
            instruction_ptr: 0,
            data_ptr:0, 
        }
    }

    pub fn with_memory<Buf>(mut self, buffer: &'mem [u8]) -> Self {
        self.memory_buffer = buffer;
        self
    }

    pub fn with_instruction_buffer(mut self, buffer: &'inst [Instruction]) -> Self {
        self.instruction_buffer = buffer;
        self
    }
}
