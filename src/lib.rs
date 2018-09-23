mod instruction;
mod interpreter;
pub mod interpreter_config;
mod parser;
extern crate unicode_segmentation;

use interpreter::Interpreter;
use interpreter_config::InterpreterConfig;
use std::fs::File;
use std::io::prelude::*;

pub fn run(filename: &str, config: InterpreterConfig) -> std::io::Result<()> {
    let mut file = File::open(&filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let contents = contents;
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    execute(&contents, &mut stdin, &mut stdout, config)
}

pub fn execute(
    program: &str,
    input: &mut Read,
    output: &mut Write,
    config: InterpreterConfig,
) -> std::io::Result<()> {
    let program = parser::parse(&program).unwrap();
    let mut interpreter = Interpreter::new(program, input, output, config);
    interpreter.execute().unwrap();
    Ok(())
}

pub fn execute_with_default_config(
    program: &str,
    input: &mut Read,
    output: &mut Write,
) -> std::io::Result<()> {
    execute(program, input, output, InterpreterConfig::default())
}
