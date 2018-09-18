use unicode_segmentation::UnicodeSegmentation;

use instruction;
use instruction::Instruction;
use std::fmt;

use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct InstructionList(Vec<Instruction>);

impl Deref for InstructionList {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Vec<Instruction> {
        &self.0
    }
}

impl fmt::Display for InstructionList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mapped: Vec<String> = self.0.iter().map(|x| x.to_string()).collect();
        let stringified = mapped.join("");
        write!(f, "{}", stringified)
    }
}

pub fn parse(input: &str) -> Result<InstructionList, &'static str> {
    let graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let ils = graphemes
        .iter()
        .filter_map(|g| instruction::instruction_of_str(g))
        .collect::<Vec<Instruction>>();
    Ok(InstructionList(ils))
}
