use parser::InstructionList;
use std::fmt;
use std::io::{Read, Write};

const MEM_SIZE: usize = 256;

pub struct Interpreter {
    instructions: InstructionList,
    pc: u64,
    ap: i64,     // logical address pointer, will have negatives
    max_ap: i64, // actual max address pointer reached, can compare
    memory: [u8; MEM_SIZE],
    input: Box<Read>,
    output: Box<Write>,
}

impl fmt::Debug for Interpreter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Interpeter: {{ instructions: {}, pc: {}, ap: {}, max_ap: {}, memory: {} }}",
            format!("{:?}", self.instructions),
            self.pc,
            self.ap,
            self.max_ap,
            format!("{:?}", self.memory.to_vec())
        )
    }
}

impl Interpreter {
    pub fn new(ins: InstructionList, input: Box<Read>, output: Box<Write>) -> Interpreter {
        Interpreter {
            pc: 0,
            ap: 0,
            max_ap: 0,
            memory: [0; MEM_SIZE],
            instructions: ins,
            input: input,
            output: output,
        }
    }
}
