use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    NOP,
    QuickIncrementDataPtr(usize),
    QuickDecrementDataPtr(usize),
    QuickIncrementByte(usize),
    QuickDecrementByte(usize),
    IncrementDataPtr,
    DecrementDataPtr,
    IncrementByte,
    DecrementByte,
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
    pub fn execute<'a, 'b, OUT: Write>(self, state: &mut Interpreter<'a, 'b>, mut stdout: OUT) {
        match self {
            Self::IncrementDataPtr => {
                state.data_ptr += 1;
            }

            Self::QuickIncrementDataPtr(ofx) => {
                state.data_ptr += ofx;
            }

            Self::DecrementDataPtr => {
                state.data_ptr -= 1;
            }

            Self::QuickDecrementDataPtr(ofx) => {
                state.data_ptr -= ofx;
            }

            Self::IncrementByte => {
                state.memory_buffer[state.data_ptr] += 1;
            }

            Self::QuickIncrementByte(ofx) => {
                state.memory_buffer[state.data_ptr] += ofx as u8;
            }

            Self::DecrementByte => {
                state.memory_buffer[state.data_ptr] -= 1;
            }

            Self::QuickDecrementByte(ofx) => {
                state.memory_buffer[state.data_ptr] -= ofx as u8;
            }

            Self::OutputByte => {
                let output = &[state.memory_buffer[state.data_ptr]][..];
                stdout
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

        let source = Self::strip_source_of_whitespace_and_comments(source);
        // println!("stripped-source:\"{}\"", source);
        let mut output = vec![Instruction::NOP; source.len()];
        let unoptimized_instructions = output.len();
        Self::tokenize_string(&source, &mut output);

        Self::optimize_instructions(&mut output);
        let optimized_intructions = output.len();

        Self::reindex_branches(&mut output);
        Self::compute_bracket_indexes(&mut output);

        println!(
            "unoptimized = '{}', optimized = '{}'",
            unoptimized_instructions, optimized_intructions
        );
        output
    }
    /// uses statemachine logic to
    fn optimize_instructions(unoptimized_code: &mut Vec<Instruction>) {
        let mut optimized_instructions = vec![];
        let mut accum_counter = 0;

        //to solve the last token problem
        unoptimized_code.push(Instruction::NOP);
        let mut current_instruction = unoptimized_code[0];

        let push_unoptimizable_inst = |inst, optimized_instructions: &mut Vec<_>| {
            if let Instruction::InputByte
            | Instruction::OutputByte
            | Instruction::LoopClose { .. }
            | Instruction::LoopOpen { .. } = inst
            {
                optimized_instructions.push(inst);
            }
        };

        for &mut inst in unoptimized_code.iter_mut() {
            match current_instruction {
                Instruction::IncrementByte => match inst {
                    Instruction::IncrementByte => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions.push(Instruction::QuickIncrementByte(accum_counter));
                        current_instruction = inst;
                        accum_counter = 1;
                        push_unoptimizable_inst(inst, &mut optimized_instructions);
                    }
                },
                Instruction::DecrementByte => match inst {
                    Instruction::DecrementByte => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions.push(Instruction::QuickDecrementByte(accum_counter));
                        current_instruction = inst;
                        accum_counter = 1;
                        push_unoptimizable_inst(inst, &mut optimized_instructions);
                    }
                },
                Instruction::IncrementDataPtr => match inst {
                    Instruction::IncrementDataPtr => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions
                            .push(Instruction::QuickIncrementDataPtr(accum_counter));
                        current_instruction = inst;
                        accum_counter = 1;

                        push_unoptimizable_inst(inst, &mut optimized_instructions);
                    }
                },
                Instruction::DecrementDataPtr => match inst {
                    Instruction::DecrementDataPtr => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions
                            .push(Instruction::QuickDecrementDataPtr(accum_counter));
                        current_instruction = inst;
                        accum_counter = 1;
                        push_unoptimizable_inst(inst, &mut optimized_instructions);
                    }
                },
                _ => {
                    accum_counter = 1;
                    current_instruction = inst;
                    push_unoptimizable_inst(inst, &mut optimized_instructions);
                }
            }
        }
        //transfer optimized code to unoptimized buffer
        unoptimized_code.clear();
        for inst in optimized_instructions {
            unoptimized_code.push(inst);
        }
    }

    fn reindex_branches(code: &mut Vec<Instruction>) {
        for (idx, inst) in code.iter_mut().enumerate() {
            match inst {
                Instruction::LoopClose { open_location } => {
                    *open_location = idx;
                }
                Instruction::LoopOpen { close_location } => {
                    *close_location = idx;
                }
                _ => (),
            }
        }
    }

    fn strip_source_of_whitespace_and_comments(source: &str) -> String {
        source
            .lines()
            .flat_map(|line| {
                line.chars()
                    .filter(|c| !c.is_whitespace())
                    .take_while(|&c| c != '#')
            })
            .collect::<String>()
    }

    fn tokenize_string(source: &str, output: &mut [Instruction]) {
        for (idx, (c, inst)) in source.chars().zip(output.iter_mut()).enumerate() {
            *inst = match c {
                '>' => Self::IncrementDataPtr,
                '<' => Self::DecrementDataPtr,
                '+' => Self::IncrementByte,
                '-' => Self::DecrementByte,
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

    fn compute_bracket_indexes(output: &mut [Instruction]) {
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
                    if let Self::LoopOpen { close_location } = matching_open_token {
                        *close_location = cached_close_location;
                    }
                }
                _ => (),
            });
    }
}
