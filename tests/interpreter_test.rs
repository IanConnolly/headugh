extern crate libheadugh;

use libheadugh::interpreter_config::EOFBehaviour;
use libheadugh::interpreter_config::InterpreterConfig;

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

fn filename_for(name: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("tests/programs/{}.bf", name));
    d
}

fn filename_for_data(name: &str) -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("tests/data/{}", name));
    d
}

fn load_program(name: &str) -> String {
    let filename = filename_for(name);
    let mut file = File::open(&filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.to_owned()
}

#[test]
fn test_exclamation_mark() {
    let program = load_program("exclamation");
    let mut output = Vec::new();
    let mut input = StubRead {};
    libheadugh::execute_with_default_config(&program, &mut input, &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "!");
}

#[test]
fn test_wikipedia_hello_world() {
    let program = load_program("wiki");
    let mut output = Vec::new();
    let mut input_handle = StubRead {};
    libheadugh::execute_with_default_config(&program, &mut input_handle, &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "Hello World!\n");
}

#[test]
fn test_rot13_io() {
    let program = load_program("rot13");
    let input = "~mlk zyx".to_owned().into_bytes().into_boxed_slice();
    let mut output = Vec::new();
    libheadugh::execute_with_default_config(&program, &mut input.as_ref(), &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "~zyx mlk");
}

#[test]
fn test_obscure_issue_h() {
    let program = load_program("obscure_issue_h");
    let mut output = Vec::new();
    let mut input = StubRead {};
    libheadugh::execute_with_default_config(&program, &mut input, &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "H\n");
}

#[test]
fn test_head() {
    let program = load_program("head");
    let input = "hi\nmy\nname\nis\nian\nand\nthis\nis\na\nsuper\nodd\nmethod\nchain"
        .to_owned()
        .into_bytes()
        .into_boxed_slice();
    let mut output = Vec::new();
    libheadugh::execute_with_default_config(&program, &mut input.as_ref(), &mut output).unwrap();

    assert_eq!(
        str::from_utf8(&output).unwrap(),
        "hi\nmy\nname\nis\nian\nand\nthis\nis\na\nsuper\n"
    );
}

#[test]
fn test_qsort() {
    let program = load_program("qsort");
    let input = "9874563210".to_owned().into_bytes().into_boxed_slice();
    let mut output = Vec::new();
    libheadugh::execute_with_default_config(&program, &mut input.as_ref(), &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "0123456789");
}

#[test]
fn test_wc() {
    let program = load_program("wc");
    // word count itself!
    let mut input = File::open(filename_for("wc")).unwrap();
    let mut output = Vec::new();
    libheadugh::execute_with_default_config(&program, &mut input, &mut output).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "\t14\t23\t533\n");
}

#[test]
fn test_read_none() {
    let program = load_program("read_none");
    let mut output = Vec::new();
    let mut input = File::open(filename_for_data("empty")).unwrap();
    let config = InterpreterConfig::new(EOFBehaviour::Unchanged);
    libheadugh::execute(&program, &mut input, &mut output, config).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "!");
}

#[test]
fn test_read_zero() {
    let program = load_program("read_zero");
    let mut output = Vec::new();
    let mut input = File::open(filename_for_data("empty")).unwrap();
    let config = InterpreterConfig::new(EOFBehaviour::Zero);
    libheadugh::execute(&program, &mut input, &mut output, config).unwrap();

    assert_eq!(str::from_utf8(&output).unwrap(), "!");
}

#[test]
fn test_read_max() {
    let program = load_program("read_max");
    let mut output = Vec::new();
    let mut input = File::open(filename_for_data("empty")).unwrap();
    let config = InterpreterConfig::new(EOFBehaviour::MaxValue);
    libheadugh::execute(&program, &mut input, &mut output, config).unwrap();

    assert_eq!(output[0], 255);
}
