use rustyline::{error::ReadlineError, history::FileHistory, DefaultEditor, Editor};

use super::InputError;

pub trait InputSource {
    fn read(&mut self) -> Result<String, InputError>;
}

pub struct REPLTerminalInputSource {
    rustyline: Editor<(), FileHistory>,
}

impl InputSource for REPLTerminalInputSource {
    fn read(&mut self) -> Result<String, InputError> {
        let read_result = self.rustyline.readline("user> ");

        match read_result {
            Ok(str) => Ok(str + "\n"),
            Err(ReadlineError::Eof) => Err(InputError::ExitIndication),
            Err(ReadlineError::Interrupted) => Err(InputError::ExitIndication),
            Err(ReadlineError::WindowResized) => Err(InputError::RetriableError),
            Err(_) => Err(InputError::NonRetriableError),
        }
    }
}

impl REPLTerminalInputSource {
    pub fn new() -> Self {
        Self {
            rustyline: DefaultEditor::new().unwrap(),
        }
    }
}

pub struct StringInputSource {
    content: Option<String>,
}

impl InputSource for StringInputSource {
    fn read(&mut self) -> Result<String, InputError> {
        let content = self.content.take();
        match content {
            Some(c) => Ok(c),
            None => Err(InputError::ExitIndication),
        }
    }
}

impl StringInputSource {
    pub fn new(content: String) -> Self {
        Self {
            content: Some(content),
        }
    }
}
