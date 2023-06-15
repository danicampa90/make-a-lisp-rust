use std::collections::VecDeque;

use super::InputSource;

pub struct InputReader {
    // fields used by refill_buffer and read_char
    buffer: VecDeque<char>,
    end: bool,

    // get_char / peek_char support
    peeked_char: Option<char>,

    input_source: Box<dyn InputSource>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum InputError {
    RetriableError,
    NonRetriableError,
    ExitIndication,
}

impl InputReader {
    fn refill_buffer(&mut self) -> Result<(), InputError> {
        match self.input_source.read() {
            Ok(str) => {
                self.buffer.append(&mut str.chars().collect());
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn read_char(&mut self) -> Result<char, InputError> {
        if self.end {
            return Err(InputError::ExitIndication);
        }

        while self.buffer.len() == 0 {
            match self.refill_buffer() {
                Ok(()) => {}
                Err(InputError::ExitIndication) => {
                    self.end = true;
                    return Err(InputError::ExitIndication);
                }
                Err(InputError::NonRetriableError) => return Err(InputError::NonRetriableError),
                Err(InputError::RetriableError) => return Err(InputError::RetriableError),
            }
        }

        let result = self.buffer.pop_front().unwrap();
        return Ok(result);
    }

    pub fn peek_char(&mut self) -> Result<char, InputError> {
        if self.peeked_char.is_none() {
            self.peeked_char = Some(self.read_char()?);
        }

        Ok(self.peeked_char.unwrap())
    }
    pub fn get_char(&mut self) -> Result<char, InputError> {
        if let Some(c) = self.peeked_char {
            self.peeked_char = None;
            Ok(c)
        } else {
            self.read_char()
        }
    }

    pub fn new(input_source: Box<dyn InputSource>) -> InputReader {
        return InputReader {
            buffer: VecDeque::new(),
            end: false,
            peeked_char: None,
            input_source,
        };
    }
}
