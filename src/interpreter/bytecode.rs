use super::*; 

#[derive(Copy, Clone, Debug)]
pub enum ByteCode {
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

impl ByteCode {
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

}
