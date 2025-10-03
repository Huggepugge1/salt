use thiserror::Error;

use crate::lexer::{Location, Token, TokenKind, Type};

#[derive(Default, Debug, Clone, PartialEq, Error)]
pub enum LexingError {
    #[error("error: invalid character `{0}`")]
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

#[derive(Debug)]
pub enum ExpectedToken {
    Statement,
    ImportSymbol,
    Specific { kind: TokenKind },
}

impl std::fmt::Display for ExpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectedToken::Statement => write!(f, "statement"),
            ExpectedToken::ImportSymbol => write!(f, "identifier or `::`"),
            ExpectedToken::Specific { kind } => write!(f, "{kind}"),
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    UnexpectedEOF,

    UnexpectedToken {
        actual: Box<Token>,
        expected: ExpectedToken,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEOF => write!(f, "UnexpectedEOF"),
            ParseError::UnexpectedToken { actual, expected } => {
                writeln!(f, "error: expected {} found {}", expected, actual.kind)?;
                writeln!(f)?;
                writeln!(f, "{}", actual.location)?;
                Ok(())
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TypeCheckError {
    UnsafeUse(Location),
    UnsafeUseNoToken,

    UndeclaredFunction(Location),
    UndeclaredFunctionNoToken,

    MismatchedType {
        expected: Type,
        actual: Type,
        location: Location,
    },
}

impl std::fmt::Display for TypeCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeCheckError::UnsafeUse(location) => {
                writeln!(f, "error: unsafe")?;
                writeln!(f)?;
                writeln!(f, "{}", location)?;
                Ok(())
            }
            TypeCheckError::UnsafeUseNoToken => unreachable!(),

            TypeCheckError::UndeclaredFunction(location) => {
                writeln!(f, "error: cannot find function `{}`", location.value())?;
                writeln!(f)?;
                writeln!(f, "{}", location)?;
                Ok(())
            }
            TypeCheckError::UndeclaredFunctionNoToken => unreachable!(),

            TypeCheckError::MismatchedType {
                expected,
                actual: found,
                location,
            } => {
                writeln!(
                    f,
                    "error: mismatched types, expected {}, found {}",
                    expected, found
                )?;
                writeln!(f)?;
                writeln!(f, "{}", location)?;
                Ok(())
            }
        }
    }
}
