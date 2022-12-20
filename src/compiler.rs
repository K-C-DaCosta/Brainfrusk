use super::*; 

pub struct Compiler;

impl Compiler{
    /// # Description
    /// parses brainfuck source into tokens used by the runtime (or compiler if I get there)
    pub fn compile(source: &str) -> Vec<ByteCode> {

        let source = Self::strip_source_of_whitespace_and_comments(source);
        // println!("stripped-source:\"{}\"", source);
        let mut output = vec![ByteCode::NOP; source.len()];
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

    pub fn compile_unoptimized(source: &str) -> Vec<ByteCode> {
        let source = Self::strip_source_of_whitespace_and_comments(source);
        // println!("stripped-source:\"{}\"", source);
        let mut output = vec![ByteCode::NOP; source.len()];
        Self::tokenize_string(&source, &mut output);
        // Self::reindex_branches(&mut output);
        Self::compute_bracket_indexes(&mut output);
        output
    }


    /// uses statemachine logic to
    fn optimize_instructions(unoptimized_code: &mut Vec<ByteCode>) {
        let mut optimized_instructions = vec![];
        let mut accum_counter = 0;

        //to solve the last token problem
        unoptimized_code.push(ByteCode::NOP);
        let mut current_instruction = unoptimized_code[0];

        let push_unoptimizable_inst = |inst, optimized_instructions: &mut Vec<_>| {
            if let ByteCode::InputByte
            | ByteCode::OutputByte
            | ByteCode::LoopClose { .. }
            | ByteCode::LoopOpen { .. } = inst
            {
                optimized_instructions.push(inst);
            }
        };

        for &mut inst in unoptimized_code.iter_mut() {
            match current_instruction {
                ByteCode::IncrementByte => match inst {
                    ByteCode::IncrementByte => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions.push(ByteCode::QuickIncrementByte(accum_counter));
                        current_instruction = inst;
                        accum_counter = 1;
                        push_unoptimizable_inst(inst, &mut optimized_instructions);
                    }
                },
                ByteCode::DecrementByte => match inst {
                    ByteCode::DecrementByte => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions.push(ByteCode::QuickDecrementByte(accum_counter));
                        current_instruction = inst;
                        accum_counter = 1;
                        push_unoptimizable_inst(inst, &mut optimized_instructions);
                    }
                },
                ByteCode::IncrementDataPtr => match inst {
                    ByteCode::IncrementDataPtr => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions
                            .push(ByteCode::QuickIncrementDataPtr(accum_counter));
                        current_instruction = inst;
                        accum_counter = 1;

                        push_unoptimizable_inst(inst, &mut optimized_instructions);
                    }
                },
                ByteCode::DecrementDataPtr => match inst {
                    ByteCode::DecrementDataPtr => {
                        accum_counter += 1;
                    }
                    _ => {
                        optimized_instructions
                            .push(ByteCode::QuickDecrementDataPtr(accum_counter));
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

    fn reindex_branches(code: &mut Vec<ByteCode>) {
        for (idx, inst) in code.iter_mut().enumerate() {
            match inst {
                ByteCode::LoopClose { open_location } => {
                    *open_location = idx;
                }
                ByteCode::LoopOpen { close_location } => {
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

    fn tokenize_string(source: &str, output: &mut [ByteCode]) {
        for (idx, (c, inst)) in source.chars().zip(output.iter_mut()).enumerate() {
            *inst = match c {
                '>' => ByteCode::IncrementDataPtr,
                '<' => ByteCode::DecrementDataPtr,
                '+' => ByteCode::IncrementByte,
                '-' => ByteCode::DecrementByte,
                ',' => ByteCode::InputByte,
                '.' => ByteCode::OutputByte,
                //store instruction location for bracket tokens
                '[' => ByteCode::LoopOpen {
                    close_location: idx,
                },
                ']' => ByteCode::LoopClose { open_location: idx },
                _ => ByteCode::NOP,
            };
        }
    }

    fn compute_bracket_indexes(output: &mut [ByteCode]) {
        //basically i use a bracket stack to detect matching brackets
        //when matching brackets are detected I update index info
        let mut bracket_stack = Vec::new();
        output
            .iter_mut()
            .filter(|inst| {
                if let ByteCode::LoopOpen { .. } | ByteCode::LoopClose { .. } = inst {
                    true
                } else {
                    false
                }
            })
            .for_each(|inst| match inst {
                ByteCode::LoopOpen { .. } => {
                    bracket_stack.push(inst);
                }
                ByteCode::LoopClose { open_location } => {
                    let matching_open_token = bracket_stack.pop().expect("mismatching brackets");

                    //save instruction location for close location
                    let cached_close_location = *open_location;

                    //write open location to close location
                    *open_location = matching_open_token
                        .bracket_location()
                        .expect("matching open token should be a bracket");

                    //write open location to close location tag
                    if let ByteCode::LoopOpen { close_location } = matching_open_token {
                        *close_location = cached_close_location;
                    }
                }
                _ => (),
            });
    }
}