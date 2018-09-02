extern crate libheadugh;

use libheadugh::run;

fn main() -> std::io::Result<()> {
    let filename = std::env::args().nth(1).unwrap();
    run(&filename)
}
