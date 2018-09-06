use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Write,
    Read,
    JumpIfZero,
    JumpUnlessZero,
}

use self::Instruction::*;

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let stringified = match self {
            MoveRight => ">",
            MoveLeft => "<",
            Increment => "+",
            Decrement => "-",
            Write => ".",
            Read => ",",
            JumpIfZero => "[",
            JumpUnlessZero => "]",
        };
        write!(f, "{}", stringified)
    }
}

pub fn instruction_of_str(input: &str) -> Option<Instruction> {
    match input {
        ">" => Some(MoveRight),
        "<" => Some(MoveLeft),
        "+" => Some(Increment),
        "-" => Some(Decrement),
        "." => Some(Write),
        "," => Some(Read),
        "[" => Some(JumpIfZero),
        "]" => Some(JumpUnlessZero),
        _ => None,
    }
}
