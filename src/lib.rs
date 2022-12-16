use std::{
    io::{self, BufWriter, Read, Write},
    time::Instant,
};
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
        let mut stdout = BufWriter::new(io::stdout());
        let mut t0 = Instant::now();
        while self.instruction_pointer_in_bounds() {
            self.current_instruction().execute(self, &mut stdout);
            if t0.elapsed().as_millis() > 1000 {
                t0 = Instant::now();
                stdout.flush().unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}

#[test]
fn simple_optimization_test() {
    let source = "++[--]++";
    let tokens = Instruction::parse_str(source);
    println!("{:?}", tokens);
}

#[test]
fn two_plus_five() {
    let source = r"
    ++       #Cell c0 = 2
    > +++++  #Cell c1 = 5
    [        #Start your loops with your cell pointer on the loop counter (c1 in our case)
    < +      #Add 1 to c0
    > -      #Subtract 1 from c1
    ]        #End your loops with the cell pointer on the loop counter

    ++++ ++++  #c1 = 8 and this will be our loop counter again
    [
    < +++ +++  #Add 6 to c0
    > -        #Subtract 1 from c1
    ]
    < .        #Print out c0 which has the value 55 which translates to 
    ";
    let mut tokens = Instruction::parse_str(source);
    let mut memory = vec![0u8; 32];
    Interpreter::new()
        .with_instruction_buffer(&mut tokens)
        .with_memory(&mut memory)
        .run();
}

#[test]
fn hello_world() {
    let source = r"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut tokens = Instruction::parse_str(source);
    // println!("optimized code = {:?}", tokens);

    let mut memory = vec![0u8; 1024];
    Interpreter::new()
        .with_instruction_buffer(&mut tokens)
        .with_memory(&mut memory)
        .run();
}


