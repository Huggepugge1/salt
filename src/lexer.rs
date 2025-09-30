use logos::Logos;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Kernel,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Void,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[token("kernel", |_| Keyword::Kernel)]
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
