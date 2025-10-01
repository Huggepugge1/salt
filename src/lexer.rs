use anyhow::Result;

use logos::{Logos, Span};

use crate::error::LexingError;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Fn,
    Raw,

    Loop,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Void,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error(LexingError, LexingError::from_lexer))]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum TokenKind {
    #[regex(r"(raw|fn|loop)", |lex| match lex.slice() {
        "fn" => Keyword::Fn,
        "raw" => Keyword::Raw,

        "loop" => Keyword::Loop,
        _ => unreachable!(),
    }, priority = 3)]
    Keyword(Keyword),

    #[token("void", |lex| match lex.slice() {
        "void" => Type::Void,
        _ => unreachable!(),
    })]
    Type(Type),

    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Intrinsic(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[token("->")]
    ReturnType,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(";")]
    Semicolon,
}

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut lex = TokenKind::lexer(source);
    while let Some(kind) = lex.next() {
        match kind {
            Ok(kind) => tokens.push(Token {
                kind,
                span: lex.span(),
            }),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(tokens)
}
