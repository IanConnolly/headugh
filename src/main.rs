extern crate libheadugh;

#[macro_use]
extern crate clap;

use libheadugh::interpreter_config::EOFBehaviour;
use libheadugh::interpreter_config::InterpreterConfig;
use libheadugh::run;

fn main() -> std::io::Result<()> {
    let matches = clap_app!(headugh =>
        (version: "0.1.0")
        (author: "Ian Connolly <ian@connolly.io>")
        (about: "\nyet another brainfuck interpreter")
        (@arg EOF: --eof +takes_value "Sets eof behaviour to `zero`, `max-value` or `unchanged`")
        (@arg INPUT: +required "sets the brainfuck file to execute")
    ).get_matches();
    let eof = match matches.value_of("EOF").map(|s| s.to_lowercase()) {
        Some(ref s) if s == "zero" => EOFBehaviour::Zero,
        Some(ref s) if s == "max-value" => EOFBehaviour::MaxValue,
        Some(ref s) if s == "unchanged" => EOFBehaviour::Unchanged,
        Some(_) => panic!("TODO(ian)"),
        None => EOFBehaviour::Zero,
    };
    let config = InterpreterConfig::new(eof);
    run(matches.value_of("INPUT").unwrap(), config)
}
