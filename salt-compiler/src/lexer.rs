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

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub length: usize,
    pub source: [Option<String>; 3],
}

impl Location {
    pub fn new(source: &str, span: Span) -> Self {
        let split_source = source
            .split("\n")
            .map(|e| (e.len() + 1, e))
            .collect::<Vec<_>>();
        let mut current_len = 0;
        let mut split_source_with_len = Vec::new();
        for (len, e) in split_source {
            split_source_with_len.push((current_len, e));
            current_len += len;
        }
        let length = span.len();
        let mut source = [None, None, None];

        let mut i = 0;
        let mut line_start = 0;
        while split_source_with_len[i].0 <= span.start {
            line_start = split_source_with_len[i].0;
            let (_, source_line) = &split_source_with_len[i];
            source[0] = source[1].clone();
            source[1] = source[2].clone();
            source[2] = Some(source_line.to_string());
            i += 1;
        }

        source[0] = source[1].clone();
        source[1] = source[2].clone();
        source[2] = Some(split_source_with_len[i].1.to_string());

        let line = i;
        let col = span.start - line_start + 1;
        Self {
            line,
            col,
            length,
            source,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
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
                location: Location::new(source, lex.span()),
            }),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(tokens)
}
