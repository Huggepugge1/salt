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
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub token: Option<Token>,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.kind)?;
        if let Some(token) = &self.token {
            writeln!(
                f,
                "{} | {}",
                token.location.line,
                token.location.source[1].clone().unwrap()
            )?;
            writeln!(
                f,
                "{} | {}{}",
                " ".repeat(token.location.line.ilog10() as usize + 1),
                " ".repeat(token.location.col - 1),
                "^".repeat(token.location.length),
            )?;
        }
        Ok(())
    }
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

    #[error("Unexpected token, expected `{expected:?}` found `{actual:?}`")]
    UnexpectedToken {
        actual: TokenKind,
        expected: ExpectedToken,
    },
}
