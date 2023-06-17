use super::{InputError, InputReader};

#[derive(PartialEq, Debug, Clone)]
pub enum LexToken {
    RoundParenOpen,
    RoundParenClose,
    SquareParenOpen,
    SquareParenClose,
    CurlyParenOpen,
    CurlyParenClose,
    Tick,                 // '
    BackTick,             // `
    TildeAt,              // ~@
    Tilde,                // ~
    At,                   // @
    Hat,                  // ^
    QuotedString(String), // "hello"
    Comment(String),      // ; this is a comment
    Name(String),         // true 10 nil anothername
}

#[derive(PartialEq, Debug, Clone)]
pub enum LexingError {
    InputError(InputError),
}

pub struct Lexer {}

impl Lexer {
    fn read_string_token(reader: &mut InputReader) -> Result<LexToken, LexingError> {
        let mut str = String::new();
        loop {
            match reader.get_char() {
                Ok(ch) if ch == '"' => return Ok(LexToken::QuotedString(str)),
                Ok(ch) if ch == '\\' => match reader.get_char() {
                    Ok(ch) if ch == 'n' => str.push('\n'),
                    Ok(ch) if ch == 'r' => str.push('\r'),
                    Ok(ch) if ch == '\\' => str.push('\\'),
                    Ok(ch) => str.push(ch),
                    Err(InputError::RetriableError) => {
                        return Err(LexingError::InputError(InputError::NonRetriableError))
                    }
                    Err(err) => return Err(err.into()),
                },
                Ok(ch) => str.push(ch),
                Err(InputError::RetriableError) => continue,
                Err(err) => return Err(err.into()),
            }
        }
    }
    fn read_name(starting_char: char, reader: &mut InputReader) -> Result<LexToken, LexingError> {
        let mut str = String::new();
        str.push(starting_char);

        let specials = vec!['(', ')', '[', ']', '{', '}', ','];

        loop {
            match reader.peek_char() {
                Ok(ch) if ch.is_whitespace() || specials.contains(&ch) => {
                    return Ok(LexToken::Name(str))
                }
                Ok(_) => str.push(reader.get_char()?),
                Err(InputError::RetriableError) => continue,
                Err(InputError::ExitIndication) if str.len() > 0 => return Ok(LexToken::Name(str)),
                Err(err) => return Err(err.into()),
            }
        }
    }
    fn read_comment(reader: &mut InputReader) -> Result<LexToken, LexingError> {
        let mut str = String::new();

        loop {
            match reader.peek_char() {
                Ok(ch) if ch != '\n' => str.push(reader.get_char()?),
                Ok(_) => return Ok(LexToken::Comment(str)),
                Err(InputError::RetriableError) => continue,
                Err(err) => return Err(err.into()),
            }
        }
    }
    pub fn read_next_token(reader: &mut InputReader) -> Result<LexToken, LexingError> {
        loop {
            let char = reader.get_char()?;
            let res = match char {
                // eat up all whitespace inbetween terms
                _ if char.is_whitespace() || char == ',' => continue,

                '(' => LexToken::RoundParenOpen,
                ')' => LexToken::RoundParenClose,
                '[' => LexToken::SquareParenOpen,
                ']' => LexToken::SquareParenClose,
                '{' => LexToken::CurlyParenOpen,
                '}' => LexToken::CurlyParenClose,
                '\'' => LexToken::Tick,
                '`' => LexToken::BackTick,
                '~' => {
                    if let Ok('@') = reader.peek_char() {
                        reader.get_char()?;
                        LexToken::TildeAt
                    } else {
                        LexToken::Tilde
                    }
                }
                '@' => LexToken::At,
                '^' => LexToken::Hat,
                '"' => Self::read_string_token(reader)?,
                ';' => Self::read_comment(reader)?,
                ch => Self::read_name(ch, reader)?,
            };
            return Ok(res);
        }
    }

    pub fn create_lexer_iterator<'a>(input: &'a mut InputReader) -> LexerIterator<'a> {
        LexerIterator::new(input)
    }
}

pub struct LexerIterator<'a> {
    input: &'a mut InputReader,
    eof_reached: bool,
}

impl<'a> LexerIterator<'a> {
    pub fn new(input: &'a mut InputReader) -> Self {
        Self {
            input,
            eof_reached: false,
        }
    }
}

impl<'a> Iterator for LexerIterator<'a> {
    type Item = Result<LexToken, LexingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof_reached {
            return None;
        }
        match Lexer::read_next_token(self.input) {
            Err(LexingError::InputError(InputError::ExitIndication)) => {
                self.eof_reached = true;
                None
            }
            result => Some(result),
        }
    }
}

impl From<InputError> for LexingError {
    fn from(value: InputError) -> Self {
        LexingError::InputError(value)
    }
}
