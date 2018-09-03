use instruction::Instruction;
use parser::InstructionList;
use std::fmt;
use std::io::ErrorKind;
use std::io::{Read, Write};

const MEM_SIZE: usize = 256;
const BUFFER_SIZE: usize = 256;

pub struct Interpreter {
    instructions: InstructionList,
    pc: usize,
    ap: isize,     // logical address pointer, will have negatives
    max_ap: isize, // actual max address pointer reached, can compare
    memory: [u8; MEM_SIZE],
    input: Box<Read>,
    output: Box<Write>,
    output_buffer: Vec<u8>,
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
            output_buffer: Vec::with_capacity(BUFFER_SIZE),
        }
    }

    fn concrete_address(&self) -> Result<usize, &'static str> {
        if self.ap < 0 {
            let as_positive = self.ap.wrapping_neg() as usize;
            if as_positive >= MEM_SIZE || as_positive >= (self.max_ap as usize) {
                Err("Out of memory")
            } else {
                Ok(MEM_SIZE - as_positive)
            }
        } else {
            // 'safe' cast as ap is positive
            if (self.ap as usize) >= MEM_SIZE {
                Err("Out of memory")
            } else {
                Ok(self.ap as usize)
            }
        }
    }

    fn write(&mut self, byte: u8) -> () {
        // don't re-allocate
        if self.output_buffer.len() == (BUFFER_SIZE - 1) {
            self.write_out_buffer();
        }
        self.output_buffer.push(byte);
    }

    fn write_out_buffer(&mut self) -> () {
        if self.output_buffer.len() > 0 {
            self.output.write(&self.output_buffer[..]).unwrap();
            self.output.flush().unwrap();
            self.output_buffer.clear();
        }
        ()
    }

    fn read_one(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        let read_byte = match self.input.read(&mut buf) {
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => Ok(0),
            Ok(_) => Ok(buf[0]),
            Err(e) => Err(e),
        };
        read_byte.unwrap()
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        while self.pc < self.instructions.as_vec().len() {
            let current_symbol = self.instructions.as_vec()[self.pc];
            match current_symbol {
                // delay throwing OOM until OOM access occurs
                Instruction::MoveRight => {
                    self.ap += 1;
                    self.max_ap = if self.ap > self.max_ap {
                        self.max_ap + 1
                    } else {
                        self.max_ap
                    };
                    self.pc += 1;
                }
                Instruction::MoveLeft => {
                    self.ap -= 1;
                    self.pc += 1;
                }
                Instruction::Increment => {
                    let address = self.concrete_address()?;
                    self.memory[address] = self.memory[address].wrapping_add(1);
                    self.pc += 1;
                }
                Instruction::Decrement => {
                    let address = self.concrete_address()?;
                    self.memory[address] = self.memory[address].wrapping_sub(1);
                    self.pc += 1;
                }
                Instruction::Write => {
                    let address = self.concrete_address()?;
                    let byte = self.memory[address];
                    self.write(byte);
                    self.pc += 1;
                }
                Instruction::Read => {
                    let address = self.concrete_address()?;
                    let read = self.read_one();
                    self.memory[address] = read;
                    self.pc += 1;
                }
                Instruction::JumpIfZero => {
                    let address = self.concrete_address()?;
                    let byte = self.memory[address];
                    self.pc = if byte == 0 { self.pc + 1 } else { self.pc + 1 }
                }
                Instruction::JumpUnlessZero => {
                    self.pc += 1;
                }
            }
        }
        self.write_out_buffer();
        Ok(())
    }
}
