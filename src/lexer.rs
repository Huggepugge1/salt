use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Loop,

    Kernel,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Void,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(r"(kernel|loop)", |lex| match lex.slice() {
        "kernel" => Keyword::Kernel,
        "loop" => Keyword::Loop,
        _ => unreachable!(),
    }, priority = 3)]
    Keyword(Keyword),

    #[token("void", |_| Type::Void)]
    Type(Type),

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
}
