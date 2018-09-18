extern crate libheadugh;

use std::io::Read;

struct StubRead;

impl Read for StubRead {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        panic!("Program read but was not meant to!");
    }
}

#[test]
fn test_exclamation_mark() {
    let input = "+++++++++++++++++++++++++++++++++.";
    let mut output = Vec::new();
    let mut input_handle = StubRead {};
    libheadugh::execute(input, &mut input_handle, &mut output).unwrap();
    assert_eq!(output[0], 33);
}
