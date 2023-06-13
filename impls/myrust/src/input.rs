use std::{
    collections::VecDeque,
    io::{self, Error, ErrorKind},
};

use rustyline::{
    error::ReadlineError,
    history::{FileHistory, MemHistory},
    DefaultEditor, Editor,
};

pub struct InputReader {
    buffer: VecDeque<char>,
    end: bool,
    rustyline: Editor<(), FileHistory>,
}

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

    pub fn read_char(&mut self) -> Result<char, InputError> {
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

    pub fn new() -> InputReader {
        return InputReader {
            buffer: VecDeque::new(),
            end: false,
            rustyline: DefaultEditor::new().unwrap(),
        };
    }
}
