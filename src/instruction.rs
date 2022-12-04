use super::*;

#[derive(Copy, Clone)]
pub enum Instruction {
    NOP,
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
    pub fn bracket_location(self) -> Option<usize> {
        let loc = match self {
            Self::LoopOpen { close_location } => close_location,
            Self::LoopClose { open_location } => open_location,
            _ => return None,
        };
        Some(loc)
    }
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

            Self::LoopOpen { close_location } => {
                if state.data() == 0 {
                    state.instruction_ptr = close_location;
                }
            }
            Self::LoopClose { open_location } => {
                if state.data() != 0 {
                    state.instruction_ptr = open_location - 1;
                }
            }
            Self::NOP => { /* Do absolutely nothing */ }
        }
        //finally increment program counter
        state.instruction_ptr += 1;
    }

    /// # Description
    /// parses brainfuck source into tokens used by the runtime (or compiler if I get there)
    pub fn parse_str(source: &str) -> Vec<Instruction> {
        let mut output = vec![Instruction::NOP; source.len()];
        Self::tokenize_string(source, &mut output);
        Self::compute_bracket_indexes(&mut output);
        output
    }
    fn tokenize_string(source:&str,output:&mut [Instruction]){
        for (idx, (c, inst)) in source.chars().zip(output.iter_mut()).enumerate() {
            *inst = match c {
                '>' => Self::IncrementBytePtr,
                '<' => Self::DecrementBytePtr,
                '+' => Self::IncrementDataPtr,
                '-' => Self::DecrementDataPtr,
                ',' => Self::InputByte,
                '.' => Self::OutputByte,
                //store instruction location for bracket tokens
                '[' => Self::LoopOpen {
                    close_location: idx,
                },
                ']' => Self::LoopClose { open_location: idx },
                _ => Self::NOP,
            };
        }
    }
    fn compute_bracket_indexes(output:&mut [Instruction]){
        //basically i use a bracket stack to detect matching brackets
        //when matching brackets are detected I update index info
        let mut bracket_stack = Vec::new();
        output
            .iter_mut()
            .filter(|inst| {
                if let Self::LoopOpen { .. } | Self::LoopClose { .. } = inst {
                    true
                } else {
                    false
                }
            })
            .for_each(|inst| match inst {
                Self::LoopOpen { .. } => {
                    bracket_stack.push(inst);
                }
                Self::LoopClose { open_location } => {
                    let matching_open_token = bracket_stack.pop().expect("mismatching brackets");
                
                    //save instruction location for close location
                    let cached_close_location = *open_location;
                    
                    //write open location to close location
                    *open_location = matching_open_token
                        .bracket_location()
                        .expect("matching open token should be a bracket");
                    
                    //write open location to close location tag
                    if let Self::LoopOpen { close_location } = matching_open_token{
                        *close_location = cached_close_location;
                    }
                }
                _ => (),
            });

    }
}
