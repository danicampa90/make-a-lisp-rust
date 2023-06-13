use std::collections::VecDeque;

use rustyline::{error::ReadlineError, history::FileHistory, DefaultEditor, Editor};

pub struct InputReader {
    // fields used by refill_buffer and read_char
    buffer: VecDeque<char>,
    end: bool,
    rustyline: Editor<(), FileHistory>,

    // get_char / peek_char support
    peeked_char: Option<char>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum InputError {
    RetriableError,
    NonRetriableError,
    ExitIndication,
}

impl InputReader {
    fn refill_buffer(&mut self) -> Result<(), InputError> {
        let read_result = self.rustyline.readline("user> ");

        match read_result {
            Ok(str) => {
                self.buffer = str.chars().collect();
                self.buffer.push_back('\n');
                Ok(())
            }
            Err(ReadlineError::Eof) => Err(InputError::ExitIndication),
            Err(ReadlineError::Interrupted) => Err(InputError::ExitIndication),
            Err(ReadlineError::WindowResized) => Err(InputError::RetriableError),
            Err(_) => Err(InputError::NonRetriableError),
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

    pub fn new() -> InputReader {
        return InputReader {
            buffer: VecDeque::new(),
            end: false,
            rustyline: DefaultEditor::new().unwrap(),
            peeked_char: None,
        };
    }
}
