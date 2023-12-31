use super::AstNode;
use super::{LexToken, LexerIterator, LexingError};
use std::collections::HashMap;
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<LexerIterator<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ParsingError {
    LexingError(LexingError),
    UnexpectedToken(LexToken),
    UnexpectedEOF,
    EOF,
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

    pub fn read_form(&mut self, eof_allowed: bool) -> Result<AstNode, ParsingError> {
        use super::LexToken::*;

        match self.peek_token() {
            Ok(tok) => match tok {
                RoundParenOpen => {
                    self.get_token()?;
                    Ok(AstNode::List(
                        self.read_form_list(LexToken::RoundParenClose)?,
                    ))
                }
                SquareParenOpen => {
                    self.get_token()?;
                    Ok(AstNode::Vector(
                        self.read_form_list(LexToken::SquareParenClose)?,
                    ))
                }
                CurlyParenOpen => {
                    self.get_token()?;
                    Ok(self.read_hashmap()?)
                }
                Tick => {
                    self.get_token()?;
                    Ok(AstNode::List(vec![
                        AstNode::UnresolvedSymbol("quote".to_string()),
                        self.read_form(false)?,
                    ]))
                }
                BackTick => {
                    self.get_token()?;
                    Ok(AstNode::List(vec![
                        AstNode::UnresolvedSymbol("quasiquote".to_string()),
                        self.read_form(false)?,
                    ]))
                }
                Tilde => {
                    self.get_token()?;
                    Ok(AstNode::List(vec![
                        AstNode::UnresolvedSymbol("unquote".to_string()),
                        self.read_form(false)?,
                    ]))
                }
                TildeAt => {
                    self.get_token()?;
                    Ok(AstNode::List(vec![
                        AstNode::UnresolvedSymbol("splice-unquote".to_string()),
                        self.read_form(false)?,
                    ]))
                }
                At => {
                    self.get_token()?;
                    Ok(AstNode::List(vec![
                        AstNode::UnresolvedSymbol("deref".to_string()),
                        self.read_form(false)?,
                    ]))
                }
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
            Err(ParsingError::UnexpectedEOF) if eof_allowed => Err(ParsingError::EOF),
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

    fn read_form_list(&mut self, until: LexToken) -> Result<Vec<AstNode>, ParsingError> {
        let mut result = vec![];
        while self.peek_token()? != until {
            result.push(self.read_form(false)?)
        }

        self.get_token()?; // get the "until" token

        Ok(result)
    }
    fn read_atom(&mut self) -> Result<AstNode, ParsingError> {
        match self.get_token()? {
            LexToken::QuotedString(str) => Ok(AstNode::String(str)),
            LexToken::Name(name) => {
                if let Ok(num) = name.parse() {
                    Ok(AstNode::Int(num))
                } else {
                    if name == "nil" {
                        Ok(AstNode::Nil)
                    } else if name == "true" {
                        Ok(AstNode::Bool(true))
                    } else if name == "false" {
                        Ok(AstNode::Bool(false))
                    } else {
                        Ok(AstNode::UnresolvedSymbol(name))
                    }
                }
            }
            unexpected => Err(ParsingError::UnexpectedToken(unexpected)),
        }
    }

    fn read_hashmap(&mut self) -> Result<AstNode, ParsingError> {
        let mut result = HashMap::new();
        while self.peek_token()? != LexToken::CurlyParenClose {
            let key_token = self.get_token()?;

            let key = match key_token {
                LexToken::QuotedString(key) => key,
                tok => return Err(ParsingError::UnexpectedToken(tok)),
            };

            result.insert(key, self.read_form(false)?);
        }

        self.get_token()?; // get the '}' token

        return Ok(AstNode::HashMap(result));
    }
}
