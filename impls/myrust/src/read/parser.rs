use super::{LexToken, LexerIterator, LexingError};
use super::{MalAtom, MalType};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<LexerIterator<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ParsingError {
    LexingError(LexingError),
    UnexpectedToken(LexToken),
    NumberParsingError(std::num::ParseIntError),
    UnexpectedEOF,
    Eof,
}
impl From<LexingError> for ParsingError {
    fn from(value: LexingError) -> Self {
        Self::LexingError(value)
    }
}

impl<'a> Parser<'a> {
    pub fn new(lexer: LexerIterator<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    pub fn read_form(&mut self, eof_allowed: bool) -> Result<MalType, ParsingError> {
        use super::LexToken::*;

        match self.peek_token() {
            Ok(tok) => match tok {
                RoundParenOpen => {
                    self.get_token()?;
                    Ok(MalType::List(
                        self.read_form_list(LexToken::RoundParenClose)?,
                    ))
                }
                SquareParenOpen => todo!(),
                CurlyParenOpen => todo!(),
                Tick => todo!(),
                BackTick => todo!(),
                TildeAt => todo!(),
                Tilde => todo!(),
                At => todo!(),
                Hat => todo!(),
                QuotedString(_) => self.read_atom(),
                Comment(_) => {
                    self.get_token()?;
                    self.read_form(eof_allowed) // read next form (RECURSIVE! hoping for tail call optimization here)
                }
                Name(_) => self.read_atom(),
                unexpected => {
                    self.lexer.next();
                    Err(ParsingError::UnexpectedToken(unexpected))
                }
            },
            Err(ParsingError::UnexpectedEOF) if eof_allowed => Err(ParsingError::Eof),
            Err(err) => Err(err),
        }
    }
    fn peek_token(&mut self) -> Result<LexToken, ParsingError> {
        let res = self.lexer.peek().ok_or(ParsingError::UnexpectedEOF)?;
        match res {
            Ok(res) => Ok(res.clone()),
            Err(err) => Err(err.clone().into()),
        }
    }
    fn get_token(&mut self) -> Result<LexToken, ParsingError> {
        let res = self.lexer.next().ok_or(ParsingError::UnexpectedEOF)?;
        res.map_err(|err| err.into())
    }

    fn read_form_list(&mut self, until: LexToken) -> Result<Vec<MalType>, ParsingError> {
        let mut result = vec![];
        while self.peek_token()? != until {
            result.push(self.read_form(false)?)
        }

        self.get_token()?; // get the "until" token

        Ok(result)
    }
    fn read_atom(&mut self) -> Result<MalType, ParsingError> {
        match self.get_token()? {
            LexToken::QuotedString(str) => Ok(MalType::Atom(MalAtom::String(str))),
            LexToken::Name(name) => {
                if name.chars().all(|ch| ch.is_ascii_digit()) {
                    Ok(MalType::Atom(MalAtom::IntNumber(name.parse().map_err(
                        |parse_err| ParsingError::NumberParsingError(parse_err),
                    )?)))
                } else {
                    Ok(MalType::Atom(MalAtom::Name(name)))
                }
            }
            unexpected => Err(ParsingError::UnexpectedToken(unexpected)),
        }
    }
}
