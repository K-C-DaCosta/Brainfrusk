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

## Running Mandlebrot example
```
cargo run --example=mandelbrot --release
```