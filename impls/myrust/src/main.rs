mod input;
use std::io::{self, Error};

use input::InputReader;

fn main() {
    let mut input = InputReader::new();
    while let Ok(c) = input.read_char() {
        print!("{}", c);
    }
}
