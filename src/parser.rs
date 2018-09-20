use unicode_segmentation::UnicodeSegmentation;

use instruction;
use instruction::Instruction;

pub fn parse(input: &str) -> Result<Vec<Instruction>, &'static str> {
    let graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    let ils = graphemes
        .iter()
        .filter_map(|g| instruction::instruction_of_str(g))
        .collect::<Vec<Instruction>>();
    Ok(ils)
}
