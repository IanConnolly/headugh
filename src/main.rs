extern crate libheadugh;

#[macro_use]
extern crate clap;

use libheadugh::run;

fn main() -> std::io::Result<()> {
    let matches = clap_app!(headugh =>
        (version: "0.1.0")
        (author: "Ian Connolly <ian@connolly.io>")
        (about: "\nyet another brainfuck interpreter")
        (@arg INPUT: +required "sets the brainfuck file to execute")
    ).get_matches();
    run(matches.value_of("INPUT").unwrap())
}
