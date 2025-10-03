use anyhow::Result;

use logos::{Logos, Span};

use crate::error::LexingError;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Use,

    Fn,
    Raw,

    Loop,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Use => write!(f, "use"),
            Keyword::Fn => write!(f, "fn"),
            Keyword::Raw => write!(f, "raw"),
            Keyword::Loop => write!(f, "loop"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Void,

    Str,

    Any,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "`void`"),
            Type::Str => write!(f, "`str`"),
            Type::Any => write!(f, "`T`"),
        }
    }
}

impl Type {
    pub fn to_ir(&self) -> &'static str {
        match self {
            Type::Void => "void",
            Type::Str => "i8*",
            Type::Any => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Location {
    pub line: usize,
    pub col: usize,
    pub length: usize,
    pub source: [Option<String>; 3],
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} | {}", self.line, self.source[1].clone().unwrap())?;
        writeln!(
            f,
            "{} | {}{}",
            " ".repeat(self.line.ilog10() as usize + 1),
            " ".repeat(self.col - 1),
            "^".repeat(self.length),
        )?;
        Ok(())
    }
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

    pub fn value(&self) -> String {
        let start = self.col - 1;
        let end = self.col - 1 + self.length;
        self.source[1].as_ref().unwrap()[start..end].to_string()
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
    #[regex(r"(use|raw|fn|loop)", |lex| match lex.slice() {
        "use" => Keyword::Use,
        "fn" => Keyword::Fn,
        "raw" => Keyword::Raw,

        "loop" => Keyword::Loop,
        _ => unreachable!(),
    }, priority = 3)]
    Keyword(Keyword),

    #[regex(r"(void|str)", |lex| match lex.slice() {
        "void" => Type::Void,
        "str" => Type::Str,
        _ => unreachable!(),
    })]
    Type(Type),

    #[regex(r#""(?:[^"]|\\")*""#, |lex| {
        let string = lex.slice().to_string();
        let len = string.len();
        string[1..len - 1].to_string()
    })]
    StringLiteral(String),

    #[regex(r"@[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Intrinsic(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("::")]
    PathSeparator,

    #[token("->")]
    Arrow,

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

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Keyword(keyword) => write!(f, "keyword `{keyword}`"),
            TokenKind::Type(t) => write!(f, "{t}"),

            TokenKind::StringLiteral(string) => write!(f, "{string:?}"),

            TokenKind::Intrinsic(intrinsic) => write!(f, "intrinsic `{intrinsic}`"),
            TokenKind::Identifier(identifier) => {
                if identifier.is_empty() {
                    write!(f, "Identifier")
                } else {
                    write!(f, "`{identifier}`")
                }
            }
            TokenKind::PathSeparator => write!(f, "`::`"),
            TokenKind::Arrow => write!(f, "`->`"),
            TokenKind::OpenParen => write!(f, "`(`"),
            TokenKind::CloseParen => write!(f, "`)`"),
            TokenKind::OpenBrace => write!(f, "`{{`"),
            TokenKind::CloseBrace => write!(f, "`}}`"),
            TokenKind::Semicolon => write!(f, "`;`"),
        }
    }
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
