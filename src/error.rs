use thiserror::Error;

use crate::lexer::{Token, TokenKind};

#[derive(Default, Debug, Clone, PartialEq, Error)]
pub enum LexingError {
    #[error("Invalid character `{0}`")]
    UnexpectedCharacter(char),

    #[default]
    #[error("")]
    Other,
}

impl LexingError {
    pub fn from_lexer(lex: &mut logos::Lexer<'_, TokenKind>) -> Self {
        LexingError::UnexpectedCharacter(lex.slice().chars().next().unwrap())
    }
}
#[derive(Error, Debug)]
#[error("{kind}: {token:?}")]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub token: Option<Token>,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, token: Option<Token>) -> ParseError {
        Self { kind, token }
    }
}

#[derive(Debug)]
pub enum ExpectedToken {
    Statement,
    Specific(TokenKind),
}

#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("Unexpected end of file")]
    UnexpectedEOF,

    #[error("Unexpected token {0:?}")]
    UnexpectedToken(ExpectedToken),
}
