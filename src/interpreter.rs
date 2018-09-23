use instruction::Instruction;
use interpreter_config::*;
use std::fmt;
use std::io::ErrorKind;
use std::io::{Read, Write};

const MEM_SIZE: usize = 256;
const BUFFER_SIZE: usize = 256;
const MAX_VALUE: u8 = 255;

pub struct Interpreter<'a> {
    instructions: Vec<Instruction>,
    pc: usize,
    ap: isize,     // logical address pointer, will have negatives
    max_ap: isize, // actual max address pointer reached, can compare
    memory: [u8; MEM_SIZE],
    input: &'a mut Read,
    output: &'a mut Write,
    output_buffer: Vec<u8>,
    config: InterpreterConfig,
}

impl<'a> fmt::Debug for Interpreter<'a> {
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

impl<'a> Interpreter<'a> {
    pub fn new(
        ins: Vec<Instruction>,
        input: &'a mut Read,
        output: &'a mut Write,
        config: InterpreterConfig,
    ) -> Interpreter<'a> {
        Interpreter {
            pc: 0,
            ap: 0,
            max_ap: 0,
            memory: [0; MEM_SIZE],
            instructions: ins,
            input: input,
            output: output,
            output_buffer: Vec::with_capacity(BUFFER_SIZE),
            config: config,
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
            self.output.write(&self.output_buffer[0..]).unwrap();
            self.output.flush().unwrap();
            self.output_buffer.clear();
        }
        ()
    }

    fn read_one(&mut self) -> Option<u8> {
        let mut buf = [0u8; 1];
        let read_byte = match self.input.read(&mut buf) {
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => match self.config.eof {
                EOFBehaviour::Zero => Ok(Some(0)),
                EOFBehaviour::MaxValue => Ok(Some(MAX_VALUE)),
                EOFBehaviour::Unchanged => Ok(None),
            },

            // If the return value of this method is Ok(n), then it must be guaranteed that 0 <= n
            // <= buf.len(). A nonzero n value indicates that the buffer buf has been filled in
            // with n bytes of data from this source. If n is 0, then it can indicate one of two
            // scenarios:
            //
            // 1) This reader has reached its "end of file" and will likely no longer be
            // able to produce bytes. Note that this does not mean that the reader will always no
            // longer be able to produce bytes.
            //
            // 2) The buffer specified was 0 bytes in length.
            // https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
            // (our buffer is definitely > 0)
            Ok(0) => match self.config.eof {
                EOFBehaviour::Zero => Ok(Some(0)),
                EOFBehaviour::MaxValue => Ok(Some(MAX_VALUE)),
                EOFBehaviour::Unchanged => Ok(None),
            },
            Ok(_) => Ok(Some(buf[0])),
            Err(e) => Err(e),
        };
        read_byte.unwrap()
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        while self.pc < self.instructions.len() {
            let current_symbol = self.instructions[self.pc];
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
                    if let Some(a) = read {
                        self.memory[address] = a
                    };
                    self.pc += 1;
                }
                Instruction::JumpIfZero => {
                    let address = self.concrete_address()?;
                    let byte = self.memory[address];
                    self.pc = if byte == 0 {
                        let mut shadow_pc = self.pc;
                        let mut stack_counter = 1;
                        while shadow_pc < self.instructions.len() && stack_counter > 0 {
                            shadow_pc += 1;
                            let shadow_symbol = self.instructions[shadow_pc];
                            let stack_modifier = match shadow_symbol {
                                Instruction::JumpIfZero => 1,
                                Instruction::JumpUnlessZero => -1,
                                _ => 0,
                            };
                            stack_counter += stack_modifier;
                        }

                        if stack_counter == 0 {
                            shadow_pc
                        } else if stack_counter > 0 {
                            panic!("did not find matching ]")
                        } else {
                            panic!("ICE: right lookahead resulted in negative stack")
                        }
                    } else {
                        self.pc + 1
                    }
                }
                Instruction::JumpUnlessZero => {
                    let address = self.concrete_address()?;
                    let byte = self.memory[address];
                    self.pc = if byte == 0 {
                        self.pc + 1
                    } else {
                        let mut shadow_pc = self.pc;
                        let mut stack_counter = 1;
                        while stack_counter > 0 {
                            shadow_pc -= 1;
                            let shadow_symbol = self.instructions[shadow_pc];
                            let stack_modifier = match shadow_symbol {
                                Instruction::JumpIfZero => -1,
                                Instruction::JumpUnlessZero => 1,
                                _ => 0,
                            };
                            stack_counter += stack_modifier;
                            if shadow_pc == 0 {
                                break;
                            }
                        }

                        if stack_counter == 0 {
                            shadow_pc
                        } else if stack_counter > 0 {
                            panic!("did not find matching [")
                        } else {
                            panic!("ICE: left lookbehind resulted in negative stack")
                        }
                    }
                }
            }
        }
        self.write_out_buffer();
        Ok(())
    }
}
