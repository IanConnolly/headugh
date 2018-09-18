extern crate libheadugh;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str;

struct StubRead;

impl Read for StubRead {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        panic!("Program read but was not meant to!");
    }
}

fn load_program(name: &str) -> String {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("tests/programs/{}.bf", name));
    let mut file = File::open(&d).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.to_owned()
}

#[test]
fn test_exclamation_mark() {
    let program = load_program("exclamation");
    let mut output = Vec::new();
    let mut input = StubRead {};
    libheadugh::execute(&program, &mut input, &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "!");
}

#[test]
fn test_wikipedia_hello_world() {
    let program = load_program("wiki");
    let mut output = Vec::new();
    let mut input_handle = StubRead {};
    libheadugh::execute(&program, &mut input_handle, &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "Hello World!\n");
}
