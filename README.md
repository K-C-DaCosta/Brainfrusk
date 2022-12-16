# Brainfrusk
A brainfuck interpreter(and maybe compiler eventually) written in Rust 

## Example (prints hello world )
```rust
    let source = r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut tokens = brainfrusk::Instruction::parse_str(source);
    let mut memory = vec![0u8; 1024];
    brainfrusk::Interpreter::new()
        .with_instruction_buffer(&mut tokens)
        .with_memory(&mut memory)
        .run();
```


## Tests
For now, you can run the most computationally expensive test(mandlebrot plotter) like this:
```
cargo test -q mandlebrot  --release -- --nocapture
```