mod instruction;
mod interpreter;
mod parser;
extern crate unicode_segmentation;

use interpreter::Interpreter;
use std::fs::File;
use std::io::prelude::*;

pub fn run(filename: &str) -> std::io::Result<()> {
    let mut file = File::open(&filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let contents = contents;
    let program = parser::parse(&contents).unwrap();
    let mut interpreter = Interpreter::new(
        program,
        Box::new(std::io::stdin()),
        Box::new(std::io::stdout()),
    );
    interpreter.execute();
    println!("{:?}", interpreter);
    Ok(())
}
