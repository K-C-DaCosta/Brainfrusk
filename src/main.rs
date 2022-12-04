use brainfrusk::*;

fn main() {
    let source = "++>+++++[<+>-]";
    let mut tokens = brainfrusk::Instruction::parse_str(source);
    let mut memory = vec![0u8; 32];
    println!("tokens:\n{:?}", tokens);

    Interpreter::new()
        .with_instruction_buffer(&mut tokens)
        .with_memory(&mut memory)
        .run();


    println!("memory dump:\n{:?}",memory);
}
