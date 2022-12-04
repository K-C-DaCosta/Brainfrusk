use std::io::{Read, Write};

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

#[derive(Copy, Clone)]
pub enum Instruction {
    IncrementDataPtr,
    DecrementDataPtr,
    IncrementBytePtr,
    DecrementBytePtr,
    OutputByte,
    InputByte,
    LoopOpen { close_location: usize },
    LoopClose { open_location: usize },
}

impl Instruction {
    pub fn execute<'a, 'b>(self, state: &mut BrainFrusk<'a, 'b>) {
        match self {
            Self::IncrementDataPtr => {
                state.data_ptr += 1;
            }
            
            Self::DecrementDataPtr => {
                state.data_ptr -= 1;
            }
            
            Self::IncrementBytePtr => {
                state.memory_buffer[state.data_ptr] += 1;
            }
            
            Self::DecrementBytePtr => {
                state.memory_buffer[state.data_ptr] -= 1;
            }

            Self::OutputByte => {
                let output = &[state.memory_buffer[state.data_ptr]][..];
                std::io::stdout()
                    .write_all(output)
                    .expect("failed to read from std_out");
            }

            Self::InputByte => {
                let mut input_byte = [0u8];
                std::io::stdin()
                    .read_exact(&mut input_byte)
                    .expect("failed to read from std_in");
                state.memory_buffer[state.data_ptr] = input_byte[0];
            }

        
            Self::LoopOpen { close_location } =>{
                if state.data() == 0 {
                    state.instruction_ptr = close_location-1;
                }
            }
            Self::LoopClose { open_location } => {
                if state.data() != 0{
                    state.instruction_ptr = open_location-1;
                }
            }
        }
    }
}
