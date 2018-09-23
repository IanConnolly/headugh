use std::default::Default;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EOFBehaviour {
    Zero,
    MaxValue,
    Unchanged,
}

#[derive(Debug, Copy, Clone)]
pub struct InterpreterConfig {
    pub eof: EOFBehaviour,
}

impl Default for InterpreterConfig {
    fn default() -> InterpreterConfig {
        InterpreterConfig {
            eof: EOFBehaviour::Zero,
        }
    }
}

impl InterpreterConfig {
    pub fn new(eof: EOFBehaviour) -> InterpreterConfig {
        InterpreterConfig { eof: eof }
    }
}
